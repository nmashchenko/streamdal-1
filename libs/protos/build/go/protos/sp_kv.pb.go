// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.32.0
// 	protoc        v3.21.12
// source: sp_kv.proto

package protos

import (
	shared "github.com/streamdal/streamdal/libs/protos/build/go/protos/shared"
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

// KVObject represents a single KV object used in protos.KVInstruction; this is
// constructed by server and broadcast out to other server nodes.
type KVObject struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Valid key regex: /^[a-zA-Z0-9_-:]+$/)
	Key string `protobuf:"bytes,1,opt,name=key,proto3" json:"key,omitempty"`
	// KV value
	Value []byte `protobuf:"bytes,2,opt,name=value,proto3" json:"value,omitempty"`
	// When was this object created
	CreatedAtUnixTsNanoUtc int64 `protobuf:"varint,3,opt,name=created_at_unix_ts_nano_utc,json=createdAtUnixTsNanoUtc,proto3" json:"created_at_unix_ts_nano_utc,omitempty"`
	// Last time the object was updated
	UpdatedAtUnixTsNanoUtc int64 `protobuf:"varint,4,opt,name=updated_at_unix_ts_nano_utc,json=updatedAtUnixTsNanoUtc,proto3" json:"updated_at_unix_ts_nano_utc,omitempty"`
}

func (x *KVObject) Reset() {
	*x = KVObject{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sp_kv_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *KVObject) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*KVObject) ProtoMessage() {}

