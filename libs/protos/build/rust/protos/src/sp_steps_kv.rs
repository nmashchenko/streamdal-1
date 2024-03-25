// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `steps/sp_steps_kv.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

///  Returned by SDK host func and interpreted by KV WASM.
// @@protoc_insertion_point(message:protos.steps.KVStepResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KVStepResponse {
    // message fields
    ///  Status of the action; interpreted by KV WASM to so it can generate a protos.WASMResponse
    // @@protoc_insertion_point(field:protos.steps.KVStepResponse.status)
    pub status: ::protobuf::EnumOrUnknown<KVStatus>,
    ///  Message containing info, debug or error details; included in protos.WASMResponse
    // @@protoc_insertion_point(field:protos.steps.KVStepResponse.message)
    pub message: ::std::string::String,
    ///  Optional because the only action that uses field is KV_ACTION_GET
    ///
    ///  DS: Not sure how we'll use KV_ACTION_GET in steps yet but this is probably
    ///  a good place to start. 09.06.2023.
    // @@protoc_insertion_point(field:protos.steps.KVStepResponse.value)
    pub value: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:protos.steps.KVStepResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KVStepResponse {
    fn default() -> &'a KVStepResponse {
        <KVStepResponse as ::protobuf::Message>::default_instance()
    }
}

