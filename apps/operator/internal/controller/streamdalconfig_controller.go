/*
Copyright 2024.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

package controller

import (
	"context"
	"fmt"
	"time"

	"github.com/streamdal/streamdal/libs/protos/build/go/protos"
	"google.golang.org/grpc/metadata"
	"google.golang.org/protobuf/encoding/protojson"
	v1 "k8s.io/api/core/v1"
	"k8s.io/apimachinery/pkg/api/errors"
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/client-go/tools/record"
	ctrl "sigs.k8s.io/controller-runtime"
	"sigs.k8s.io/controller-runtime/pkg/client"
	"sigs.k8s.io/controller-runtime/pkg/log"

	crdv1 "github.com/streamdal/streamdal/apps/operator/api/v1"
	"github.com/streamdal/streamdal/apps/operator/util"
	"github.com/streamdal/streamdal/apps/operator/validate"
)

const (
	FinalizerName = "streamdal.com/finalizer"
	CreatedBy     = "streamdal-operator"

	ReconcileActionUpdate ReconcileAction = "update" // "update" covers both "create" and "update"
	ReconcileActionDelete ReconcileAction = "delete"

	ResourceTypeAudience     ResourceType = "audience"
	ResourceTypeNotification ResourceType = "notification"
	ResourceTypeWasmModule   ResourceType = "wasm"
	ResourceTypePipeline     ResourceType = "pipeline"
)

// StreamdalConfigReconciler reconciles a StreamdalConfig object
type StreamdalConfigReconciler struct {
	client.Client
	Scheme   *runtime.Scheme
	Recorder record.EventRecorder

	// Used by periodic reconciler to detect shutdown
	ShutdownCtx context.Context
}

type ReconcileRequest struct {
	Action ReconcileAction
	Config *crdv1.StreamdalConfig
	Client protos.ExternalClient
}

type ReconcileAction string
type ResourceType string

type HandleFunc struct {
	Resource ResourceType
	Function func(ctx context.Context, rr *ReconcileRequest, wantedCfg, serverCfg *protos.Config) (*HandleStatus, error)
}

type HandleStatus struct {
	Resource   ResourceType
	Action     ReconcileAction
	NumCreated int
	NumUpdated int
	NumDeleted int
}

//+kubebuilder:rbac:groups=crd.streamdal.com,resources=streamdalconfigs,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=crd.streamdal.com,resources=streamdalconfigs/status,verbs=get;update;patch
//+kubebuilder:rbac:groups=crd.streamdal.com,resources=streamdalconfigs/finalizers,verbs=update

// Reconcile is part of the main kubernetes reconciliation loop which aims to
// move the current state of the cluster closer to the desired state.
// TODO(user): Modify the Reconcile function to compare the state specified by
// the StreamdalConfig object against the actual cluster state, and then
// perform operations to make the cluster state reflect the state specified by
// the user.
//
// For more details, check Reconcile and its Result here:
// - https://pkg.go.dev/sigs.k8s.io/controller-runtime@v0.17.2/pkg/reconcile
func (r *StreamdalConfigReconciler) Reconcile(ctx context.Context, req ctrl.Request) (ctrl.Result, error) {
	llog := log.FromContext(ctx).WithValues(
		"method", "Reconcile",
		"namespace", req.Namespace,
		"name", req.Name,
	)

	llog.Info("Received reconcile request")

	rr, err := r.setupReconcileAction(ctx, &req)
	if err != nil {
		if errors.IsNotFound(err) {
			llog.Info("Resource not found - nothing to do - object is probably deleted")
			return ctrl.Result{}, nil
		}

		llog.Error(err, "Failed to complete reconcile setup (won't retry)")

		// The only reason we want to do this extra check is if setupReconcileAction()
		// has a bug and doesn't return a config.
		if rr != nil && rr.Config != nil {
			if errors.IsInvalid(err) {
				r.Recorder.Event(rr.Config, v1.EventTypeWarning, "CR validation error (won't retry)", err.Error())
			} else {
				r.Recorder.Event(rr.Config, v1.EventTypeWarning, "Reconciliation setup error (won't retry)", err.Error())
			}

			// TODO: Can't get status to come through "k describe streamdalconfig <name>" - why?
			//rr.Config.Status.Fatal = true
			//rr.Config.Status.Status = "error"
			//rr.Config.Status.Message = "Failed to complete reconcile setup. Error: " + err.Error()
			//
			//if err := r.Update(ctx, rr.Config); err != nil {
			//	llog.Error(err, "unable to update StreamdalConfig status")
			//}
		}

		// NOTE: Have to return a nil error because k8s considers a non-nil
		// error as a transient error and will keep re-queueing the request.
		return ctrl.Result{}, nil
	}

	result, err := r.handleResources(ctx, rr)
	if err != nil {
		// TODO: llog.Error() should probably be llog.Info() in most cases because
		// k8s errors are _really_ loud.
		llog.Error(err, "Failed to complete reconcile action", "action", rr.Action)
		r.Recorder.Event(rr.Config, v1.EventTypeWarning, "Handle resources error", err.Error())

		// TODO: Should differentiate between a transient error and a non-transient
		// error so that we can requeue appropriately.
		return ctrl.Result{}, nil
	}

	var (
		action string
	)

	switch rr.Action {
	case ReconcileActionUpdate:
		err = r.addFinalizer(ctx, rr)
		action = "add"
	case ReconcileActionDelete:
		err = r.removeFinalizer(ctx, rr)
		action = "remove"
	}

	if err != nil {
		llog.Error(err, fmt.Sprintf("Failed to %s finalizer", action))
		r.Recorder.Event(rr.Config, v1.EventTypeWarning, fmt.Sprintf("Finalizer '%s' error", action), err.Error())

		return ctrl.Result{}, nil
	}

	llog.Info("Reconcile action completed", "action", rr.Action)

	return result, nil
}

// SetupWithManager sets up the controller with the Manager.
func (r *StreamdalConfigReconciler) SetupWithManager(mgr ctrl.Manager) error {
	// Launch periodic reconciler
	llog := log.Log.WithValues("method", "SetupWithManager")
	llog.Info("Starting periodic reconciler")

	r.Recorder = mgr.GetEventRecorderFor("streamdal-operator")

	go r.runPeriodicReconciler()

	return ctrl.NewControllerManagedBy(mgr).
		For(&crdv1.StreamdalConfig{}).
		Complete(r)
}

// handleResources is a helper that will iterate over all resources in the CR
// and execute the appropriate handler method for each resource type.
func (r *StreamdalConfigReconciler) handleResources(ctx context.Context, rr *ReconcileRequest) (ctrl.Result, error) {
	llog := log.Log.WithValues("method", "handleResources")
	llog.Info("Handling resource in streamdal server", "action", rr.Action)

	// Add auth metadata to ctx that's used when talking to streamdal grpc api
	ctx = createCtxWithAuth(ctx, rr.Config.Spec.ServerAuth)

	// Attempt to load server config
	serverConfig, err := rr.Client.GetConfig(ctx, &protos.GetConfigRequest{})
	if err != nil {
		return ctrl.Result{}, fmt.Errorf("failed to get server config: %s", err)
	}

	if err := validate.StreamdalProtosConfig(serverConfig.Config); err != nil {
		llog.Error(err, "Failed to validate server config")
		return ctrl.Result{}, err
	}

	for _, cfgItem := range rr.Config.Spec.Configs {
		// Attempt to load config in CR / wanted config
		wantedConfig := &protos.Config{}
		if err := protojson.Unmarshal([]byte(cfgItem.Config), wantedConfig); err != nil {
			return ctrl.Result{}, fmt.Errorf("failed to unmarshal wanted config '%s': %s", cfgItem.Name, err)
		}

		if err := validate.StreamdalProtosConfig(wantedConfig); err != nil {
			llog.Error(err, fmt.Sprintf("Failed to validate wanted config '%s'", cfgItem.Name))
			return ctrl.Result{}, err
		}

		// NOTE: Pipelines should be handled last as they will reference other resources

		handleFuncs := []HandleFunc{
			{
				Resource: ResourceTypeAudience,
				Function: r.handleAudiences,
			},
			{
				Resource: ResourceTypeNotification,
				Function: r.handleNotifications,
			},

			// Punting on support for wasm modules for now because of bytes ~DS 04.16.2024
			//{
			//	Resource:     "wasm module(s)",
			//	Function: r.handleWasmModules,
			//},

			{
				Resource: ResourceTypePipeline,
				Function: r.handlePipelines,
			},

			// Audience <-> pipeline mapping
			{
				Resource: "audience <-> pipeline mappings",
				Function: r.handleMappings,
			},
		}

		for _, f := range handleFuncs {
			llog.Info("Running handle function", "resource", f.Resource)
			status, err := f.Function(ctx, rr, wantedConfig, serverConfig.Config)
			if err != nil {
				return ctrl.Result{}, fmt.Errorf("failed to handle resource '%s': %s", f.Resource, err)
			}

			llog.Info("Handle function completed", "resource", f.Resource, "numCreated", status.NumCreated, "numUpdated", status.NumUpdated)
		}
	}

	return ctrl.Result{}, nil
}

// setupReconcileAction will validate the request, determine the type of action
// it is (create or delete) and create a grpc client that can be used for talking
// to the streamdal server. NOTE: Both "create" and "update" should be handled
// by the update action.
func (r *StreamdalConfigReconciler) setupReconcileAction(ctx context.Context, req *ctrl.Request) (*ReconcileRequest, error) {
	var cfg crdv1.StreamdalConfig

	if err := r.Get(ctx, req.NamespacedName, &cfg); err != nil {
		return nil, fmt.Errorf("unable to fetch StreamdalConfig object: %w", err)
	}

	reconcileAction := &ReconcileRequest{
		Config: &cfg,
	}

	if err := validate.StreamdalConfig(&cfg); err != nil {
		return reconcileAction, errors.NewInternalError(fmt.Errorf("invalid streamdal config: %s", err))
	}

	// This is a valid request for a valid resource, we know we will definitely
	// need to talk to the streamdal server.
	grpcClient, err := util.NewGrpcExternalClient(ctx, cfg.Spec.ServerAddress, cfg.Spec.ServerAuth)
	if err != nil {
		return reconcileAction, errors.NewInternalError(fmt.Errorf("unable to create grpc client: %s", err))
	}

	reconcileAction.Client = grpcClient

	if cfg.DeletionTimestamp != nil {
		reconcileAction.Action = ReconcileActionDelete
		return reconcileAction, nil
	}

	// Not a delete - either a create or an update
	reconcileAction.Action = ReconcileActionUpdate

	return reconcileAction, nil
}

// addFinalizer adds the streamdal-operator finalizer to the CR; won't add a new
// entry if finalizer already present.
func (r *StreamdalConfigReconciler) addFinalizer(ctx context.Context, rr *ReconcileRequest) error {
	llog := log.Log.WithValues("method", "addFinalizer")
	llog.Info("Adding finalizer")

	// Don't add finalizer if it already exists
	for _, f := range rr.Config.GetFinalizers() {
		if f == FinalizerName {
			llog.Info("Finalizer already exists, nothing to do")
			return nil
		}
	}

	rr.Config.Finalizers = append(rr.Config.Finalizers, FinalizerName)

	if err := r.Update(ctx, rr.Config); err != nil {
		return fmt.Errorf("failed to update CR with finalizer: %s", err)
	}

	return nil
}

// removeFinalizer removes the streamdal-operator finalizer and any dupes from
// the CR; returns nil if finalizer doesn't exist.
func (r *StreamdalConfigReconciler) removeFinalizer(ctx context.Context, rr *ReconcileRequest) error {
	llog := log.Log.WithValues("method", "removeFinalizer")
	llog.Info("Removing finalizer")

	// Don't remove finalizer if it doesn't exist
	finalizers := rr.Config.GetFinalizers()

	for i, f := range finalizers {
		// Go through all finalizers in case there are dupes
		if f == FinalizerName {
			rr.Config.Finalizers = append(finalizers[:i], finalizers[i+1:]...)
		}
	}

	if err := r.Update(ctx, rr.Config); err != nil {
		return fmt.Errorf("failed to update CR without finalizer: %s", err)
	}

	return nil
}

// runPeriodicReconciler is intended to be ran as a goroutine, started at operator
// startup. It will run every DefaultReconcileInterval or CR.periodicReconcilerInterval
// (if set).
//
// The periodic reconciler is designed to periodically check and ensure that
// the state of a system aligns with its desired state. It automatically
// corrects discrepancies by triggering necessary updates or fixes. This
// mechanism is essential for maintaining system integrity and reliability over
// time.
//
// A periodic reconciler is needed in addition to the regular reconcile loop to
// handle cases where external changes or system errors might not trigger events
// detected by the regular loop. For example: someone manually removed a config
// entry in the streamdal server via the UI - the periodic reconciler would detect
// the discrepancy and correct it by re-adding the config entry.
func (r *StreamdalConfigReconciler) runPeriodicReconciler() {
	// TODO: No need to catch shutdown (so far)
	llog := log.Log.WithValues("method", "runPeriodicReconciler")
	llog.Info("Starting")

	// TODO: How do we know what streamdal server to talk to?
	// IDEA: What if we look through all K8S objects and look for streamdal servers?
	// ^ Not a good idea - maybe we don't want to have some streamdal servers be
	// managed by the K8S operator.
	// IDEA 2: As resources are created, k8s operator adds the server to a map
	// of "known servers".

MAIN:
	for {
		select {
		case <-r.ShutdownCtx.Done():
			llog.Info("Caught shutdown signal - stopping")
			break MAIN
		default:
			llog.Info("Run")
			time.Sleep(3 * time.Second)
		}
	}

	llog.Info("Stopped")
}

// Helper for creating a ctx used for making auth'd grpc calls to the streamdal server
func createCtxWithAuth(ctx context.Context, authToken string) context.Context {
	return metadata.NewOutgoingContext(ctx, metadata.Pairs(util.AuthTokenMetadata, authToken))
}
