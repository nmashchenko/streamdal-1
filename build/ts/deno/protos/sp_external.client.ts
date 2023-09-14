// @generated by protobuf-ts 2.9.0 with parameter long_type_string
// @generated from protobuf file "sp_external.proto" (package "protos", syntax proto3)
// tslint:disable
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { External } from "./sp_external.ts";
import type { TestResponse } from "./sp_external.ts";
import type { TestRequest } from "./sp_external.ts";
import type { TailResponse } from "./sp_common.ts";
import type { TailRequest } from "./sp_common.ts";
import type { GetMetricsResponse } from "./sp_external.ts";
import type { GetMetricsRequest } from "./sp_external.ts";
import type { DeleteAudienceRequest } from "./sp_external.ts";
import type { DetachNotificationRequest } from "./sp_external.ts";
import type { AttachNotificationRequest } from "./sp_external.ts";
import type { GetNotificationResponse } from "./sp_external.ts";
import type { GetNotificationRequest } from "./sp_external.ts";
import type { GetNotificationsResponse } from "./sp_external.ts";
import type { GetNotificationsRequest } from "./sp_external.ts";
import type { DeleteNotificationRequest } from "./sp_external.ts";
import type { UpdateNotificationRequest } from "./sp_external.ts";
import type { CreateNotificationRequest } from "./sp_external.ts";
import type { ResumePipelineRequest } from "./sp_external.ts";
import type { PausePipelineRequest } from "./sp_external.ts";
import type { DetachPipelineRequest } from "./sp_external.ts";
import type { AttachPipelineRequest } from "./sp_external.ts";
import type { DeletePipelineRequest } from "./sp_external.ts";
import type { StandardResponse } from "./sp_common.ts";
import type { UpdatePipelineRequest } from "./sp_external.ts";
import type { CreatePipelineResponse } from "./sp_external.ts";
import type { CreatePipelineRequest } from "./sp_external.ts";
import type { GetPipelineResponse } from "./sp_external.ts";
import type { GetPipelineRequest } from "./sp_external.ts";
import type { GetPipelinesResponse } from "./sp_external.ts";
import type { GetPipelinesRequest } from "./sp_external.ts";
import type { ServerStreamingCall } from "@protobuf-ts/runtime-rpc";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { GetAllResponse } from "./sp_external.ts";
import type { GetAllRequest } from "./sp_external.ts";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service protos.External
 */