func (x *KVObject) ProtoReflect() protoreflect.Message {
	mi := &file_sp_kv_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use KVObject.ProtoReflect.Descriptor instead.
func (*KVObject) Descriptor() ([]byte, []int) {
	return file_sp_kv_proto_rawDescGZIP(), []int{0}
}

func (x *KVObject) GetKey() string {
	if x != nil {
		return x.Key
	}
	return ""
}

func (x *KVObject) GetValue() []byte {
	if x != nil {
		return x.Value
	}
	return nil
}

func (x *KVObject) GetCreatedAtUnixTsNanoUtc() int64 {
	if x != nil {
		return x.CreatedAtUnixTsNanoUtc
	}
	return 0
}

func (x *KVObject) GetUpdatedAtUnixTsNanoUtc() int64 {
	if x != nil {
		return x.UpdatedAtUnixTsNanoUtc
	}
	return 0
}

// Container for one or more KVObject's; server broadcasts KVCommand that
// contains one or more of these instructions when a "POST /api/v1/kv" request
// is made.
type KVInstruction struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Unique ID for this instruction
	Id string `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	// What kind of an action is this?
	Action shared.KVAction `protobuf:"varint,2,opt,name=action,proto3,enum=protos.shared.KVAction" json:"action,omitempty"`
	// KV object
	Object *KVObject `protobuf:"bytes,3,opt,name=object,proto3" json:"object,omitempty"`
	// When this instruction was requested (usually will be the HTTP API request time)
	RequestedAtUnixTsNanoUtc int64 `protobuf:"varint,4,opt,name=requested_at_unix_ts_nano_utc,json=requestedAtUnixTsNanoUtc,proto3" json:"requested_at_unix_ts_nano_utc,omitempty"`
}

func (x *KVInstruction) Reset() {
	*x = KVInstruction{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sp_kv_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *KVInstruction) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*KVInstruction) ProtoMessage() {}

func (x *KVInstruction) ProtoReflect() protoreflect.Message {
	mi := &file_sp_kv_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use KVInstruction.ProtoReflect.Descriptor instead.
func (*KVInstruction) Descriptor() ([]byte, []int) {
	return file_sp_kv_proto_rawDescGZIP(), []int{1}
}

func (x *KVInstruction) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *KVInstruction) GetAction() shared.KVAction {
	if x != nil {
		return x.Action
	}
	return shared.KVAction(0)
}

func (x *KVInstruction) GetObject() *KVObject {
	if x != nil {
		return x.Object
	}
	return nil
}

func (x *KVInstruction) GetRequestedAtUnixTsNanoUtc() int64 {
	if x != nil {
		return x.RequestedAtUnixTsNanoUtc
	}
	return 0
}

// Used for broadcasting KV instructions to other server nodes.
// NOTE: While this data structure is similar to KVCommand it makes sense to
// keep them separate. It would cause more confusion if we tried to re-use
// KVCommand for the purpose of broadcasting AND for sending SDK commands. ~DS
//
// This request structure is used for including all updates - create/update/delete.
type KVRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Instructions []*KVInstruction `protobuf:"bytes,1,rep,name=instructions,proto3" json:"instructions,omitempty"`
	Overwrite    bool             `protobuf:"varint,2,opt,name=overwrite,proto3" json:"overwrite,omitempty"`
}

func (x *KVRequest) Reset() {
	*x = KVRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sp_kv_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *KVRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*KVRequest) ProtoMessage() {}

func (x *KVRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sp_kv_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use KVRequest.ProtoReflect.Descriptor instead.
func (*KVRequest) Descriptor() ([]byte, []int) {
	return file_sp_kv_proto_rawDescGZIP(), []int{2}
}

func (x *KVRequest) GetInstructions() []*KVInstruction {
	if x != nil {
		return x.Instructions
	}
	return nil
}

func (x *KVRequest) GetOverwrite() bool {
	if x != nil {
		return x.Overwrite
	}
	return false
}

// "POST /api/v1/kv" accepts JSON of this type for it's request payload. This is
// converted by BroadcastKV() to a KVCommand
type KVCreateHTTPRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Kvs []*KVObject `protobuf:"bytes,1,rep,name=kvs,proto3" json:"kvs,omitempty"`
	// Whether to treat create as upsert -- ie. do not error if key already exists
	Overwrite bool `protobuf:"varint,2,opt,name=overwrite,proto3" json:"overwrite,omitempty"`
}

func (x *KVCreateHTTPRequest) Reset() {
	*x = KVCreateHTTPRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sp_kv_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *KVCreateHTTPRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*KVCreateHTTPRequest) ProtoMessage() {}

func (x *KVCreateHTTPRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sp_kv_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use KVCreateHTTPRequest.ProtoReflect.Descriptor instead.
func (*KVCreateHTTPRequest) Descriptor() ([]byte, []int) {
	return file_sp_kv_proto_rawDescGZIP(), []int{3}
}

func (x *KVCreateHTTPRequest) GetKvs() []*KVObject {
	if x != nil {
		return x.Kvs
	}
	return nil
}

func (x *KVCreateHTTPRequest) GetOverwrite() bool {
	if x != nil {
		return x.Overwrite
	}
	return false
}

type KVUpdateHTTPRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Kvs []*KVObject `protobuf:"bytes,1,rep,name=kvs,proto3" json:"kvs,omitempty"`
}

func (x *KVUpdateHTTPRequest) Reset() {
	*x = KVUpdateHTTPRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sp_kv_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *KVUpdateHTTPRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*KVUpdateHTTPRequest) ProtoMessage() {}

func (x *KVUpdateHTTPRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sp_kv_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use KVUpdateHTTPRequest.ProtoReflect.Descriptor instead.
func (*KVUpdateHTTPRequest) Descriptor() ([]byte, []int) {
	return file_sp_kv_proto_rawDescGZIP(), []int{4}
}

func (x *KVUpdateHTTPRequest) GetKvs() []*KVObject {
	if x != nil {
		return x.Kvs
	}
	return nil
}

var File_sp_kv_proto protoreflect.FileDescriptor

var file_sp_kv_proto_rawDesc = []byte{
	0x0a, 0x0b, 0x73, 0x70, 0x5f, 0x6b, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x73, 0x1a, 0x16, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x2f, 0x73, 0x70,
	0x5f, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xac, 0x01,
	0x0a, 0x08, 0x4b, 0x56, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65,
	0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05,
	0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c,
	0x75, 0x65, 0x12, 0x3b, 0x0a, 0x1b, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74,
	0x5f, 0x75, 0x6e, 0x69, 0x78, 0x5f, 0x74, 0x73, 0x5f, 0x6e, 0x61, 0x6e, 0x6f, 0x5f, 0x75, 0x74,
	0x63, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x16, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64,
	0x41, 0x74, 0x55, 0x6e, 0x69, 0x78, 0x54, 0x73, 0x4e, 0x61, 0x6e, 0x6f, 0x55, 0x74, 0x63, 0x12,
	0x3b, 0x0a, 0x1b, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x5f, 0x75, 0x6e,
	0x69, 0x78, 0x5f, 0x74, 0x73, 0x5f, 0x6e, 0x61, 0x6e, 0x6f, 0x5f, 0x75, 0x74, 0x63, 0x18, 0x04,
	0x20, 0x01, 0x28, 0x03, 0x52, 0x16, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x41, 0x74, 0x55,
	0x6e, 0x69, 0x78, 0x54, 0x73, 0x4e, 0x61, 0x6e, 0x6f, 0x55, 0x74, 0x63, 0x22, 0xbb, 0x01, 0x0a,
	0x0d, 0x4b, 0x56, 0x49, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0e,
	0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x2f,
	0x0a, 0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x2e, 0x4b,
	0x56, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12,
	0x28, 0x0a, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x10, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4b, 0x56, 0x4f, 0x62, 0x6a, 0x65, 0x63,
	0x74, 0x52, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x3f, 0x0a, 0x1d, 0x72, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x5f, 0x75, 0x6e, 0x69, 0x78, 0x5f, 0x74,
	0x73, 0x5f, 0x6e, 0x61, 0x6e, 0x6f, 0x5f, 0x75, 0x74, 0x63, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03,
	0x52, 0x18, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x41, 0x74, 0x55, 0x6e, 0x69,
	0x78, 0x54, 0x73, 0x4e, 0x61, 0x6e, 0x6f, 0x55, 0x74, 0x63, 0x22, 0x64, 0x0a, 0x09, 0x4b, 0x56,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x39, 0x0a, 0x0c, 0x69, 0x6e, 0x73, 0x74, 0x72,
	0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4b, 0x56, 0x49, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63,
	0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0c, 0x69, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x12, 0x1c, 0x0a, 0x09, 0x6f, 0x76, 0x65, 0x72, 0x77, 0x72, 0x69, 0x74, 0x65, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x6f, 0x76, 0x65, 0x72, 0x77, 0x72, 0x69, 0x74, 0x65,
	0x22, 0x57, 0x0a, 0x13, 0x4b, 0x56, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x48, 0x54, 0x54, 0x50,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x22, 0x0a, 0x03, 0x6b, 0x76, 0x73, 0x18, 0x01,
	0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4b, 0x56,
	0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x03, 0x6b, 0x76, 0x73, 0x12, 0x1c, 0x0a, 0x09, 0x6f,
	0x76, 0x65, 0x72, 0x77, 0x72, 0x69, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09,
	0x6f, 0x76, 0x65, 0x72, 0x77, 0x72, 0x69, 0x74, 0x65, 0x22, 0x39, 0x0a, 0x13, 0x4b, 0x56, 0x55,
	0x70, 0x64, 0x61, 0x74, 0x65, 0x48, 0x54, 0x54, 0x50, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
	0x12, 0x22, 0x0a, 0x03, 0x6b, 0x76, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4b, 0x56, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x52,
	0x03, 0x6b, 0x76, 0x73, 0x42, 0x48, 0x5a, 0x3a, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63,
	0x6f, 0x6d, 0x2f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x64, 0x61, 0x6c, 0x2f, 0x73, 0x74, 0x72,
	0x65, 0x61, 0x6d, 0x64, 0x61, 0x6c, 0x2f, 0x6c, 0x69, 0x62, 0x73, 0x2f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x73, 0x2f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x67, 0x6f, 0x2f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x73, 0xea, 0x02, 0x09, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x64, 0x61, 0x6c, 0x62, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_sp_kv_proto_rawDescOnce sync.Once
	file_sp_kv_proto_rawDescData = file_sp_kv_proto_rawDesc
)

func file_sp_kv_proto_rawDescGZIP() []byte {
	file_sp_kv_proto_rawDescOnce.Do(func() {
		file_sp_kv_proto_rawDescData = protoimpl.X.CompressGZIP(file_sp_kv_proto_rawDescData)
	})
	return file_sp_kv_proto_rawDescData
}

var file_sp_kv_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_sp_kv_proto_goTypes = []interface{}{
	(*KVObject)(nil),            // 0: protos.KVObject
	(*KVInstruction)(nil),       // 1: protos.KVInstruction
	(*KVRequest)(nil),           // 2: protos.KVRequest
	(*KVCreateHTTPRequest)(nil), // 3: protos.KVCreateHTTPRequest
	(*KVUpdateHTTPRequest)(nil), // 4: protos.KVUpdateHTTPRequest
	(shared.KVAction)(0),        // 5: protos.shared.KVAction
}
var file_sp_kv_proto_depIdxs = []int32{
	5, // 0: protos.KVInstruction.action:type_name -> protos.shared.KVAction
	0, // 1: protos.KVInstruction.object:type_name -> protos.KVObject
	1, // 2: protos.KVRequest.instructions:type_name -> protos.KVInstruction
	0, // 3: protos.KVCreateHTTPRequest.kvs:type_name -> protos.KVObject
	0, // 4: protos.KVUpdateHTTPRequest.kvs:type_name -> protos.KVObject
	5, // [5:5] is the sub-list for method output_type
	5, // [5:5] is the sub-list for method input_type
	5, // [5:5] is the sub-list for extension type_name
	5, // [5:5] is the sub-list for extension extendee
	0, // [0:5] is the sub-list for field type_name
}

func init() { file_sp_kv_proto_init() }
func file_sp_kv_proto_init() {
	if File_sp_kv_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sp_kv_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*KVObject); i {
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
		file_sp_kv_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*KVInstruction); i {
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
		file_sp_kv_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*KVRequest); i {
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
		file_sp_kv_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*KVCreateHTTPRequest); i {
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
		file_sp_kv_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*KVUpdateHTTPRequest); i {
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
			RawDescriptor: file_sp_kv_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_sp_kv_proto_goTypes,
		DependencyIndexes: file_sp_kv_proto_depIdxs,
		MessageInfos:      file_sp_kv_proto_msgTypes,
	}.Build()
	File_sp_kv_proto = out.File
	file_sp_kv_proto_rawDesc = nil
	file_sp_kv_proto_goTypes = nil
	file_sp_kv_proto_depIdxs = nil
}
