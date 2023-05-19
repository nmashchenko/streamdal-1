// ps_common_backends.proto contains the list of all supported / available backends.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        v3.21.6
// source: ps_common_backends.proto

package common

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

type BackendType int32

const (
	BackendType_BACKEND_TYPE_UNSET             BackendType = 0
	BackendType_BACKEND_TYPE_KAFKA             BackendType = 1
	BackendType_BACKEND_TYPE_RABBIT            BackendType = 2
	BackendType_BACKEND_TYPE_RABBIT_STREAMS    BackendType = 3
	BackendType_BACKEND_TYPE_NSQ               BackendType = 4
	BackendType_BACKEND_TYPE_NATS              BackendType = 5
	BackendType_BACKEND_TYPE_NATS_STREAMING    BackendType = 6
	BackendType_BACKEND_TYPE_GCP_PUBSUB        BackendType = 7
	BackendType_BACKEND_TYPE_AZURE_SERVICE_BUS BackendType = 8
	BackendType_BACKEND_TYPE_AZURE_EVENT_HUB   BackendType = 9
	BackendType_BACKEND_TYPE_AWS_SQS           BackendType = 10
	BackendType_BACKEND_TYPE_AWS_SNS           BackendType = 11
	BackendType_BACKEND_TYPE_REDIS_PUBSUB      BackendType = 12
	BackendType_BACKEND_TYPE_REDIS_STREAMS     BackendType = 13
	BackendType_BACKEND_TYPE_ACTIVEMQ          BackendType = 14
	BackendType_BACKEND_TYPE_PULSAR            BackendType = 15
	BackendType_BACKEND_TYPE_MQTT              BackendType = 16
	BackendType_BACKEND_TYPE_POSTGRES_CDC      BackendType = 17
	BackendType_BACKEND_TYPE_MONGODB_CDC       BackendType = 18
	BackendType_BACKEND_TYPE_KUBE_MQ           BackendType = 19
	BackendType_BACKEND_TYPE_AWS_KINESIS       BackendType = 20
)

// Enum value maps for BackendType.
var (
	BackendType_name = map[int32]string{
		0:  "BACKEND_TYPE_UNSET",
		1:  "BACKEND_TYPE_KAFKA",
		2:  "BACKEND_TYPE_RABBIT",
		3:  "BACKEND_TYPE_RABBIT_STREAMS",
		4:  "BACKEND_TYPE_NSQ",
		5:  "BACKEND_TYPE_NATS",
		6:  "BACKEND_TYPE_NATS_STREAMING",
		7:  "BACKEND_TYPE_GCP_PUBSUB",
		8:  "BACKEND_TYPE_AZURE_SERVICE_BUS",
		9:  "BACKEND_TYPE_AZURE_EVENT_HUB",
		10: "BACKEND_TYPE_AWS_SQS",
		11: "BACKEND_TYPE_AWS_SNS",
		12: "BACKEND_TYPE_REDIS_PUBSUB",
		13: "BACKEND_TYPE_REDIS_STREAMS",
		14: "BACKEND_TYPE_ACTIVEMQ",
		15: "BACKEND_TYPE_PULSAR",
		16: "BACKEND_TYPE_MQTT",
		17: "BACKEND_TYPE_POSTGRES_CDC",
		18: "BACKEND_TYPE_MONGODB_CDC",
		19: "BACKEND_TYPE_KUBE_MQ",
		20: "BACKEND_TYPE_AWS_KINESIS",
	}
	BackendType_value = map[string]int32{
		"BACKEND_TYPE_UNSET":             0,
		"BACKEND_TYPE_KAFKA":             1,
		"BACKEND_TYPE_RABBIT":            2,
		"BACKEND_TYPE_RABBIT_STREAMS":    3,
		"BACKEND_TYPE_NSQ":               4,
		"BACKEND_TYPE_NATS":              5,
		"BACKEND_TYPE_NATS_STREAMING":    6,
		"BACKEND_TYPE_GCP_PUBSUB":        7,
		"BACKEND_TYPE_AZURE_SERVICE_BUS": 8,
		"BACKEND_TYPE_AZURE_EVENT_HUB":   9,
		"BACKEND_TYPE_AWS_SQS":           10,
		"BACKEND_TYPE_AWS_SNS":           11,
		"BACKEND_TYPE_REDIS_PUBSUB":      12,
		"BACKEND_TYPE_REDIS_STREAMS":     13,
		"BACKEND_TYPE_ACTIVEMQ":          14,
		"BACKEND_TYPE_PULSAR":            15,
		"BACKEND_TYPE_MQTT":              16,
		"BACKEND_TYPE_POSTGRES_CDC":      17,
		"BACKEND_TYPE_MONGODB_CDC":       18,
		"BACKEND_TYPE_KUBE_MQ":           19,
		"BACKEND_TYPE_AWS_KINESIS":       20,
	}
)

