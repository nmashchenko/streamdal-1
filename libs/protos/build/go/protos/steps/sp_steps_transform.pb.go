// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        v3.21.12
// source: steps/sp_steps_transform.proto

package steps

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

type TransformType int32

const (
	TransformType_TRANSFORM_TYPE_UNKNOWN         TransformType = 0
	TransformType_TRANSFORM_TYPE_REPLACE_VALUE   TransformType = 1
	TransformType_TRANSFORM_TYPE_DELETE_FIELD    TransformType = 2
	TransformType_TRANSFORM_TYPE_OBFUSCATE_VALUE TransformType = 3
	TransformType_TRANSFORM_TYPE_MASK_VALUE      TransformType = 4
	TransformType_TRANSFORM_TYPE_TRUNCATE_VALUE  TransformType = 5 // TODO: type for delete all keys except specified ones
)

// Enum value maps for TransformType.
var (
	TransformType_name = map[int32]string{
		0: "TRANSFORM_TYPE_UNKNOWN",
		1: "TRANSFORM_TYPE_REPLACE_VALUE",
		2: "TRANSFORM_TYPE_DELETE_FIELD",
		3: "TRANSFORM_TYPE_OBFUSCATE_VALUE",
		4: "TRANSFORM_TYPE_MASK_VALUE",
		5: "TRANSFORM_TYPE_TRUNCATE_VALUE",
	}
	TransformType_value = map[string]int32{
		"TRANSFORM_TYPE_UNKNOWN":         0,
		"TRANSFORM_TYPE_REPLACE_VALUE":   1,
		"TRANSFORM_TYPE_DELETE_FIELD":    2,
		"TRANSFORM_TYPE_OBFUSCATE_VALUE": 3,
		"TRANSFORM_TYPE_MASK_VALUE":      4,
		"TRANSFORM_TYPE_TRUNCATE_VALUE":  5,
	}
)

func (x TransformType) Enum() *TransformType {
	p := new(TransformType)
	*p = x
	return p
}

