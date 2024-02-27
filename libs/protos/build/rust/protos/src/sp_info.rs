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

//! Generated file from `sp_info.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:protos.LiveInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LiveInfo {
    // message fields
    ///  If empty, client has not announced any audiences
    // @@protoc_insertion_point(field:protos.LiveInfo.audiences)
    pub audiences: ::std::vec::Vec<super::sp_common::Audience>,
    // @@protoc_insertion_point(field:protos.LiveInfo.client)
    pub client: ::protobuf::MessageField<ClientInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:protos.LiveInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LiveInfo {
    fn default() -> &'a LiveInfo {
        <LiveInfo as ::protobuf::Message>::default_instance()
    }
}

impl LiveInfo {
    pub fn new() -> LiveInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "audiences",
            |m: &LiveInfo| { &m.audiences },
            |m: &mut LiveInfo| { &mut m.audiences },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ClientInfo>(
            "client",
            |m: &LiveInfo| { &m.client },
            |m: &mut LiveInfo| { &mut m.client },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LiveInfo>(
            "LiveInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LiveInfo {
    const NAME: &'static str = "LiveInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.audiences.push(is.read_message()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.client)?;
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
        for value in &self.audiences {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.client.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.audiences {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if let Some(v) = self.client.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> LiveInfo {
        LiveInfo::new()
    }

    fn clear(&mut self) {
        self.audiences.clear();
        self.client.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LiveInfo {
        static instance: LiveInfo = LiveInfo {
            audiences: ::std::vec::Vec::new(),
            client: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LiveInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LiveInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LiveInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LiveInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:protos.PipelineInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PipelineInfo {
    // message fields
    ///  What audience(s) this pipeline is attached to (none if empty)
    // @@protoc_insertion_point(field:protos.PipelineInfo.audiences)
    pub audiences: ::std::vec::Vec<super::sp_common::Audience>,
    ///  Pipeline config
    // @@protoc_insertion_point(field:protos.PipelineInfo.pipeline)
    pub pipeline: ::protobuf::MessageField<super::sp_pipeline::Pipeline>,
    // special fields
    // @@protoc_insertion_point(special_field:protos.PipelineInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PipelineInfo {
    fn default() -> &'a PipelineInfo {
        <PipelineInfo as ::protobuf::Message>::default_instance()
    }
}

impl PipelineInfo {
    pub fn new() -> PipelineInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "audiences",
            |m: &PipelineInfo| { &m.audiences },
            |m: &mut PipelineInfo| { &mut m.audiences },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::sp_pipeline::Pipeline>(
            "pipeline",
            |m: &PipelineInfo| { &m.pipeline },
            |m: &mut PipelineInfo| { &mut m.pipeline },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PipelineInfo>(
            "PipelineInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PipelineInfo {
    const NAME: &'static str = "PipelineInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.audiences.push(is.read_message()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pipeline)?;
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
        for value in &self.audiences {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.pipeline.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.audiences {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if let Some(v) = self.pipeline.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> PipelineInfo {
        PipelineInfo::new()
    }

    fn clear(&mut self) {
        self.audiences.clear();
        self.pipeline.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PipelineInfo {
        static instance: PipelineInfo = PipelineInfo {
            audiences: ::std::vec::Vec::new(),
            pipeline: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PipelineInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PipelineInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PipelineInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PipelineInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  Most of this is constructed by client SDKs and provided during Register call
// @@protoc_insertion_point(message:protos.ClientInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClientInfo {
    // message fields
    // @@protoc_insertion_point(field:protos.ClientInfo.client_type)
    pub client_type: ::protobuf::EnumOrUnknown<ClientType>,
    // @@protoc_insertion_point(field:protos.ClientInfo.library_name)
    pub library_name: ::std::string::String,
    // @@protoc_insertion_point(field:protos.ClientInfo.library_version)
    pub library_version: ::std::string::String,
    // @@protoc_insertion_point(field:protos.ClientInfo.language)
    pub language: ::std::string::String,
    // @@protoc_insertion_point(field:protos.ClientInfo.arch)
    pub arch: ::std::string::String,
    // @@protoc_insertion_point(field:protos.ClientInfo.os)
    pub os: ::std::string::String,
    ///  Filled out by server on GetAll()
    // @@protoc_insertion_point(field:protos.ClientInfo._session_id)
    pub _session_id: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:protos.ClientInfo._service_name)
    pub _service_name: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:protos.ClientInfo._node_name)
    pub _node_name: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:protos.ClientInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClientInfo {
    fn default() -> &'a ClientInfo {
        <ClientInfo as ::protobuf::Message>::default_instance()
    }
}

impl ClientInfo {
    pub fn new() -> ClientInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_type",
            |m: &ClientInfo| { &m.client_type },
            |m: &mut ClientInfo| { &mut m.client_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "library_name",
            |m: &ClientInfo| { &m.library_name },
            |m: &mut ClientInfo| { &mut m.library_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "library_version",
            |m: &ClientInfo| { &m.library_version },
            |m: &mut ClientInfo| { &mut m.library_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "language",
            |m: &ClientInfo| { &m.language },
            |m: &mut ClientInfo| { &mut m.language },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "arch",
            |m: &ClientInfo| { &m.arch },
            |m: &mut ClientInfo| { &mut m.arch },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "os",
            |m: &ClientInfo| { &m.os },
            |m: &mut ClientInfo| { &mut m.os },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "_session_id",
            |m: &ClientInfo| { &m._session_id },
            |m: &mut ClientInfo| { &mut m._session_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "_service_name",
            |m: &ClientInfo| { &m._service_name },
            |m: &mut ClientInfo| { &mut m._service_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "_node_name",
            |m: &ClientInfo| { &m._node_name },
            |m: &mut ClientInfo| { &mut m._node_name },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClientInfo>(
            "ClientInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClientInfo {
    const NAME: &'static str = "ClientInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.client_type = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.library_name = is.read_string()?;
                },
                26 => {
                    self.library_version = is.read_string()?;
                },
                34 => {
                    self.language = is.read_string()?;
                },
                42 => {
                    self.arch = is.read_string()?;
                },
                50 => {
                    self.os = is.read_string()?;
                },
                58 => {
                    self._session_id = ::std::option::Option::Some(is.read_string()?);
                },
                66 => {
                    self._service_name = ::std::option::Option::Some(is.read_string()?);
                },
                74 => {
                    self._node_name = ::std::option::Option::Some(is.read_string()?);
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
        if self.client_type != ::protobuf::EnumOrUnknown::new(ClientType::CLIENT_TYPE_UNSET) {
            my_size += ::protobuf::rt::int32_size(1, self.client_type.value());
        }
        if !self.library_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.library_name);
        }
        if !self.library_version.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.library_version);
        }
        if !self.language.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.language);
        }
        if !self.arch.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.arch);
        }
        if !self.os.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.os);
        }
        if let Some(v) = self._session_id.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(v) = self._service_name.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self._node_name.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.client_type != ::protobuf::EnumOrUnknown::new(ClientType::CLIENT_TYPE_UNSET) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.client_type))?;
        }
        if !self.library_name.is_empty() {
            os.write_string(2, &self.library_name)?;
        }
        if !self.library_version.is_empty() {
            os.write_string(3, &self.library_version)?;
        }
        if !self.language.is_empty() {
            os.write_string(4, &self.language)?;
        }
        if !self.arch.is_empty() {
            os.write_string(5, &self.arch)?;
        }
        if !self.os.is_empty() {
            os.write_string(6, &self.os)?;
        }
        if let Some(v) = self._session_id.as_ref() {
            os.write_string(7, v)?;
        }
        if let Some(v) = self._service_name.as_ref() {
            os.write_string(8, v)?;
        }
        if let Some(v) = self._node_name.as_ref() {
            os.write_string(9, v)?;
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

    fn new() -> ClientInfo {
        ClientInfo::new()
    }

    fn clear(&mut self) {
        self.client_type = ::protobuf::EnumOrUnknown::new(ClientType::CLIENT_TYPE_UNSET);
        self.library_name.clear();
        self.library_version.clear();
        self.language.clear();
        self.arch.clear();
        self.os.clear();
        self._session_id = ::std::option::Option::None;
        self._service_name = ::std::option::Option::None;
        self._node_name = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClientInfo {
        static instance: ClientInfo = ClientInfo {
            client_type: ::protobuf::EnumOrUnknown::from_i32(0),
            library_name: ::std::string::String::new(),
            library_version: ::std::string::String::new(),
            language: ::std::string::String::new(),
            arch: ::std::string::String::new(),
            os: ::std::string::String::new(),
            _session_id: ::std::option::Option::None,
            _service_name: ::std::option::Option::None,
            _node_name: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClientInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClientInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClientInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:protos.ClientType)
pub enum ClientType {
    // @@protoc_insertion_point(enum_value:protos.ClientType.CLIENT_TYPE_UNSET)
    CLIENT_TYPE_UNSET = 0,
    // @@protoc_insertion_point(enum_value:protos.ClientType.CLIENT_TYPE_SDK)
    CLIENT_TYPE_SDK = 1,
    // @@protoc_insertion_point(enum_value:protos.ClientType.CLIENT_TYPE_SHIM)
    CLIENT_TYPE_SHIM = 2,
}

impl ::protobuf::Enum for ClientType {
    const NAME: &'static str = "ClientType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ClientType> {
        match value {
            0 => ::std::option::Option::Some(ClientType::CLIENT_TYPE_UNSET),
            1 => ::std::option::Option::Some(ClientType::CLIENT_TYPE_SDK),
            2 => ::std::option::Option::Some(ClientType::CLIENT_TYPE_SHIM),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<ClientType> {
        match str {
            "CLIENT_TYPE_UNSET" => ::std::option::Option::Some(ClientType::CLIENT_TYPE_UNSET),
            "CLIENT_TYPE_SDK" => ::std::option::Option::Some(ClientType::CLIENT_TYPE_SDK),
            "CLIENT_TYPE_SHIM" => ::std::option::Option::Some(ClientType::CLIENT_TYPE_SHIM),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ClientType] = &[
        ClientType::CLIENT_TYPE_UNSET,
        ClientType::CLIENT_TYPE_SDK,
        ClientType::CLIENT_TYPE_SHIM,
    ];
}

impl ::protobuf::EnumFull for ClientType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ClientType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ClientType {
    fn default() -> Self {
        ClientType::CLIENT_TYPE_UNSET
    }
}

impl ClientType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ClientType>("ClientType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rsp_info.proto\x12\x06protos\x1a\x0fsp_common.proto\x1a\x11sp_pipelin\
    e.proto\"f\n\x08LiveInfo\x12.\n\taudiences\x18\x01\x20\x03(\x0b2\x10.pro\
    tos.AudienceR\taudiences\x12*\n\x06client\x18\x02\x20\x01(\x0b2\x12.prot\
    os.ClientInfoR\x06client\"l\n\x0cPipelineInfo\x12.\n\taudiences\x18\x01\
    \x20\x03(\x0b2\x10.protos.AudienceR\taudiences\x12,\n\x08pipeline\x18\
    \x02\x20\x01(\x0b2\x10.protos.PipelineR\x08pipeline\"\xef\x02\n\nClientI\
    nfo\x123\n\x0bclient_type\x18\x01\x20\x01(\x0e2\x12.protos.ClientTypeR\n\
    clientType\x12!\n\x0clibrary_name\x18\x02\x20\x01(\tR\x0blibraryName\x12\
    '\n\x0flibrary_version\x18\x03\x20\x01(\tR\x0elibraryVersion\x12\x1a\n\
    \x08language\x18\x04\x20\x01(\tR\x08language\x12\x12\n\x04arch\x18\x05\
    \x20\x01(\tR\x04arch\x12\x0e\n\x02os\x18\x06\x20\x01(\tR\x02os\x12#\n\
    \x0b_session_id\x18\x07\x20\x01(\tH\0R\tSessionId\x88\x01\x01\x12'\n\r_s\
    ervice_name\x18\x08\x20\x01(\tH\x01R\x0bServiceName\x88\x01\x01\x12!\n\n\
    _node_name\x18\t\x20\x01(\tH\x02R\x08NodeName\x88\x01\x01B\x0e\n\x0cX_se\
    ssion_idB\x10\n\x0eX_service_nameB\r\n\x0bX_node_name*N\n\nClientType\
    \x12\x15\n\x11CLIENT_TYPE_UNSET\x10\0\x12\x13\n\x0fCLIENT_TYPE_SDK\x10\
    \x01\x12\x14\n\x10CLIENT_TYPE_SHIM\x10\x02B<Z:github.com/streamdal/strea\
    mdal/libs/protos/build/go/protosJ\xd4\x0b\n\x06\x12\x04\0\0*\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\0\x0f\n\t\n\x02\x03\
    \0\x12\x03\x04\0\x19\n\t\n\x02\x03\x01\x12\x03\x05\0\x1b\n\x08\n\x01\x08\
    \x12\x03\x07\0Q\n\t\n\x02\x08\x0b\x12\x03\x07\0Q\n\n\n\x02\x04\0\x12\x04\
    \t\0\r\x01\n\n\n\x03\x04\0\x01\x12\x03\t\x08\x10\n?\n\x04\x04\0\x02\0\
    \x12\x03\x0b\x02\"\x1a2\x20If\x20empty,\x20client\x20has\x20not\x20annou\
    nced\x20any\x20audiences\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x0b\x02\n\
    \n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0b\x0b\x13\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x0b\x14\x1d\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0b\x20!\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x02\x18\n\x0c\n\x05\x04\0\x02\x01\
    \x06\x12\x03\x0c\x02\x0c\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\r\x13\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0c\x16\x17\n\n\n\x02\x04\x01\x12\
    \x04\x0f\0\x15\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0f\x08\x14\nL\n\x04\
    \x04\x01\x02\0\x12\x03\x11\x02)\x1a?\x20What\x20audience(s)\x20this\x20p\
    ipeline\x20is\x20attached\x20to\x20(none\x20if\x20empty)\n\n\x0c\n\x05\
    \x04\x01\x02\0\x04\x12\x03\x11\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\
    \x03\x11\x0b\x1a\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x11\x1b$\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\x11'(\n\x1e\n\x04\x04\x01\x02\x01\x12\x03\
    \x14\x02\x1f\x1a\x11\x20Pipeline\x20config\n\n\x0c\n\x05\x04\x01\x02\x01\
    \x06\x12\x03\x14\x02\x11\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x14\x12\
    \x1a\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x14\x1d\x1e\n\n\n\x02\x05\0\
    \x12\x04\x17\0\x1b\x01\n\n\n\x03\x05\0\x01\x12\x03\x17\x05\x0f\n\x0b\n\
    \x04\x05\0\x02\0\x12\x03\x18\x02\x18\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\
    \x18\x02\x13\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x18\x16\x17\n\x0b\n\x04\
    \x05\0\x02\x01\x12\x03\x19\x02\x16\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\
    \x19\x02\x11\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x19\x14\x15\n\x0b\n\
    \x04\x05\0\x02\x02\x12\x03\x1a\x02\x17\n\x0c\n\x05\x05\0\x02\x02\x01\x12\
    \x03\x1a\x02\x12\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x1a\x15\x16\nZ\n\
    \x02\x04\x02\x12\x04\x1e\0*\x01\x1aN\x20Most\x20of\x20this\x20is\x20cons\
    tructed\x20by\x20client\x20SDKs\x20and\x20provided\x20during\x20Register\
    \x20call\n\n\n\n\x03\x04\x02\x01\x12\x03\x1e\x08\x12\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03\x1f\x02\x1d\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x1f\x02\
    \x0c\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1f\r\x18\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x03\x1f\x1b\x1c\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x20\
    \x02\x1a\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x20\x02\x08\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03\x20\t\x15\n\x0c\n\x05\x04\x02\x02\x01\x03\
    \x12\x03\x20\x18\x19\n\x0b\n\x04\x04\x02\x02\x02\x12\x03!\x02\x1d\n\x0c\
    \n\x05\x04\x02\x02\x02\x05\x12\x03!\x02\x08\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03!\t\x18\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03!\x1b\x1c\n\
    \x0b\n\x04\x04\x02\x02\x03\x12\x03\"\x02\x16\n\x0c\n\x05\x04\x02\x02\x03\
    \x05\x12\x03\"\x02\x08\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03\"\t\x11\n\
    \x0c\n\x05\x04\x02\x02\x03\x03\x12\x03\"\x14\x15\n\x0b\n\x04\x04\x02\x02\
    \x04\x12\x03#\x02\x12\n\x0c\n\x05\x04\x02\x02\x04\x05\x12\x03#\x02\x08\n\
    \x0c\n\x05\x04\x02\x02\x04\x01\x12\x03#\t\r\n\x0c\n\x05\x04\x02\x02\x04\
    \x03\x12\x03#\x10\x11\n\x0b\n\x04\x04\x02\x02\x05\x12\x03$\x02\x10\n\x0c\
    \n\x05\x04\x02\x02\x05\x05\x12\x03$\x02\x08\n\x0c\n\x05\x04\x02\x02\x05\
    \x01\x12\x03$\t\x0b\n\x0c\n\x05\x04\x02\x02\x05\x03\x12\x03$\x0e\x0f\nf\
    \n\x04\x04\x02\x02\x06\x12\x03'\x02\"\x1a\"\x20Filled\x20out\x20by\x20se\
    rver\x20on\x20GetAll()\n\"5\x20protolint:disable:this\x20FIELD_NAMES_LOW\
    ER_SNAKE_CASE\n\n\x0c\n\x05\x04\x02\x02\x06\x04\x12\x03'\x02\n\n\x0c\n\
    \x05\x04\x02\x02\x06\x05\x12\x03'\x0b\x11\n\x0c\n\x05\x04\x02\x02\x06\
    \x01\x12\x03'\x12\x1d\n\x0c\n\x05\x04\x02\x02\x06\x03\x12\x03'\x20!\nB\n\
    \x04\x04\x02\x02\x07\x12\x03(\x02$\"5\x20protolint:disable:this\x20FIELD\
    _NAMES_LOWER_SNAKE_CASE\n\n\x0c\n\x05\x04\x02\x02\x07\x04\x12\x03(\x02\n\
    \n\x0c\n\x05\x04\x02\x02\x07\x05\x12\x03(\x0b\x11\n\x0c\n\x05\x04\x02\
    \x02\x07\x01\x12\x03(\x12\x1f\n\x0c\n\x05\x04\x02\x02\x07\x03\x12\x03(\"\
    #\nB\n\x04\x04\x02\x02\x08\x12\x03)\x02!\"5\x20protolint:disable:this\
    \x20FIELD_NAMES_LOWER_SNAKE_CASE\n\n\x0c\n\x05\x04\x02\x02\x08\x04\x12\
    \x03)\x02\n\n\x0c\n\x05\x04\x02\x02\x08\x05\x12\x03)\x0b\x11\n\x0c\n\x05\
    \x04\x02\x02\x08\x01\x12\x03)\x12\x1c\n\x0c\n\x05\x04\x02\x02\x08\x03\
    \x12\x03)\x1f\x20b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::sp_common::file_descriptor().clone());
            deps.push(super::sp_pipeline::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(LiveInfo::generated_message_descriptor_data());
            messages.push(PipelineInfo::generated_message_descriptor_data());
            messages.push(ClientInfo::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(ClientType::generated_enum_descriptor_data());
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
