// This has to be a separate pkg to avoid circular import problems with Go.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        v3.21.12
// source: shared/sp_shared.proto

package shared

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

// KVAction is a shared type that is used for protos.KVCommand and protos.KVStep.
// Note that only a subset of actions are used for protos.KVCommand (CREATE,
// UPDATE, DELETE, DELETE_ALL) while protos.KVStep uses most of them.
//
// protolint:disable:next ENUM_FIELD_NAMES_PREFIX
type KVAction int32

const (
	KVAction_KV_ACTION_UNSET      KVAction = 0
	KVAction_KV_ACTION_GET        KVAction = 1
	KVAction_KV_ACTION_CREATE     KVAction = 2
	KVAction_KV_ACTION_UPDATE     KVAction = 3
	KVAction_KV_ACTION_EXISTS     KVAction = 4
	KVAction_KV_ACTION_DELETE     KVAction = 5
	KVAction_KV_ACTION_DELETE_ALL KVAction = 6
)

// Enum value maps for KVAction.
var (
	KVAction_name = map[int32]string{
		0: "KV_ACTION_UNSET",
		1: "KV_ACTION_GET",
		2: "KV_ACTION_CREATE",
		3: "KV_ACTION_UPDATE",
		4: "KV_ACTION_EXISTS",
		5: "KV_ACTION_DELETE",
		6: "KV_ACTION_DELETE_ALL",
	}
	KVAction_value = map[string]int32{
		"KV_ACTION_UNSET":      0,
		"KV_ACTION_GET":        1,
		"KV_ACTION_CREATE":     2,
		"KV_ACTION_UPDATE":     3,
		"KV_ACTION_EXISTS":     4,
		"KV_ACTION_DELETE":     5,
		"KV_ACTION_DELETE_ALL": 6,
	}
)

func (x KVAction) Enum() *KVAction {
	p := new(KVAction)
	*p = x
	return p
}

func (x KVAction) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (KVAction) Descriptor() protoreflect.EnumDescriptor {
	return file_shared_sp_shared_proto_enumTypes[0].Descriptor()
}

func (KVAction) Type() protoreflect.EnumType {
	return &file_shared_sp_shared_proto_enumTypes[0]
}

func (x KVAction) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use KVAction.Descriptor instead.
func (KVAction) EnumDescriptor() ([]byte, []int) {
	return file_shared_sp_shared_proto_rawDescGZIP(), []int{0}
}

var File_shared_sp_shared_proto protoreflect.FileDescriptor

var file_shared_sp_shared_proto_rawDesc = []byte{
	0x0a, 0x16, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x2f, 0x73, 0x70, 0x5f, 0x73, 0x68, 0x61, 0x72,
	0x65, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73,
	0x2e, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x2a, 0xa4, 0x01, 0x0a, 0x08, 0x4b, 0x56, 0x41, 0x63,
	0x74, 0x69, 0x6f, 0x6e, 0x12, 0x13, 0x0a, 0x0f, 0x4b, 0x56, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f,
	0x4e, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x11, 0x0a, 0x0d, 0x4b, 0x56, 0x5f,
	0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x47, 0x45, 0x54, 0x10, 0x01, 0x12, 0x14, 0x0a, 0x10,
	0x4b, 0x56, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45,
	0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x4b, 0x56, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f,
	0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x10, 0x03, 0x12, 0x14, 0x0a, 0x10, 0x4b, 0x56, 0x5f, 0x41,
	0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x45, 0x58, 0x49, 0x53, 0x54, 0x53, 0x10, 0x04, 0x12, 0x14,
	0x0a, 0x10, 0x4b, 0x56, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x44, 0x45, 0x4c, 0x45,
	0x54, 0x45, 0x10, 0x05, 0x12, 0x18, 0x0a, 0x14, 0x4b, 0x56, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f,
	0x4e, 0x5f, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x5f, 0x41, 0x4c, 0x4c, 0x10, 0x06, 0x42, 0x3e,
	0x5a, 0x3c, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x74, 0x72,
	0x65, 0x61, 0x6d, 0x64, 0x61, 0x6c, 0x2f, 0x6d, 0x6f, 0x6e, 0x6f, 0x2f, 0x6c, 0x69, 0x62, 0x73,
	0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x67, 0x6f,
	0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x62, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_shared_sp_shared_proto_rawDescOnce sync.Once
	file_shared_sp_shared_proto_rawDescData = file_shared_sp_shared_proto_rawDesc
)

func file_shared_sp_shared_proto_rawDescGZIP() []byte {
	file_shared_sp_shared_proto_rawDescOnce.Do(func() {
		file_shared_sp_shared_proto_rawDescData = protoimpl.X.CompressGZIP(file_shared_sp_shared_proto_rawDescData)
	})
	return file_shared_sp_shared_proto_rawDescData
}

var file_shared_sp_shared_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_shared_sp_shared_proto_goTypes = []interface{}{
	(KVAction)(0), // 0: protos.shared.KVAction
}
var file_shared_sp_shared_proto_depIdxs = []int32{
	0, // [0:0] is the sub-list for method output_type
	0, // [0:0] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_shared_sp_shared_proto_init() }
func file_shared_sp_shared_proto_init() {
	if File_shared_sp_shared_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_shared_sp_shared_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   0,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_shared_sp_shared_proto_goTypes,
		DependencyIndexes: file_shared_sp_shared_proto_depIdxs,
		EnumInfos:         file_shared_sp_shared_proto_enumTypes,
	}.Build()
	File_shared_sp_shared_proto = out.File
	file_shared_sp_shared_proto_rawDesc = nil
	file_shared_sp_shared_proto_goTypes = nil
	file_shared_sp_shared_proto_depIdxs = nil
}