func (x BackendType) Enum() *BackendType {
	p := new(BackendType)
	*p = x
	return p
}

func (x BackendType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (BackendType) Descriptor() protoreflect.EnumDescriptor {
	return file_ps_common_backends_proto_enumTypes[0].Descriptor()
}

func (BackendType) Type() protoreflect.EnumType {
	return &file_ps_common_backends_proto_enumTypes[0]
}

func (x BackendType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use BackendType.Descriptor instead.
func (BackendType) EnumDescriptor() ([]byte, []int) {
	return file_ps_common_backends_proto_rawDescGZIP(), []int{0}
}

var File_ps_common_backends_proto protoreflect.FileDescriptor

var file_ps_common_backends_proto_rawDesc = []byte{
	0x0a, 0x18, 0x70, 0x73, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x62, 0x61, 0x63, 0x6b,
	0x65, 0x6e, 0x64, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2a, 0xdb, 0x04, 0x0a, 0x0b, 0x42, 0x61,
	0x63, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x12, 0x42, 0x41, 0x43,
	0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10,
	0x00, 0x12, 0x16, 0x0a, 0x12, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50,
	0x45, 0x5f, 0x4b, 0x41, 0x46, 0x4b, 0x41, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x42, 0x41, 0x43,
	0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x52, 0x41, 0x42, 0x42, 0x49, 0x54,
	0x10, 0x02, 0x12, 0x1f, 0x0a, 0x1b, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59,
	0x50, 0x45, 0x5f, 0x52, 0x41, 0x42, 0x42, 0x49, 0x54, 0x5f, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4d,
	0x53, 0x10, 0x03, 0x12, 0x14, 0x0a, 0x10, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54,
	0x59, 0x50, 0x45, 0x5f, 0x4e, 0x53, 0x51, 0x10, 0x04, 0x12, 0x15, 0x0a, 0x11, 0x42, 0x41, 0x43,
	0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x4e, 0x41, 0x54, 0x53, 0x10, 0x05,
	0x12, 0x1f, 0x0a, 0x1b, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45,
	0x5f, 0x4e, 0x41, 0x54, 0x53, 0x5f, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4d, 0x49, 0x4e, 0x47, 0x10,
	0x06, 0x12, 0x1b, 0x0a, 0x17, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50,
	0x45, 0x5f, 0x47, 0x43, 0x50, 0x5f, 0x50, 0x55, 0x42, 0x53, 0x55, 0x42, 0x10, 0x07, 0x12, 0x22,
	0x0a, 0x1e, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x41,
	0x5a, 0x55, 0x52, 0x45, 0x5f, 0x53, 0x45, 0x52, 0x56, 0x49, 0x43, 0x45, 0x5f, 0x42, 0x55, 0x53,
	0x10, 0x08, 0x12, 0x20, 0x0a, 0x1c, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59,
	0x50, 0x45, 0x5f, 0x41, 0x5a, 0x55, 0x52, 0x45, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x48,
	0x55, 0x42, 0x10, 0x09, 0x12, 0x18, 0x0a, 0x14, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f,
	0x54, 0x59, 0x50, 0x45, 0x5f, 0x41, 0x57, 0x53, 0x5f, 0x53, 0x51, 0x53, 0x10, 0x0a, 0x12, 0x18,
	0x0a, 0x14, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x41,
	0x57, 0x53, 0x5f, 0x53, 0x4e, 0x53, 0x10, 0x0b, 0x12, 0x1d, 0x0a, 0x19, 0x42, 0x41, 0x43, 0x4b,
	0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x52, 0x45, 0x44, 0x49, 0x53, 0x5f, 0x50,
	0x55, 0x42, 0x53, 0x55, 0x42, 0x10, 0x0c, 0x12, 0x1e, 0x0a, 0x1a, 0x42, 0x41, 0x43, 0x4b, 0x45,
	0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x52, 0x45, 0x44, 0x49, 0x53, 0x5f, 0x53, 0x54,
	0x52, 0x45, 0x41, 0x4d, 0x53, 0x10, 0x0d, 0x12, 0x19, 0x0a, 0x15, 0x42, 0x41, 0x43, 0x4b, 0x45,
	0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x56, 0x45, 0x4d, 0x51,
	0x10, 0x0e, 0x12, 0x17, 0x0a, 0x13, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59,
	0x50, 0x45, 0x5f, 0x50, 0x55, 0x4c, 0x53, 0x41, 0x52, 0x10, 0x0f, 0x12, 0x15, 0x0a, 0x11, 0x42,
	0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x4d, 0x51, 0x54, 0x54,
	0x10, 0x10, 0x12, 0x1d, 0x0a, 0x19, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59,
	0x50, 0x45, 0x5f, 0x50, 0x4f, 0x53, 0x54, 0x47, 0x52, 0x45, 0x53, 0x5f, 0x43, 0x44, 0x43, 0x10,
	0x11, 0x12, 0x1c, 0x0a, 0x18, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50,
	0x45, 0x5f, 0x4d, 0x4f, 0x4e, 0x47, 0x4f, 0x44, 0x42, 0x5f, 0x43, 0x44, 0x43, 0x10, 0x12, 0x12,
	0x18, 0x0a, 0x14, 0x42, 0x41, 0x43, 0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f,
	0x4b, 0x55, 0x42, 0x45, 0x5f, 0x4d, 0x51, 0x10, 0x13, 0x12, 0x1c, 0x0a, 0x18, 0x42, 0x41, 0x43,
	0x4b, 0x45, 0x4e, 0x44, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x41, 0x57, 0x53, 0x5f, 0x4b, 0x49,
	0x4e, 0x45, 0x53, 0x49, 0x53, 0x10, 0x14, 0x42, 0x3d, 0x5a, 0x3b, 0x67, 0x69, 0x74, 0x68, 0x75,
	0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x63, 0x6f, 0x72, 0x70, 0x2f,
	0x70, 0x6c, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x2d, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x73, 0x2f,
	0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x67, 0x6f, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f,
	0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_ps_common_backends_proto_rawDescOnce sync.Once
	file_ps_common_backends_proto_rawDescData = file_ps_common_backends_proto_rawDesc
)

func file_ps_common_backends_proto_rawDescGZIP() []byte {
	file_ps_common_backends_proto_rawDescOnce.Do(func() {
		file_ps_common_backends_proto_rawDescData = protoimpl.X.CompressGZIP(file_ps_common_backends_proto_rawDescData)
	})
	return file_ps_common_backends_proto_rawDescData
}

var file_ps_common_backends_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_ps_common_backends_proto_goTypes = []interface{}{
	(BackendType)(0), // 0: protos.common.BackendType
}
var file_ps_common_backends_proto_depIdxs = []int32{
	0, // [0:0] is the sub-list for method output_type
	0, // [0:0] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_ps_common_backends_proto_init() }
func file_ps_common_backends_proto_init() {
	if File_ps_common_backends_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_ps_common_backends_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   0,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_ps_common_backends_proto_goTypes,
		DependencyIndexes: file_ps_common_backends_proto_depIdxs,
		EnumInfos:         file_ps_common_backends_proto_enumTypes,
	}.Build()
	File_ps_common_backends_proto = out.File
	file_ps_common_backends_proto_rawDesc = nil
	file_ps_common_backends_proto_goTypes = nil
	file_ps_common_backends_proto_depIdxs = nil
}
