// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        v3.21.12
// source: sp_sdk.proto

package protos

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type AbortStatus int32

const (
	AbortStatus_ABORT_STATUS_UNSET   AbortStatus = 0
	AbortStatus_ABORT_STATUS_CURRENT AbortStatus = 1
	AbortStatus_ABORT_STATUS_ALL     AbortStatus = 2
)

// Enum value maps for AbortStatus.
var (
	AbortStatus_name = map[int32]string{
		0: "ABORT_STATUS_UNSET",
		1: "ABORT_STATUS_CURRENT",
		2: "ABORT_STATUS_ALL",
	}
	AbortStatus_value = map[string]int32{
		"ABORT_STATUS_UNSET":   0,
		"ABORT_STATUS_CURRENT": 1,
		"ABORT_STATUS_ALL":     2,
	}
)

func (x AbortStatus) Enum() *AbortStatus {
	p := new(AbortStatus)
	*p = x
	return p
}

func (x AbortStatus) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (AbortStatus) Descriptor() protoreflect.EnumDescriptor {
	return file_sp_sdk_proto_enumTypes[0].Descriptor()
}

func (AbortStatus) Type() protoreflect.EnumType {
	return &file_sp_sdk_proto_enumTypes[0]
}

func (x AbortStatus) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use AbortStatus.Descriptor instead.
func (AbortStatus) EnumDescriptor() ([]byte, []int) {
	return file_sp_sdk_proto_rawDescGZIP(), []int{0}
}

// Common return response used by all SDKs
type SDKResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains (potentially) modified input data
	Data []byte `protobuf:"bytes,1,opt,name=data,proto3" json:"data,omitempty"`
	// Indicates if .Process() was successful; check error_message for more details
	Error bool `protobuf:"varint,2,opt,name=error,proto3" json:"error,omitempty"`
	// If an error == true, this will contain a human-readable error message
	ErrorMessage string `protobuf:"bytes,3,opt,name=error_message,json=errorMessage,proto3" json:"error_message,omitempty"`
	// An array of pipelines that the SDK executed and the status of each step
	PipelineStatus []*PipelineStatus `protobuf:"bytes,4,rep,name=pipeline_status,json=pipelineStatus,proto3" json:"pipeline_status,omitempty"`
}

func (x *SDKResponse) Reset() {
	*x = SDKResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sp_sdk_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SDKResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SDKResponse) ProtoMessage() {}

func (x *SDKResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sp_sdk_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SDKResponse.ProtoReflect.Descriptor instead.
func (*SDKResponse) Descriptor() ([]byte, []int) {
	return file_sp_sdk_proto_rawDescGZIP(), []int{0}
}

func (x *SDKResponse) GetData() []byte {
	if x != nil {
		return x.Data
	}
	return nil
}

func (x *SDKResponse) GetError() bool {
	if x != nil {
		return x.Error
	}
	return false
}

func (x *SDKResponse) GetErrorMessage() string {
	if x != nil {
		return x.ErrorMessage
	}
	return ""
}

func (x *SDKResponse) GetPipelineStatus() []*PipelineStatus {
	if x != nil {
		return x.PipelineStatus
	}
	return nil
}

type PipelineStatus struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// ID of the pipeline
	Id string `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	// The name of the pipeline
	Name string `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	// The status of each step in the pipeline
	StepStatus []*StepStatus `protobuf:"bytes,3,rep,name=step_status,json=stepStatus,proto3" json:"step_status,omitempty"`
}

func (x *PipelineStatus) Reset() {
	*x = PipelineStatus{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sp_sdk_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *PipelineStatus) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PipelineStatus) ProtoMessage() {}

func (x *PipelineStatus) ProtoReflect() protoreflect.Message {
	mi := &file_sp_sdk_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use PipelineStatus.ProtoReflect.Descriptor instead.
func (*PipelineStatus) Descriptor() ([]byte, []int) {
	return file_sp_sdk_proto_rawDescGZIP(), []int{1}
}

func (x *PipelineStatus) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *PipelineStatus) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *PipelineStatus) GetStepStatus() []*StepStatus {
	if x != nil {
		return x.StepStatus
	}
	return nil
}

type StepStatus struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The name of the step
	Name string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	// Did an error occur during the step?
	Error bool `protobuf:"varint,2,opt,name=error,proto3" json:"error,omitempty"`
	// If error == true, this will contain a human-readable error message
	ErrorMessage string `protobuf:"bytes,3,opt,name=error_message,json=errorMessage,proto3" json:"error_message,omitempty"`
	// If error == true, this will indicate whether current or upcoming pipeline
	// execution was aborted.
	AbortStatus AbortStatus `protobuf:"varint,4,opt,name=abort_status,json=abortStatus,proto3,enum=protos.AbortStatus" json:"abort_status,omitempty"`
}

func (x *StepStatus) Reset() {
	*x = StepStatus{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sp_sdk_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *StepStatus) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*StepStatus) ProtoMessage() {}

func (x *StepStatus) ProtoReflect() protoreflect.Message {
	mi := &file_sp_sdk_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use StepStatus.ProtoReflect.Descriptor instead.
func (*StepStatus) Descriptor() ([]byte, []int) {
	return file_sp_sdk_proto_rawDescGZIP(), []int{2}
}

func (x *StepStatus) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *StepStatus) GetError() bool {
	if x != nil {
		return x.Error
	}
	return false
}