func (x TransformType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (TransformType) Descriptor() protoreflect.EnumDescriptor {
	return file_steps_sp_steps_transform_proto_enumTypes[0].Descriptor()
}

func (TransformType) Type() protoreflect.EnumType {
	return &file_steps_sp_steps_transform_proto_enumTypes[0]
}

func (x TransformType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use TransformType.Descriptor instead.
func (TransformType) EnumDescriptor() ([]byte, []int) {
	return file_steps_sp_steps_transform_proto_rawDescGZIP(), []int{0}
}

type TransformTruncateType int32

const (
	TransformTruncateType_TRANSFORM_TRUNCATE_TYPE_UNKNOWN    TransformTruncateType = 0
	TransformTruncateType_TRANSFORM_TRUNCATE_TYPE_LENGTH     TransformTruncateType = 1
	TransformTruncateType_TRANSFORM_TRUNCATE_TYPE_PERCENTAGE TransformTruncateType = 2
)

// Enum value maps for TransformTruncateType.
var (
	TransformTruncateType_name = map[int32]string{
		0: "TRANSFORM_TRUNCATE_TYPE_UNKNOWN",
		1: "TRANSFORM_TRUNCATE_TYPE_LENGTH",
		2: "TRANSFORM_TRUNCATE_TYPE_PERCENTAGE",
	}
	TransformTruncateType_value = map[string]int32{
		"TRANSFORM_TRUNCATE_TYPE_UNKNOWN":    0,
		"TRANSFORM_TRUNCATE_TYPE_LENGTH":     1,
		"TRANSFORM_TRUNCATE_TYPE_PERCENTAGE": 2,
	}
)

func (x TransformTruncateType) Enum() *TransformTruncateType {
	p := new(TransformTruncateType)
	*p = x
	return p
}

func (x TransformTruncateType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (TransformTruncateType) Descriptor() protoreflect.EnumDescriptor {
	return file_steps_sp_steps_transform_proto_enumTypes[1].Descriptor()
}

func (TransformTruncateType) Type() protoreflect.EnumType {
	return &file_steps_sp_steps_transform_proto_enumTypes[1]
}

func (x TransformTruncateType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use TransformTruncateType.Descriptor instead.
func (TransformTruncateType) EnumDescriptor() ([]byte, []int) {
	return file_steps_sp_steps_transform_proto_rawDescGZIP(), []int{1}
}

type TransformStep struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Deprecated: Marked as deprecated in steps/sp_steps_transform.proto.
	Path string `protobuf:"bytes,1,opt,name=path,proto3" json:"path,omitempty"`
	// Deprecated: Marked as deprecated in steps/sp_steps_transform.proto.
	Value string        `protobuf:"bytes,2,opt,name=value,proto3" json:"value,omitempty"` // Should this be bytes? ~DS
	Type  TransformType `protobuf:"varint,3,opt,name=type,proto3,enum=protos.steps.TransformType" json:"type,omitempty"`
	// Types that are assignable to Options:
	//
	//	*TransformStep_ReplaceValueOptions
	//	*TransformStep_DeleteFieldOptions
	//	*TransformStep_ObfuscateOptions
	//	*TransformStep_MaskOptions
	//	*TransformStep_TruncateOptions
	Options isTransformStep_Options `protobuf_oneof:"options"`
}

func (x *TransformStep) Reset() {
	*x = TransformStep{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steps_sp_steps_transform_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TransformStep) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TransformStep) ProtoMessage() {}

func (x *TransformStep) ProtoReflect() protoreflect.Message {
	mi := &file_steps_sp_steps_transform_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TransformStep.ProtoReflect.Descriptor instead.
func (*TransformStep) Descriptor() ([]byte, []int) {
	return file_steps_sp_steps_transform_proto_rawDescGZIP(), []int{0}
}

// Deprecated: Marked as deprecated in steps/sp_steps_transform.proto.
func (x *TransformStep) GetPath() string {
	if x != nil {
		return x.Path
	}
	return ""
}

// Deprecated: Marked as deprecated in steps/sp_steps_transform.proto.
func (x *TransformStep) GetValue() string {
	if x != nil {
		return x.Value
	}
	return ""
}

func (x *TransformStep) GetType() TransformType {
	if x != nil {
		return x.Type
	}
	return TransformType_TRANSFORM_TYPE_UNKNOWN
}

func (m *TransformStep) GetOptions() isTransformStep_Options {
	if m != nil {
		return m.Options
	}
	return nil
}

func (x *TransformStep) GetReplaceValueOptions() *TransformReplaceValueStep {
	if x, ok := x.GetOptions().(*TransformStep_ReplaceValueOptions); ok {
		return x.ReplaceValueOptions
	}
	return nil
}

func (x *TransformStep) GetDeleteFieldOptions() *TransformDeleteFieldStep {
	if x, ok := x.GetOptions().(*TransformStep_DeleteFieldOptions); ok {
		return x.DeleteFieldOptions
	}
	return nil
}

func (x *TransformStep) GetObfuscateOptions() *TransformObfuscateOptions {
	if x, ok := x.GetOptions().(*TransformStep_ObfuscateOptions); ok {
		return x.ObfuscateOptions
	}
	return nil
}

func (x *TransformStep) GetMaskOptions() *TransformMaskOptions {
	if x, ok := x.GetOptions().(*TransformStep_MaskOptions); ok {
		return x.MaskOptions
	}
	return nil
}

func (x *TransformStep) GetTruncateOptions() *TransformTruncateOptions {
	if x, ok := x.GetOptions().(*TransformStep_TruncateOptions); ok {
		return x.TruncateOptions
	}
	return nil
}

type isTransformStep_Options interface {
	isTransformStep_Options()
}

type TransformStep_ReplaceValueOptions struct {
	ReplaceValueOptions *TransformReplaceValueStep `protobuf:"bytes,101,opt,name=replace_value_options,json=replaceValueOptions,proto3,oneof"`
}

type TransformStep_DeleteFieldOptions struct {
	DeleteFieldOptions *TransformDeleteFieldStep `protobuf:"bytes,102,opt,name=delete_field_options,json=deleteFieldOptions,proto3,oneof"`
}

type TransformStep_ObfuscateOptions struct {
	ObfuscateOptions *TransformObfuscateOptions `protobuf:"bytes,103,opt,name=obfuscate_options,json=obfuscateOptions,proto3,oneof"`
}

type TransformStep_MaskOptions struct {
	MaskOptions *TransformMaskOptions `protobuf:"bytes,104,opt,name=mask_options,json=maskOptions,proto3,oneof"`
}

type TransformStep_TruncateOptions struct {
	TruncateOptions *TransformTruncateOptions `protobuf:"bytes,105,opt,name=truncate_options,json=truncateOptions,proto3,oneof"`
}

func (*TransformStep_ReplaceValueOptions) isTransformStep_Options() {}

func (*TransformStep_DeleteFieldOptions) isTransformStep_Options() {}

func (*TransformStep_ObfuscateOptions) isTransformStep_Options() {}

func (*TransformStep_MaskOptions) isTransformStep_Options() {}

func (*TransformStep_TruncateOptions) isTransformStep_Options() {}

type TransformTruncateOptions struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Type TransformTruncateType `protobuf:"varint,1,opt,name=type,proto3,enum=protos.steps.TransformTruncateType" json:"type,omitempty"`
	Path string                `protobuf:"bytes,2,opt,name=path,proto3" json:"path,omitempty"`
	// Truncate after this many bytes or this percentage of the original value
	Value int32 `protobuf:"varint,3,opt,name=value,proto3" json:"value,omitempty"`
}

func (x *TransformTruncateOptions) Reset() {
	*x = TransformTruncateOptions{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steps_sp_steps_transform_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TransformTruncateOptions) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TransformTruncateOptions) ProtoMessage() {}

func (x *TransformTruncateOptions) ProtoReflect() protoreflect.Message {
	mi := &file_steps_sp_steps_transform_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TransformTruncateOptions.ProtoReflect.Descriptor instead.
func (*TransformTruncateOptions) Descriptor() ([]byte, []int) {
	return file_steps_sp_steps_transform_proto_rawDescGZIP(), []int{1}
}

func (x *TransformTruncateOptions) GetType() TransformTruncateType {
	if x != nil {
		return x.Type
	}
	return TransformTruncateType_TRANSFORM_TRUNCATE_TYPE_UNKNOWN
}

func (x *TransformTruncateOptions) GetPath() string {
	if x != nil {
		return x.Path
	}
	return ""
}

func (x *TransformTruncateOptions) GetValue() int32 {
	if x != nil {
		return x.Value
	}
	return 0
}

type TransformDeleteFieldStep struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Path string `protobuf:"bytes,1,opt,name=path,proto3" json:"path,omitempty"`
}

func (x *TransformDeleteFieldStep) Reset() {
	*x = TransformDeleteFieldStep{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steps_sp_steps_transform_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TransformDeleteFieldStep) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TransformDeleteFieldStep) ProtoMessage() {}

func (x *TransformDeleteFieldStep) ProtoReflect() protoreflect.Message {
	mi := &file_steps_sp_steps_transform_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TransformDeleteFieldStep.ProtoReflect.Descriptor instead.
func (*TransformDeleteFieldStep) Descriptor() ([]byte, []int) {
	return file_steps_sp_steps_transform_proto_rawDescGZIP(), []int{2}
}

func (x *TransformDeleteFieldStep) GetPath() string {
	if x != nil {
		return x.Path
	}
	return ""
}

type TransformReplaceValueStep struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Path  string `protobuf:"bytes,1,opt,name=path,proto3" json:"path,omitempty"`
	Value string `protobuf:"bytes,2,opt,name=value,proto3" json:"value,omitempty"`
}

func (x *TransformReplaceValueStep) Reset() {
	*x = TransformReplaceValueStep{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steps_sp_steps_transform_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TransformReplaceValueStep) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TransformReplaceValueStep) ProtoMessage() {}

func (x *TransformReplaceValueStep) ProtoReflect() protoreflect.Message {
	mi := &file_steps_sp_steps_transform_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TransformReplaceValueStep.ProtoReflect.Descriptor instead.
func (*TransformReplaceValueStep) Descriptor() ([]byte, []int) {
	return file_steps_sp_steps_transform_proto_rawDescGZIP(), []int{3}
}

func (x *TransformReplaceValueStep) GetPath() string {
	if x != nil {
		return x.Path
	}
	return ""
}

func (x *TransformReplaceValueStep) GetValue() string {
	if x != nil {
		return x.Value
	}
	return ""
}

type TransformObfuscateOptions struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Path string `protobuf:"bytes,1,opt,name=path,proto3" json:"path,omitempty"`
}

func (x *TransformObfuscateOptions) Reset() {
	*x = TransformObfuscateOptions{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steps_sp_steps_transform_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TransformObfuscateOptions) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TransformObfuscateOptions) ProtoMessage() {}

func (x *TransformObfuscateOptions) ProtoReflect() protoreflect.Message {
	mi := &file_steps_sp_steps_transform_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TransformObfuscateOptions.ProtoReflect.Descriptor instead.
func (*TransformObfuscateOptions) Descriptor() ([]byte, []int) {
	return file_steps_sp_steps_transform_proto_rawDescGZIP(), []int{4}
}

func (x *TransformObfuscateOptions) GetPath() string {
	if x != nil {
		return x.Path
	}
	return ""
}

type TransformMaskOptions struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Path string `protobuf:"bytes,1,opt,name=path,proto3" json:"path,omitempty"`
	Mask string `protobuf:"bytes,2,opt,name=mask,proto3" json:"mask,omitempty"`
}

func (x *TransformMaskOptions) Reset() {
	*x = TransformMaskOptions{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steps_sp_steps_transform_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TransformMaskOptions) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TransformMaskOptions) ProtoMessage() {}

func (x *TransformMaskOptions) ProtoReflect() protoreflect.Message {
	mi := &file_steps_sp_steps_transform_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TransformMaskOptions.ProtoReflect.Descriptor instead.
func (*TransformMaskOptions) Descriptor() ([]byte, []int) {
	return file_steps_sp_steps_transform_proto_rawDescGZIP(), []int{5}
}

func (x *TransformMaskOptions) GetPath() string {
	if x != nil {
		return x.Path
	}
	return ""
}

func (x *TransformMaskOptions) GetMask() string {
	if x != nil {
		return x.Mask
	}
	return ""
}

var File_steps_sp_steps_transform_proto protoreflect.FileDescriptor

var file_steps_sp_steps_transform_proto_rawDesc = []byte{
	0x0a, 0x1e, 0x73, 0x74, 0x65, 0x70, 0x73, 0x2f, 0x73, 0x70, 0x5f, 0x73, 0x74, 0x65, 0x70, 0x73,
	0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x12, 0x0c, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x73, 0x74, 0x65, 0x70, 0x73, 0x22, 0xae,
	0x04, 0x0a, 0x0d, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x53, 0x74, 0x65, 0x70,
	0x12, 0x16, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x02,
	0x18, 0x01, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x18, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
	0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x02, 0x18, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c,
	0x75, 0x65, 0x12, 0x2f, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e,
	0x32, 0x1b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x73, 0x74, 0x65, 0x70, 0x73, 0x2e,
	0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74,
	0x79, 0x70, 0x65, 0x12, 0x5d, 0x0a, 0x15, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x5f, 0x76,
	0x61, 0x6c, 0x75, 0x65, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x65, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x27, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x73, 0x74, 0x65, 0x70,
	0x73, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x70, 0x6c, 0x61,
	0x63, 0x65, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x53, 0x74, 0x65, 0x70, 0x48, 0x00, 0x52, 0x13, 0x72,
	0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x12, 0x5a, 0x0a, 0x14, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x66, 0x69, 0x65,
	0x6c, 0x64, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x66, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x26, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x73, 0x74, 0x65, 0x70, 0x73, 0x2e,
	0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x46,
	0x69, 0x65, 0x6c, 0x64, 0x53, 0x74, 0x65, 0x70, 0x48, 0x00, 0x52, 0x12, 0x64, 0x65, 0x6c, 0x65,
	0x74, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x56,
	0x0a, 0x11, 0x6f, 0x62, 0x66, 0x75, 0x73, 0x63, 0x61, 0x74, 0x65, 0x5f, 0x6f, 0x70, 0x74, 0x69,
	0x6f, 0x6e, 0x73, 0x18, 0x67, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x73, 0x2e, 0x73, 0x74, 0x65, 0x70, 0x73, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f,
	0x72, 0x6d, 0x4f, 0x62, 0x66, 0x75, 0x73, 0x63, 0x61, 0x74, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x48, 0x00, 0x52, 0x10, 0x6f, 0x62, 0x66, 0x75, 0x73, 0x63, 0x61, 0x74, 0x65, 0x4f,
	0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x47, 0x0a, 0x0c, 0x6d, 0x61, 0x73, 0x6b, 0x5f, 0x6f,
	0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x68, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x73, 0x74, 0x65, 0x70, 0x73, 0x2e, 0x54, 0x72, 0x61, 0x6e,
	0x73, 0x66, 0x6f, 0x72, 0x6d, 0x4d, 0x61, 0x73, 0x6b, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73,
	0x48, 0x00, 0x52, 0x0b, 0x6d, 0x61, 0x73, 0x6b, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12,
	0x53, 0x0a, 0x10, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x5f, 0x6f, 0x70, 0x74, 0x69,
	0x6f, 0x6e, 0x73, 0x18, 0x69, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x73, 0x2e, 0x73, 0x74, 0x65, 0x70, 0x73, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f,
	0x72, 0x6d, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
	0x73, 0x48, 0x00, 0x52, 0x0f, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x4f, 0x70, 0x74,
	0x69, 0x6f, 0x6e, 0x73, 0x42, 0x09, 0x0a, 0x07, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22,
	0x7d, 0x0a, 0x18, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x54, 0x72, 0x75, 0x6e,
	0x63, 0x61, 0x74, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x37, 0x0a, 0x04, 0x74,
	0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x23, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x73, 0x2e, 0x73, 0x74, 0x65, 0x70, 0x73, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f,
	0x72, 0x6d, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04,
	0x74, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
	0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x2e,
	0x0a, 0x18, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x44, 0x65, 0x6c, 0x65, 0x74,
	0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x53, 0x74, 0x65, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61,
	0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x45,
	0x0a, 0x19, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x70, 0x6c, 0x61,
	0x63, 0x65, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x53, 0x74, 0x65, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x70,
	0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12,
	0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05,
	0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x2f, 0x0a, 0x19, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f,
	0x72, 0x6d, 0x4f, 0x62, 0x66, 0x75, 0x73, 0x63, 0x61, 0x74, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x3e, 0x0a, 0x14, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66,
	0x6f, 0x72, 0x6d, 0x4d, 0x61, 0x73, 0x6b, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x12,
	0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61,
	0x74, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x6d, 0x61, 0x73, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x04, 0x6d, 0x61, 0x73, 0x6b, 0x2a, 0xd4, 0x01, 0x0a, 0x0d, 0x54, 0x72, 0x61, 0x6e, 0x73,
	0x66, 0x6f, 0x72, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1a, 0x0a, 0x16, 0x54, 0x52, 0x41, 0x4e,
	0x53, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f,
	0x57, 0x4e, 0x10, 0x00, 0x12, 0x20, 0x0a, 0x1c, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x46, 0x4f, 0x52,
	0x4d, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x52, 0x45, 0x50, 0x4c, 0x41, 0x43, 0x45, 0x5f, 0x56,
	0x41, 0x4c, 0x55, 0x45, 0x10, 0x01, 0x12, 0x1f, 0x0a, 0x1b, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x46,
	0x4f, 0x52, 0x4d, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x5f,
	0x46, 0x49, 0x45, 0x4c, 0x44, 0x10, 0x02, 0x12, 0x22, 0x0a, 0x1e, 0x54, 0x52, 0x41, 0x4e, 0x53,
	0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x4f, 0x42, 0x46, 0x55, 0x53, 0x43,
	0x41, 0x54, 0x45, 0x5f, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x10, 0x03, 0x12, 0x1d, 0x0a, 0x19, 0x54,
	0x52, 0x41, 0x4e, 0x53, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x4d, 0x41,
	0x53, 0x4b, 0x5f, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x10, 0x04, 0x12, 0x21, 0x0a, 0x1d, 0x54, 0x52,
	0x41, 0x4e, 0x53, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x54, 0x52, 0x55,
	0x4e, 0x43, 0x41, 0x54, 0x45, 0x5f, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x10, 0x05, 0x2a, 0x88, 0x01,
	0x0a, 0x15, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x54, 0x72, 0x75, 0x6e, 0x63,
	0x61, 0x74, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x23, 0x0a, 0x1f, 0x54, 0x52, 0x41, 0x4e, 0x53,
	0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x54, 0x52, 0x55, 0x4e, 0x43, 0x41, 0x54, 0x45, 0x5f, 0x54, 0x59,
	0x50, 0x45, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x22, 0x0a, 0x1e,
	0x54, 0x52, 0x41, 0x4e, 0x53, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x54, 0x52, 0x55, 0x4e, 0x43, 0x41,
	0x54, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x4c, 0x45, 0x4e, 0x47, 0x54, 0x48, 0x10, 0x01,
	0x12, 0x26, 0x0a, 0x22, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x54, 0x52,
	0x55, 0x4e, 0x43, 0x41, 0x54, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x50, 0x45, 0x52, 0x43,
	0x45, 0x4e, 0x54, 0x41, 0x47, 0x45, 0x10, 0x02, 0x42, 0x42, 0x5a, 0x40, 0x67, 0x69, 0x74, 0x68,
	0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x64, 0x61, 0x6c,
	0x2f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x64, 0x61, 0x6c, 0x2f, 0x6c, 0x69, 0x62, 0x73, 0x2f,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x67, 0x6f, 0x2f,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f, 0x73, 0x74, 0x65, 0x70, 0x73, 0x62, 0x06, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_steps_sp_steps_transform_proto_rawDescOnce sync.Once
	file_steps_sp_steps_transform_proto_rawDescData = file_steps_sp_steps_transform_proto_rawDesc
)

func file_steps_sp_steps_transform_proto_rawDescGZIP() []byte {
	file_steps_sp_steps_transform_proto_rawDescOnce.Do(func() {
		file_steps_sp_steps_transform_proto_rawDescData = protoimpl.X.CompressGZIP(file_steps_sp_steps_transform_proto_rawDescData)
	})
	return file_steps_sp_steps_transform_proto_rawDescData
}

var file_steps_sp_steps_transform_proto_enumTypes = make([]protoimpl.EnumInfo, 2)
var file_steps_sp_steps_transform_proto_msgTypes = make([]protoimpl.MessageInfo, 6)
var file_steps_sp_steps_transform_proto_goTypes = []interface{}{
	(TransformType)(0),                // 0: protos.steps.TransformType
	(TransformTruncateType)(0),        // 1: protos.steps.TransformTruncateType
	(*TransformStep)(nil),             // 2: protos.steps.TransformStep
	(*TransformTruncateOptions)(nil),  // 3: protos.steps.TransformTruncateOptions
	(*TransformDeleteFieldStep)(nil),  // 4: protos.steps.TransformDeleteFieldStep
	(*TransformReplaceValueStep)(nil), // 5: protos.steps.TransformReplaceValueStep
	(*TransformObfuscateOptions)(nil), // 6: protos.steps.TransformObfuscateOptions
	(*TransformMaskOptions)(nil),      // 7: protos.steps.TransformMaskOptions
}
var file_steps_sp_steps_transform_proto_depIdxs = []int32{
	0, // 0: protos.steps.TransformStep.type:type_name -> protos.steps.TransformType
	5, // 1: protos.steps.TransformStep.replace_value_options:type_name -> protos.steps.TransformReplaceValueStep
	4, // 2: protos.steps.TransformStep.delete_field_options:type_name -> protos.steps.TransformDeleteFieldStep
	6, // 3: protos.steps.TransformStep.obfuscate_options:type_name -> protos.steps.TransformObfuscateOptions
	7, // 4: protos.steps.TransformStep.mask_options:type_name -> protos.steps.TransformMaskOptions
	3, // 5: protos.steps.TransformStep.truncate_options:type_name -> protos.steps.TransformTruncateOptions
	1, // 6: protos.steps.TransformTruncateOptions.type:type_name -> protos.steps.TransformTruncateType
	7, // [7:7] is the sub-list for method output_type
	7, // [7:7] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_steps_sp_steps_transform_proto_init() }
func file_steps_sp_steps_transform_proto_init() {
	if File_steps_sp_steps_transform_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_steps_sp_steps_transform_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TransformStep); i {
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
		file_steps_sp_steps_transform_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TransformTruncateOptions); i {
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
		file_steps_sp_steps_transform_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TransformDeleteFieldStep); i {
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
		file_steps_sp_steps_transform_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TransformReplaceValueStep); i {
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
		file_steps_sp_steps_transform_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TransformObfuscateOptions); i {
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
		file_steps_sp_steps_transform_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TransformMaskOptions); i {
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
	file_steps_sp_steps_transform_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*TransformStep_ReplaceValueOptions)(nil),
		(*TransformStep_DeleteFieldOptions)(nil),
		(*TransformStep_ObfuscateOptions)(nil),
		(*TransformStep_MaskOptions)(nil),
		(*TransformStep_TruncateOptions)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_steps_sp_steps_transform_proto_rawDesc,
			NumEnums:      2,
			NumMessages:   6,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_steps_sp_steps_transform_proto_goTypes,
		DependencyIndexes: file_steps_sp_steps_transform_proto_depIdxs,
		EnumInfos:         file_steps_sp_steps_transform_proto_enumTypes,
		MessageInfos:      file_steps_sp_steps_transform_proto_msgTypes,
	}.Build()
	File_steps_sp_steps_transform_proto = out.File
	file_steps_sp_steps_transform_proto_rawDesc = nil
	file_steps_sp_steps_transform_proto_goTypes = nil
	file_steps_sp_steps_transform_proto_depIdxs = nil
}