export interface IExternalClient {
    /**
     * Should return everything that is needed to build the initial view in the console
     *
     * @generated from protobuf rpc: GetAll(protos.GetAllRequest) returns (protos.GetAllResponse);
     */
    getAll(input: GetAllRequest, options?: RpcOptions): UnaryCall<GetAllRequest, GetAllResponse>;
    /**
     * Temporary method to test gRPC-Web streaming
     *
     * @generated from protobuf rpc: GetAllStream(protos.GetAllRequest) returns (stream protos.GetAllResponse);
     */
    getAllStream(input: GetAllRequest, options?: RpcOptions): ServerStreamingCall<GetAllRequest, GetAllResponse>;
    /**
     * Returns pipelines (_wasm_bytes field is stripped)
     *
     * @generated from protobuf rpc: GetPipelines(protos.GetPipelinesRequest) returns (protos.GetPipelinesResponse);
     */
    getPipelines(input: GetPipelinesRequest, options?: RpcOptions): UnaryCall<GetPipelinesRequest, GetPipelinesResponse>;
    /**
     * Returns a single pipeline (_wasm_bytes field is stripped)
     *
     * @generated from protobuf rpc: GetPipeline(protos.GetPipelineRequest) returns (protos.GetPipelineResponse);
     */
    getPipeline(input: GetPipelineRequest, options?: RpcOptions): UnaryCall<GetPipelineRequest, GetPipelineResponse>;
    /**
     * Create a new pipeline; id must be left empty on create
     *
     * @generated from protobuf rpc: CreatePipeline(protos.CreatePipelineRequest) returns (protos.CreatePipelineResponse);
     */
    createPipeline(input: CreatePipelineRequest, options?: RpcOptions): UnaryCall<CreatePipelineRequest, CreatePipelineResponse>;
    /**
     * Update an existing pipeline; id must be set
     *
     * @generated from protobuf rpc: UpdatePipeline(protos.UpdatePipelineRequest) returns (protos.StandardResponse);
     */
    updatePipeline(input: UpdatePipelineRequest, options?: RpcOptions): UnaryCall<UpdatePipelineRequest, StandardResponse>;
    /**
     * Delete a pipeline
     *
     * @generated from protobuf rpc: DeletePipeline(protos.DeletePipelineRequest) returns (protos.StandardResponse);
     */
    deletePipeline(input: DeletePipelineRequest, options?: RpcOptions): UnaryCall<DeletePipelineRequest, StandardResponse>;
    /**
     * Attach a pipeline to an audience
     *
     * @generated from protobuf rpc: AttachPipeline(protos.AttachPipelineRequest) returns (protos.StandardResponse);
     */
    attachPipeline(input: AttachPipelineRequest, options?: RpcOptions): UnaryCall<AttachPipelineRequest, StandardResponse>;
    /**
     * Detach a pipeline from an audience
     *
     * @generated from protobuf rpc: DetachPipeline(protos.DetachPipelineRequest) returns (protos.StandardResponse);
     */
    detachPipeline(input: DetachPipelineRequest, options?: RpcOptions): UnaryCall<DetachPipelineRequest, StandardResponse>;
    /**
     * Pause a pipeline; noop if pipeline is already paused
     *
     * @generated from protobuf rpc: PausePipeline(protos.PausePipelineRequest) returns (protos.StandardResponse);
     */
    pausePipeline(input: PausePipelineRequest, options?: RpcOptions): UnaryCall<PausePipelineRequest, StandardResponse>;
    /**
     * Resume a pipeline; noop if pipeline is not paused
     *
     * @generated from protobuf rpc: ResumePipeline(protos.ResumePipelineRequest) returns (protos.StandardResponse);
     */
    resumePipeline(input: ResumePipelineRequest, options?: RpcOptions): UnaryCall<ResumePipelineRequest, StandardResponse>;
    /**
     * Create a new notification config
     *
     * @generated from protobuf rpc: CreateNotification(protos.CreateNotificationRequest) returns (protos.StandardResponse);
     */
    createNotification(input: CreateNotificationRequest, options?: RpcOptions): UnaryCall<CreateNotificationRequest, StandardResponse>;
    /**
     * Update an existing notification config
     *
     * @generated from protobuf rpc: UpdateNotification(protos.UpdateNotificationRequest) returns (protos.StandardResponse);
     */
    updateNotification(input: UpdateNotificationRequest, options?: RpcOptions): UnaryCall<UpdateNotificationRequest, StandardResponse>;
    /**
     * Delete a notification config
     *
     * @generated from protobuf rpc: DeleteNotification(protos.DeleteNotificationRequest) returns (protos.StandardResponse);
     */
    deleteNotification(input: DeleteNotificationRequest, options?: RpcOptions): UnaryCall<DeleteNotificationRequest, StandardResponse>;
    /**
     * Returns all notification configs
     *
     * @generated from protobuf rpc: GetNotifications(protos.GetNotificationsRequest) returns (protos.GetNotificationsResponse);
     */
    getNotifications(input: GetNotificationsRequest, options?: RpcOptions): UnaryCall<GetNotificationsRequest, GetNotificationsResponse>;
    /**
     * Returns a single notification config
     *
     * @generated from protobuf rpc: GetNotification(protos.GetNotificationRequest) returns (protos.GetNotificationResponse);
     */
    getNotification(input: GetNotificationRequest, options?: RpcOptions): UnaryCall<GetNotificationRequest, GetNotificationResponse>;
    /**
     * Attach a notification config to a pipeline
     *
     * @generated from protobuf rpc: AttachNotification(protos.AttachNotificationRequest) returns (protos.StandardResponse);
     */
    attachNotification(input: AttachNotificationRequest, options?: RpcOptions): UnaryCall<AttachNotificationRequest, StandardResponse>;
    /**
     * Detach a notification config from a pipeline
     *
     * @generated from protobuf rpc: DetachNotification(protos.DetachNotificationRequest) returns (protos.StandardResponse);
     */
    detachNotification(input: DetachNotificationRequest, options?: RpcOptions): UnaryCall<DetachNotificationRequest, StandardResponse>;
    /**
     * Delete an un-attached audience
     *
     * @generated from protobuf rpc: DeleteAudience(protos.DeleteAudienceRequest) returns (protos.StandardResponse);
     */
    deleteAudience(input: DeleteAudienceRequest, options?: RpcOptions): UnaryCall<DeleteAudienceRequest, StandardResponse>;
    /**
     * Returns all metric counters
     *
     * @generated from protobuf rpc: GetMetrics(protos.GetMetricsRequest) returns (stream protos.GetMetricsResponse);
     */
    getMetrics(input: GetMetricsRequest, options?: RpcOptions): ServerStreamingCall<GetMetricsRequest, GetMetricsResponse>;
    /**
     * @generated from protobuf rpc: Tail(protos.TailRequest) returns (stream protos.TailResponse);
     */
    tail(input: TailRequest, options?: RpcOptions): ServerStreamingCall<TailRequest, TailResponse>;
    /**
     * Test method
     *
     * @generated from protobuf rpc: Test(protos.TestRequest) returns (protos.TestResponse);
     */
    test(input: TestRequest, options?: RpcOptions): UnaryCall<TestRequest, TestResponse>;
}
/**
 * @generated from protobuf service protos.External
 */