impl KVStepResponse {
    pub fn new() -> KVStepResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &KVStepResponse| { &m.status },
            |m: &mut KVStepResponse| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message",
            |m: &KVStepResponse| { &m.message },
            |m: &mut KVStepResponse| { &mut m.message },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "value",
            |m: &KVStepResponse| { &m.value },
            |m: &mut KVStepResponse| { &mut m.value },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KVStepResponse>(
            "KVStepResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KVStepResponse {
    const NAME: &'static str = "KVStepResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.message = is.read_string()?;
                },
                26 => {
                    self.value = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.status != ::protobuf::EnumOrUnknown::new(KVStatus::KV_STATUS_UNSET) {
            my_size += ::protobuf::rt::int32_size(1, self.status.value());
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.status != ::protobuf::EnumOrUnknown::new(KVStatus::KV_STATUS_UNSET) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        }
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(3, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> KVStepResponse {
        KVStepResponse::new()
    }

    fn clear(&mut self) {
        self.status = ::protobuf::EnumOrUnknown::new(KVStatus::KV_STATUS_UNSET);
        self.message.clear();
        self.value = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KVStepResponse {
        static instance: KVStepResponse = KVStepResponse {
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            message: ::std::string::String::new(),
            value: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KVStepResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KVStepResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KVStepResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KVStepResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  Used in PipelineSteps and passed to KV host func; constructed by frontend
// @@protoc_insertion_point(message:protos.steps.KVStep)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KVStep {
    // message fields
    ///  What type of action this step should perform
    // @@protoc_insertion_point(field:protos.steps.KVStep.action)
    pub action: ::protobuf::EnumOrUnknown<super::sp_shared::KVAction>,
    ///  How the key field will be used to perform lookup
    // @@protoc_insertion_point(field:protos.steps.KVStep.mode)
    pub mode: ::protobuf::EnumOrUnknown<KVMode>,
    ///  The key the action is taking place on
    // @@protoc_insertion_point(field:protos.steps.KVStep.key)
    pub key: ::std::string::String,
    ///  Optional because the only action that needs value is KV_ACTION_CREATE
    // @@protoc_insertion_point(field:protos.steps.KVStep.value)
    pub value: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:protos.steps.KVStep.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KVStep {
    fn default() -> &'a KVStep {
        <KVStep as ::protobuf::Message>::default_instance()
    }
}

impl KVStep {
    pub fn new() -> KVStep {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "action",
            |m: &KVStep| { &m.action },
            |m: &mut KVStep| { &mut m.action },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mode",
            |m: &KVStep| { &m.mode },
            |m: &mut KVStep| { &mut m.mode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "key",
            |m: &KVStep| { &m.key },
            |m: &mut KVStep| { &mut m.key },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "value",
            |m: &KVStep| { &m.value },
            |m: &mut KVStep| { &mut m.value },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KVStep>(
            "KVStep",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KVStep {
    const NAME: &'static str = "KVStep";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.action = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.mode = is.read_enum_or_unknown()?;
                },
                26 => {
                    self.key = is.read_string()?;
                },
                34 => {
                    self.value = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.action != ::protobuf::EnumOrUnknown::new(super::sp_shared::KVAction::KV_ACTION_UNSET) {
            my_size += ::protobuf::rt::int32_size(1, self.action.value());
        }
        if self.mode != ::protobuf::EnumOrUnknown::new(KVMode::KV_MODE_UNSET) {
            my_size += ::protobuf::rt::int32_size(2, self.mode.value());
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.key);
        }
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.action != ::protobuf::EnumOrUnknown::new(super::sp_shared::KVAction::KV_ACTION_UNSET) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.action))?;
        }
        if self.mode != ::protobuf::EnumOrUnknown::new(KVMode::KV_MODE_UNSET) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.mode))?;
        }
        if !self.key.is_empty() {
            os.write_string(3, &self.key)?;
        }
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(4, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> KVStep {
        KVStep::new()
    }

    fn clear(&mut self) {
        self.action = ::protobuf::EnumOrUnknown::new(super::sp_shared::KVAction::KV_ACTION_UNSET);
        self.mode = ::protobuf::EnumOrUnknown::new(KVMode::KV_MODE_UNSET);
        self.key.clear();
        self.value = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KVStep {
        static instance: KVStep = KVStep {
            action: ::protobuf::EnumOrUnknown::from_i32(0),
            mode: ::protobuf::EnumOrUnknown::from_i32(0),
            key: ::std::string::String::new(),
            value: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KVStep {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KVStep").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KVStep {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KVStep {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  Used by frontend when constructing a pipeline that contains a KV step that
///  performs any KV request. The mode determines _what_ the contents of the
///  key will be. Read comments about "static" vs "dynamic".
///  protolint:disable:next ENUM_FIELD_NAMES_PREFIX
#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:protos.steps.KVMode)
pub enum KVMode {
    // @@protoc_insertion_point(enum_value:protos.steps.KVMode.KV_MODE_UNSET)
    KV_MODE_UNSET = 0,
    // @@protoc_insertion_point(enum_value:protos.steps.KVMode.KV_MODE_STATIC)
    KV_MODE_STATIC = 1,
    // @@protoc_insertion_point(enum_value:protos.steps.KVMode.KV_MODE_DYNAMIC)
    KV_MODE_DYNAMIC = 2,
}

impl ::protobuf::Enum for KVMode {
    const NAME: &'static str = "KVMode";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KVMode> {
        match value {
            0 => ::std::option::Option::Some(KVMode::KV_MODE_UNSET),
            1 => ::std::option::Option::Some(KVMode::KV_MODE_STATIC),
            2 => ::std::option::Option::Some(KVMode::KV_MODE_DYNAMIC),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<KVMode> {
        match str {
            "KV_MODE_UNSET" => ::std::option::Option::Some(KVMode::KV_MODE_UNSET),
            "KV_MODE_STATIC" => ::std::option::Option::Some(KVMode::KV_MODE_STATIC),
            "KV_MODE_DYNAMIC" => ::std::option::Option::Some(KVMode::KV_MODE_DYNAMIC),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [KVMode] = &[
        KVMode::KV_MODE_UNSET,
        KVMode::KV_MODE_STATIC,
        KVMode::KV_MODE_DYNAMIC,
    ];
}

impl ::protobuf::EnumFull for KVMode {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("KVMode").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for KVMode {
    fn default() -> Self {
        KVMode::KV_MODE_UNSET
    }
}

impl KVMode {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<KVMode>("KVMode")
    }
}

///  Returned by KV host func and interpreted by KV WASM.
///  protolint:disable:next ENUM_FIELD_NAMES_PREFIX
#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:protos.steps.KVStatus)
pub enum KVStatus {
    // @@protoc_insertion_point(enum_value:protos.steps.KVStatus.KV_STATUS_UNSET)
    KV_STATUS_UNSET = 0,
    // @@protoc_insertion_point(enum_value:protos.steps.KVStatus.KV_STATUS_SUCCESS)
    KV_STATUS_SUCCESS = 1,
    // @@protoc_insertion_point(enum_value:protos.steps.KVStatus.KV_STATUS_FAILURE)
    KV_STATUS_FAILURE = 2,
    // @@protoc_insertion_point(enum_value:protos.steps.KVStatus.KV_STATUS_ERROR)
    KV_STATUS_ERROR = 3,
}

impl ::protobuf::Enum for KVStatus {
    const NAME: &'static str = "KVStatus";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KVStatus> {
        match value {
            0 => ::std::option::Option::Some(KVStatus::KV_STATUS_UNSET),
            1 => ::std::option::Option::Some(KVStatus::KV_STATUS_SUCCESS),
            2 => ::std::option::Option::Some(KVStatus::KV_STATUS_FAILURE),
            3 => ::std::option::Option::Some(KVStatus::KV_STATUS_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<KVStatus> {
        match str {
            "KV_STATUS_UNSET" => ::std::option::Option::Some(KVStatus::KV_STATUS_UNSET),
            "KV_STATUS_SUCCESS" => ::std::option::Option::Some(KVStatus::KV_STATUS_SUCCESS),
            "KV_STATUS_FAILURE" => ::std::option::Option::Some(KVStatus::KV_STATUS_FAILURE),
            "KV_STATUS_ERROR" => ::std::option::Option::Some(KVStatus::KV_STATUS_ERROR),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [KVStatus] = &[
        KVStatus::KV_STATUS_UNSET,
        KVStatus::KV_STATUS_SUCCESS,
        KVStatus::KV_STATUS_FAILURE,
        KVStatus::KV_STATUS_ERROR,
    ];
}

impl ::protobuf::EnumFull for KVStatus {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("KVStatus").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for KVStatus {
    fn default() -> Self {
        KVStatus::KV_STATUS_UNSET
    }
}

impl KVStatus {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<KVStatus>("KVStatus")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17steps/sp_steps_kv.proto\x12\x0cprotos.steps\x1a\x16shared/sp_share\
    d.proto\"\x7f\n\x0eKVStepResponse\x12.\n\x06status\x18\x01\x20\x01(\x0e2\
    \x16.protos.steps.KVStatusR\x06status\x12\x18\n\x07message\x18\x02\x20\
    \x01(\tR\x07message\x12\x19\n\x05value\x18\x03\x20\x01(\x0cH\0R\x05value\
    \x88\x01\x01B\x08\n\x06_value\"\x9a\x01\n\x06KVStep\x12/\n\x06action\x18\
    \x01\x20\x01(\x0e2\x17.protos.shared.KVActionR\x06action\x12(\n\x04mode\
    \x18\x02\x20\x01(\x0e2\x14.protos.steps.KVModeR\x04mode\x12\x10\n\x03key\
    \x18\x03\x20\x01(\tR\x03key\x12\x19\n\x05value\x18\x04\x20\x01(\x0cH\0R\
    \x05value\x88\x01\x01B\x08\n\x06_value*D\n\x06KVMode\x12\x11\n\rKV_MODE_\
    UNSET\x10\0\x12\x12\n\x0eKV_MODE_STATIC\x10\x01\x12\x13\n\x0fKV_MODE_DYN\
    AMIC\x10\x02*b\n\x08KVStatus\x12\x13\n\x0fKV_STATUS_UNSET\x10\0\x12\x15\
    \n\x11KV_STATUS_SUCCESS\x10\x01\x12\x15\n\x11KV_STATUS_FAILURE\x10\x02\
    \x12\x13\n\x0fKV_STATUS_ERROR\x10\x03BNZ@github.com/streamdal/streamdal/\
    libs/protos/build/go/protos/steps\xea\x02\tStreamdalJ\x93\x1b\n\x06\x12\
    \x04\0\0[\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x02\0\x15\nE\n\x02\x03\0\x12\x03\x05\0\x20\x1a:\x20We\x20are\x20importi\
    ng\x20this\x20to\x20get\x20access\x20to\x20the\x20KVAction\x20type\n\n\
    \x08\n\x01\x08\x12\x03\x07\0W\n\t\n\x02\x08\x0b\x12\x03\x07\0W\n\x08\n\
    \x01\x08\x12\x03\x08\0\"\n\t\n\x02\x08-\x12\x03\x08\0\"\n\xa7\n\n\x02\
    \x05\0\x12\x04%\04\x01\x1a\xfe\x01\x20Used\x20by\x20frontend\x20when\x20\
    constructing\x20a\x20pipeline\x20that\x20contains\x20a\x20KV\x20step\x20\
    that\n\x20performs\x20any\x20KV\x20request.\x20The\x20mode\x20determines\
    \x20_what_\x20the\x20contents\x20of\x20the\n\x20key\x20will\x20be.\x20Re\
    ad\x20comments\x20about\x20\"static\"\x20vs\x20\"dynamic\".\n\x20protoli\
    nt:disable:next\x20ENUM_FIELD_NAMES_PREFIX\n2\x99\x08\x20!!!!!!!!!!!!!!!\
    !!!!!!!!!!!!\x20IMPORTANT\x20!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\n\x20KV\
    \x20consists\x20of\x20two\x20parts:\n\n\x20-\x20KVStep\n\x20-\x20KVStepR\
    esponse\n\n\x20KVStep\x20is\x20used\x20in\x20PipelineSteps\x20that\x20wi\
    ll\x20execute\x20a\x20specific\x20KV\x20request;\n\x20the\x20actual\x20K\
    V\x20lookup\x20is\x20performed\x20by\x20the\x20KV\x20WASM\x20func\x20tha\
    t\x20calls\x20out\x20to\n\x20HostFuncKVExists()\x20that\x20is\x20a\x20fu\
    nction\x20exported\x20by\x20the\x20SDK.\n\n\x20The\x20HostFuncKVExists()\
    \x20function\x20is\x20needed\x20because\x20as\x20of\x2008.30.2023,\x20WA\
    SM\x20does\n\x20not\x20have\x20socket\x20support,\x20so\x20we\x20need\
    \x20to\x20call\x20out\x20to\x20the\x20SDK\x20to\x20perform\x20the\n\x20a\
    ctual\x20KV\x20API\x20call.\n\n\x20NOTE:\x20The\x20KV\x20host\x20funcs\
    \x20accept\x20a\x20special\x20request\x20type\x20but\x20return\x20a\x20g\
    eneric\n\x20response.\x20This\x20is\x20done\x20so\x20that\x20we\x20can\
    \x20include\x20custom\x20request\x20params\x20that\n\x20might\x20only\
    \x20be\x20relevant\x20to\x20that\x20specific\x20KV\x20func\x20while\x20t\
    he\x20response\x20will\n\x20contain\x20fields\x20that\x20are\x20common\
    \x20to\x20all\x20KV\x20funcs.\x20Ie.\x20KVExistsRequest\x20requires\n\
    \x20you\x20to\x20specify\x20the\x20lookup\x20mode\x20(which\x20would\x20\
    not\x20be\x20needed\x20for\x20something\x20like\n\x20a\x20KVGet\x20reque\
    st),\x20while\x20the\x20response\x20is\x20generally\x20the\x20same\x20-\
    \x20did\x20it\x20succeed?\n\x20did\x20it\x20fail?\x20was\x20there\x20an\
    \x20internal\x20error?\x20what\x20is\x20the\x20return\x20data\x20(if\x20\
    any)?\n\n\n\n\x03\x05\0\x01\x12\x03%\x05\x0b\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03&\x02\x14\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03&\x02\x0f\n\x0c\n\x05\
    \x05\0\x02\0\x02\x12\x03&\x12\x13\nR\n\x04\x05\0\x02\x01\x12\x03)\x02\
    \x15\x1aE\x20Will\x20cause\x20the\x20KV\x20lookup\x20to\x20use\x20the\
    \x20key\x20string\x20as-is\x20for\x20the\x20lookup\n\n\x0c\n\x05\x05\0\
    \x02\x01\x01\x12\x03)\x02\x10\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03)\x13\
    \x14\n\xe3\x02\n\x04\x05\0\x02\x02\x12\x033\x02\x16\x1a\xd5\x02\x20DYNAM\
    IC\x20mode\x20will\x20cause\x20the\x20KV\x20lookup\x20WASM\x20to\x20use\
    \x20the\x20key\x20to\x20lookup\x20the\n\x20associated\x20value\x20and\
    \x20use\x20the\x20result\x20for\x20the\x20key\x20existence\x20check.\n\n\
    \x20For\x20example,\x20if\x20\"key\"\x20in\x20KVHostFuncRequest\x20is\
    \x20set\x20to\x20\"foo\",\x20KV\x20WASM\x20will\x20do\n\x20the\x20follow\
    ing:\n\n\x201.\x20Lookup\x20the\x20value\x20of\x20\"foo\"\x20in\x20the\
    \x20payload\x20(which\x20is\x20\"bar\")\n\x202.\x20Use\x20\"bar\"\x20as\
    \x20the\x20\"key\"\x20for\x20the\x20KV\x20lookup\n\n\x0c\n\x05\x05\0\x02\
    \x02\x01\x12\x033\x02\x11\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x033\x14\x15\
    \nr\n\x02\x05\x01\x12\x048\0=\x01\x1af\x20Returned\x20by\x20KV\x20host\
    \x20func\x20and\x20interpreted\x20by\x20KV\x20WASM.\n\x20protolint:disab\
    le:next\x20ENUM_FIELD_NAMES_PREFIX\n\n\n\n\x03\x05\x01\x01\x12\x038\x05\
    \r\n\x0b\n\x04\x05\x01\x02\0\x12\x039\x02\x16\n\x0c\n\x05\x05\x01\x02\0\
    \x01\x12\x039\x02\x11\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x039\x14\x15\n\
    \x0b\n\x04\x05\x01\x02\x01\x12\x03:\x02\x18\n\x0c\n\x05\x05\x01\x02\x01\
    \x01\x12\x03:\x02\x13\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03:\x16\x17\n\
    \x0b\n\x04\x05\x01\x02\x02\x12\x03;\x02\x18\n\x0c\n\x05\x05\x01\x02\x02\
    \x01\x12\x03;\x02\x13\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03;\x16\x17\n\
    \x0b\n\x04\x05\x01\x02\x03\x12\x03<\x02\x16\n\x0c\n\x05\x05\x01\x02\x03\
    \x01\x12\x03<\x02\x11\n\x0c\n\x05\x05\x01\x02\x03\x02\x12\x03<\x14\x15\n\
    C\n\x02\x04\0\x12\x04@\0L\x01\x1a7\x20Returned\x20by\x20SDK\x20host\x20f\
    unc\x20and\x20interpreted\x20by\x20KV\x20WASM.\n\n\n\n\x03\x04\0\x01\x12\
    \x03@\x08\x16\ng\n\x04\x04\0\x02\0\x12\x03B\x02\x16\x1aZ\x20Status\x20of\
    \x20the\x20action;\x20interpreted\x20by\x20KV\x20WASM\x20to\x20so\x20it\
    \x20can\x20generate\x20a\x20protos.WASMResponse\n\n\x0c\n\x05\x04\0\x02\
    \0\x06\x12\x03B\x02\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03B\x0b\x11\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03B\x14\x15\n_\n\x04\x04\0\x02\x01\x12\x03E\
    \x02\x15\x1aR\x20Message\x20containing\x20info,\x20debug\x20or\x20error\
    \x20details;\x20included\x20in\x20protos.WASMResponse\n\n\x0c\n\x05\x04\
    \0\x02\x01\x05\x12\x03E\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03E\t\
    \x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03E\x13\x14\n\xc2\x01\n\x04\x04\
    \0\x02\x02\x12\x03K\x02\x1b\x1a\xb4\x01\x20Optional\x20because\x20the\
    \x20only\x20action\x20that\x20uses\x20field\x20is\x20KV_ACTION_GET\n\n\
    \x20DS:\x20Not\x20sure\x20how\x20we'll\x20use\x20KV_ACTION_GET\x20in\x20\
    steps\x20yet\x20but\x20this\x20is\x20probably\n\x20a\x20good\x20place\
    \x20to\x20start.\x2009.06.2023.\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03K\
    \x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03K\x0b\x10\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03K\x11\x16\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03K\x19\
    \x1a\nW\n\x02\x04\x01\x12\x04O\0[\x01\x1aK\x20Used\x20in\x20PipelineStep\
    s\x20and\x20passed\x20to\x20KV\x20host\x20func;\x20constructed\x20by\x20\
    frontend\n\n\n\n\x03\x04\x01\x01\x12\x03O\x08\x0e\n;\n\x04\x04\x01\x02\0\
    \x12\x03Q\x02$\x1a.\x20What\x20type\x20of\x20action\x20this\x20step\x20s\
    hould\x20perform\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03Q\x02\x18\n\x0c\
    \n\x05\x04\x01\x02\0\x01\x12\x03Q\x19\x1f\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03Q\"#\n?\n\x04\x04\x01\x02\x01\x12\x03T\x02\x12\x1a2\x20How\x20th\
    e\x20key\x20field\x20will\x20be\x20used\x20to\x20perform\x20lookup\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x06\x12\x03T\x02\x08\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03T\t\r\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03T\x10\x11\n\
    4\n\x04\x04\x01\x02\x02\x12\x03W\x02\x11\x1a'\x20The\x20key\x20the\x20ac\
    tion\x20is\x20taking\x20place\x20on\n\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03W\x02\x08\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03W\t\x0c\n\x0c\n\
    \x05\x04\x01\x02\x02\x03\x12\x03W\x0f\x10\nT\n\x04\x04\x01\x02\x03\x12\
    \x03Z\x02\x1b\x1aG\x20Optional\x20because\x20the\x20only\x20action\x20th\
    at\x20needs\x20value\x20is\x20KV_ACTION_CREATE\n\n\x0c\n\x05\x04\x01\x02\
    \x03\x04\x12\x03Z\x02\n\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03Z\x0b\x10\
    \n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03Z\x11\x16\n\x0c\n\x05\x04\x01\
    \x02\x03\x03\x12\x03Z\x19\x1ab\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::sp_shared::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(KVStepResponse::generated_message_descriptor_data());
            messages.push(KVStep::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(2);
            enums.push(KVMode::generated_enum_descriptor_data());
            enums.push(KVStatus::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