func (x *StepStatus) GetErrorMessage() string {
	if x != nil {
		return x.ErrorMessage
	}
	return ""
}

func (x *StepStatus) GetAbortStatus() AbortStatus {
	if x != nil {
		return x.AbortStatus
	}
	return AbortStatus_ABORT_STATUS_UNSET
}

var File_sp_sdk_proto protoreflect.FileDescriptor

var file_sp_sdk_proto_rawDesc = []byte{
	0x0a, 0x0c, 0x73, 0x70, 0x5f, 0x73, 0x64, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x22, 0x9d, 0x01, 0x0a, 0x0b, 0x53, 0x44, 0x4b, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72,
	0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
	0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
	0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65,
	0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3f, 0x0a, 0x0f, 0x70, 0x69, 0x70, 0x65, 0x6c, 0x69, 0x6e,
	0x65, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x16,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x50, 0x69, 0x70, 0x65, 0x6c, 0x69, 0x6e, 0x65,
	0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x0e, 0x70, 0x69, 0x70, 0x65, 0x6c, 0x69, 0x6e, 0x65,
	0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x69, 0x0a, 0x0e, 0x50, 0x69, 0x70, 0x65, 0x6c, 0x69,
	0x6e, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x33, 0x0a, 0x0b,
	0x73, 0x74, 0x65, 0x70, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
	0x0b, 0x32, 0x12, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x53,
	0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x0a, 0x73, 0x74, 0x65, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75,
	0x73, 0x22, 0x93, 0x01, 0x0a, 0x0a, 0x53, 0x74, 0x65, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
	0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
	0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x08, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72,
	0x72, 0x6f, 0x72, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x0c, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
	0x36, 0x0a, 0x0c, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18,
	0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x41,
	0x62, 0x6f, 0x72, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x0b, 0x61, 0x62, 0x6f, 0x72,
	0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2a, 0x55, 0x0a, 0x0b, 0x41, 0x62, 0x6f, 0x72, 0x74,
	0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x16, 0x0a, 0x12, 0x41, 0x42, 0x4f, 0x52, 0x54, 0x5f,
	0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x18,
	0x0a, 0x14, 0x41, 0x42, 0x4f, 0x52, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x43,
	0x55, 0x52, 0x52, 0x45, 0x4e, 0x54, 0x10, 0x01, 0x12, 0x14, 0x0a, 0x10, 0x41, 0x42, 0x4f, 0x52,
	0x54, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x41, 0x4c, 0x4c, 0x10, 0x02, 0x42, 0x37,
	0x5a, 0x35, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x74, 0x72,
	0x65, 0x61, 0x6d, 0x64, 0x61, 0x6c, 0x2f, 0x6d, 0x6f, 0x6e, 0x6f, 0x2f, 0x6c, 0x69, 0x62, 0x73,
	0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x67, 0x6f,
	0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_sp_sdk_proto_rawDescOnce sync.Once
	file_sp_sdk_proto_rawDescData = file_sp_sdk_proto_rawDesc
)

func file_sp_sdk_proto_rawDescGZIP() []byte {
	file_sp_sdk_proto_rawDescOnce.Do(func() {
		file_sp_sdk_proto_rawDescData = protoimpl.X.CompressGZIP(file_sp_sdk_proto_rawDescData)
	})
	return file_sp_sdk_proto_rawDescData
}

var file_sp_sdk_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_sp_sdk_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_sp_sdk_proto_goTypes = []interface{}{
	(AbortStatus)(0),       // 0: protos.AbortStatus
	(*SDKResponse)(nil),    // 1: protos.SDKResponse
	(*PipelineStatus)(nil), // 2: protos.PipelineStatus
	(*StepStatus)(nil),     // 3: protos.StepStatus
}
var file_sp_sdk_proto_depIdxs = []int32{
	2, // 0: protos.SDKResponse.pipeline_status:type_name -> protos.PipelineStatus
	3, // 1: protos.PipelineStatus.step_status:type_name -> protos.StepStatus
	0, // 2: protos.StepStatus.abort_status:type_name -> protos.AbortStatus
	3, // [3:3] is the sub-list for method output_type
	3, // [3:3] is the sub-list for method input_type
	3, // [3:3] is the sub-list for extension type_name
	3, // [3:3] is the sub-list for extension extendee
	0, // [0:3] is the sub-list for field type_name
}

func init() { file_sp_sdk_proto_init() }
func file_sp_sdk_proto_init() {
	if File_sp_sdk_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sp_sdk_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SDKResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sp_sdk_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*PipelineStatus); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sp_sdk_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*StepStatus); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_sp_sdk_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_sp_sdk_proto_goTypes,
		DependencyIndexes: file_sp_sdk_proto_depIdxs,
		EnumInfos:         file_sp_sdk_proto_enumTypes,
		MessageInfos:      file_sp_sdk_proto_msgTypes,
	}.Build()
	File_sp_sdk_proto = out.File
	file_sp_sdk_proto_rawDesc = nil
	file_sp_sdk_proto_goTypes = nil
	file_sp_sdk_proto_depIdxs = nil
}