export class ExternalClient implements IExternalClient, ServiceInfo {
    typeName = External.typeName;
    methods = External.methods;
    options = External.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * Should return everything that is needed to build the initial view in the console
     *
     * @generated from protobuf rpc: GetAll(protos.GetAllRequest) returns (protos.GetAllResponse);
     */
    getAll(input: GetAllRequest, options?: RpcOptions): UnaryCall<GetAllRequest, GetAllResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetAllRequest, GetAllResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Temporary method to test gRPC-Web streaming
     *
     * @generated from protobuf rpc: GetAllStream(protos.GetAllRequest) returns (stream protos.GetAllResponse);
     */
    getAllStream(input: GetAllRequest, options?: RpcOptions): ServerStreamingCall<GetAllRequest, GetAllResponse> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetAllRequest, GetAllResponse>("serverStreaming", this._transport, method, opt, input);
    }
    /**
     * Returns pipelines (_wasm_bytes field is stripped)
     *
     * @generated from protobuf rpc: GetPipelines(protos.GetPipelinesRequest) returns (protos.GetPipelinesResponse);
     */
    getPipelines(input: GetPipelinesRequest, options?: RpcOptions): UnaryCall<GetPipelinesRequest, GetPipelinesResponse> {
        const method = this.methods[2], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetPipelinesRequest, GetPipelinesResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Returns a single pipeline (_wasm_bytes field is stripped)
     *
     * @generated from protobuf rpc: GetPipeline(protos.GetPipelineRequest) returns (protos.GetPipelineResponse);
     */
    getPipeline(input: GetPipelineRequest, options?: RpcOptions): UnaryCall<GetPipelineRequest, GetPipelineResponse> {
        const method = this.methods[3], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetPipelineRequest, GetPipelineResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Create a new pipeline; id must be left empty on create
     *
     * @generated from protobuf rpc: CreatePipeline(protos.CreatePipelineRequest) returns (protos.CreatePipelineResponse);
     */
    createPipeline(input: CreatePipelineRequest, options?: RpcOptions): UnaryCall<CreatePipelineRequest, CreatePipelineResponse> {
        const method = this.methods[4], opt = this._transport.mergeOptions(options);
        return stackIntercept<CreatePipelineRequest, CreatePipelineResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Update an existing pipeline; id must be set
     *
     * @generated from protobuf rpc: UpdatePipeline(protos.UpdatePipelineRequest) returns (protos.StandardResponse);
     */
    updatePipeline(input: UpdatePipelineRequest, options?: RpcOptions): UnaryCall<UpdatePipelineRequest, StandardResponse> {
        const method = this.methods[5], opt = this._transport.mergeOptions(options);
        return stackIntercept<UpdatePipelineRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Delete a pipeline
     *
     * @generated from protobuf rpc: DeletePipeline(protos.DeletePipelineRequest) returns (protos.StandardResponse);
     */
    deletePipeline(input: DeletePipelineRequest, options?: RpcOptions): UnaryCall<DeletePipelineRequest, StandardResponse> {
        const method = this.methods[6], opt = this._transport.mergeOptions(options);
        return stackIntercept<DeletePipelineRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Attach a pipeline to an audience
     *
     * @generated from protobuf rpc: AttachPipeline(protos.AttachPipelineRequest) returns (protos.StandardResponse);
     */
    attachPipeline(input: AttachPipelineRequest, options?: RpcOptions): UnaryCall<AttachPipelineRequest, StandardResponse> {
        const method = this.methods[7], opt = this._transport.mergeOptions(options);
        return stackIntercept<AttachPipelineRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Detach a pipeline from an audience
     *
     * @generated from protobuf rpc: DetachPipeline(protos.DetachPipelineRequest) returns (protos.StandardResponse);
     */
    detachPipeline(input: DetachPipelineRequest, options?: RpcOptions): UnaryCall<DetachPipelineRequest, StandardResponse> {
        const method = this.methods[8], opt = this._transport.mergeOptions(options);
        return stackIntercept<DetachPipelineRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Pause a pipeline; noop if pipeline is already paused
     *
     * @generated from protobuf rpc: PausePipeline(protos.PausePipelineRequest) returns (protos.StandardResponse);
     */
    pausePipeline(input: PausePipelineRequest, options?: RpcOptions): UnaryCall<PausePipelineRequest, StandardResponse> {
        const method = this.methods[9], opt = this._transport.mergeOptions(options);
        return stackIntercept<PausePipelineRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Resume a pipeline; noop if pipeline is not paused
     *
     * @generated from protobuf rpc: ResumePipeline(protos.ResumePipelineRequest) returns (protos.StandardResponse);
     */
    resumePipeline(input: ResumePipelineRequest, options?: RpcOptions): UnaryCall<ResumePipelineRequest, StandardResponse> {
        const method = this.methods[10], opt = this._transport.mergeOptions(options);
        return stackIntercept<ResumePipelineRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Create a new notification config
     *
     * @generated from protobuf rpc: CreateNotification(protos.CreateNotificationRequest) returns (protos.StandardResponse);
     */
    createNotification(input: CreateNotificationRequest, options?: RpcOptions): UnaryCall<CreateNotificationRequest, StandardResponse> {
        const method = this.methods[11], opt = this._transport.mergeOptions(options);
        return stackIntercept<CreateNotificationRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Update an existing notification config
     *
     * @generated from protobuf rpc: UpdateNotification(protos.UpdateNotificationRequest) returns (protos.StandardResponse);
     */
    updateNotification(input: UpdateNotificationRequest, options?: RpcOptions): UnaryCall<UpdateNotificationRequest, StandardResponse> {
        const method = this.methods[12], opt = this._transport.mergeOptions(options);
        return stackIntercept<UpdateNotificationRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Delete a notification config
     *
     * @generated from protobuf rpc: DeleteNotification(protos.DeleteNotificationRequest) returns (protos.StandardResponse);
     */
    deleteNotification(input: DeleteNotificationRequest, options?: RpcOptions): UnaryCall<DeleteNotificationRequest, StandardResponse> {
        const method = this.methods[13], opt = this._transport.mergeOptions(options);
        return stackIntercept<DeleteNotificationRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Returns all notification configs
     *
     * @generated from protobuf rpc: GetNotifications(protos.GetNotificationsRequest) returns (protos.GetNotificationsResponse);
     */
    getNotifications(input: GetNotificationsRequest, options?: RpcOptions): UnaryCall<GetNotificationsRequest, GetNotificationsResponse> {
        const method = this.methods[14], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetNotificationsRequest, GetNotificationsResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Returns a single notification config
     *
     * @generated from protobuf rpc: GetNotification(protos.GetNotificationRequest) returns (protos.GetNotificationResponse);
     */
    getNotification(input: GetNotificationRequest, options?: RpcOptions): UnaryCall<GetNotificationRequest, GetNotificationResponse> {
        const method = this.methods[15], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetNotificationRequest, GetNotificationResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Attach a notification config to a pipeline
     *
     * @generated from protobuf rpc: AttachNotification(protos.AttachNotificationRequest) returns (protos.StandardResponse);
     */
    attachNotification(input: AttachNotificationRequest, options?: RpcOptions): UnaryCall<AttachNotificationRequest, StandardResponse> {
        const method = this.methods[16], opt = this._transport.mergeOptions(options);
        return stackIntercept<AttachNotificationRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Detach a notification config from a pipeline
     *
     * @generated from protobuf rpc: DetachNotification(protos.DetachNotificationRequest) returns (protos.StandardResponse);
     */
    detachNotification(input: DetachNotificationRequest, options?: RpcOptions): UnaryCall<DetachNotificationRequest, StandardResponse> {
        const method = this.methods[17], opt = this._transport.mergeOptions(options);
        return stackIntercept<DetachNotificationRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Delete an un-attached audience
     *
     * @generated from protobuf rpc: DeleteAudience(protos.DeleteAudienceRequest) returns (protos.StandardResponse);
     */
    deleteAudience(input: DeleteAudienceRequest, options?: RpcOptions): UnaryCall<DeleteAudienceRequest, StandardResponse> {
        const method = this.methods[18], opt = this._transport.mergeOptions(options);
        return stackIntercept<DeleteAudienceRequest, StandardResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * Returns all metric counters
     *
     * @generated from protobuf rpc: GetMetrics(protos.GetMetricsRequest) returns (stream protos.GetMetricsResponse);
     */
    getMetrics(input: GetMetricsRequest, options?: RpcOptions): ServerStreamingCall<GetMetricsRequest, GetMetricsResponse> {
        const method = this.methods[19], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetMetricsRequest, GetMetricsResponse>("serverStreaming", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: Tail(protos.TailRequest) returns (stream protos.TailResponse);
     */
    tail(input: TailRequest, options?: RpcOptions): ServerStreamingCall<TailRequest, TailResponse> {
        const method = this.methods[20], opt = this._transport.mergeOptions(options);
        return stackIntercept<TailRequest, TailResponse>("serverStreaming", this._transport, method, opt, input);
    }
    /**
     * Test method
     *
     * @generated from protobuf rpc: Test(protos.TestRequest) returns (protos.TestResponse);
     */
    test(input: TestRequest, options?: RpcOptions): UnaryCall<TestRequest, TestResponse> {
        const method = this.methods[21], opt = this._transport.mergeOptions(options);
        return stackIntercept<TestRequest, TestResponse>("unary", this._transport, method, opt, input);
    }
}
