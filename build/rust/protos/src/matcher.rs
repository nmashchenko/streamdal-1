// This file is generated by rust-protobuf 3.2.0. Do not edit
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

//! Generated file from `steps/match.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:steps.MatchRequest)
pub struct MatchRequest {
    // message fields
    // @@protoc_insertion_point(field:steps.MatchRequest.data)
    pub data: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:steps.MatchRequest.path)
    pub path: ::std::string::String,
    // @@protoc_insertion_point(field:steps.MatchRequest.args)
    pub args: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:steps.MatchRequest.negate)
    pub negate: bool,
    // @@protoc_insertion_point(field:steps.MatchRequest.type)
    pub type_: ::protobuf::EnumOrUnknown<MatchType>,
    // special fields
    // @@protoc_insertion_point(special_field:steps.MatchRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MatchRequest {
    fn default() -> &'a MatchRequest {
        <MatchRequest as ::protobuf::Message>::default_instance()
    }
}

impl MatchRequest {
    pub fn new() -> MatchRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &MatchRequest| { &m.data },
            |m: &mut MatchRequest| { &mut m.data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "path",
            |m: &MatchRequest| { &m.path },
            |m: &mut MatchRequest| { &mut m.path },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "args",
            |m: &MatchRequest| { &m.args },
            |m: &mut MatchRequest| { &mut m.args },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "negate",
            |m: &MatchRequest| { &m.negate },
            |m: &mut MatchRequest| { &mut m.negate },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &MatchRequest| { &m.type_ },
            |m: &mut MatchRequest| { &mut m.type_ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MatchRequest>(
            "MatchRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MatchRequest {
    const NAME: &'static str = "MatchRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.data = is.read_bytes()?;
                },
                18 => {
                    self.path = is.read_string()?;
                },
                26 => {
                    self.args.push(is.read_string()?);
                },
                32 => {
                    self.negate = is.read_bool()?;
                },
                40 => {
                    self.type_ = is.read_enum_or_unknown()?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        }
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.path);
        }
        for value in &self.args {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if self.negate != false {
            my_size += 1 + 1;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(MatchType::MATCH_TYPE_UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(5, self.type_.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
        }
        if !self.path.is_empty() {
            os.write_string(2, &self.path)?;
        }
        for v in &self.args {
            os.write_string(3, &v)?;
        };
        if self.negate != false {
            os.write_bool(4, self.negate)?;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(MatchType::MATCH_TYPE_UNKNOWN) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.type_))?;
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

    fn new() -> MatchRequest {
        MatchRequest::new()
    }

    fn clear(&mut self) {
        self.data.clear();
        self.path.clear();
        self.args.clear();
        self.negate = false;
        self.type_ = ::protobuf::EnumOrUnknown::new(MatchType::MATCH_TYPE_UNKNOWN);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MatchRequest {
        static instance: MatchRequest = MatchRequest {
            data: ::std::vec::Vec::new(),
            path: ::std::string::String::new(),
            args: ::std::vec::Vec::new(),
            negate: false,
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MatchRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MatchRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MatchRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MatchRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:steps.MatchResponse)
pub struct MatchResponse {
    // message fields
    // @@protoc_insertion_point(field:steps.MatchResponse.is_match)
    pub is_match: bool,
    // @@protoc_insertion_point(field:steps.MatchResponse.error)
    pub error: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:steps.MatchResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MatchResponse {
    fn default() -> &'a MatchResponse {
        <MatchResponse as ::protobuf::Message>::default_instance()
    }
}

impl MatchResponse {
    pub fn new() -> MatchResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_match",
            |m: &MatchResponse| { &m.is_match },
            |m: &mut MatchResponse| { &mut m.is_match },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "error",
            |m: &MatchResponse| { &m.error },
            |m: &mut MatchResponse| { &mut m.error },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MatchResponse>(
            "MatchResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MatchResponse {
    const NAME: &'static str = "MatchResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.is_match = is.read_bool()?;
                },
                18 => {
                    self.error = is.read_string()?;
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
        if self.is_match != false {
            my_size += 1 + 1;
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_match != false {
            os.write_bool(1, self.is_match)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

    fn new() -> MatchResponse {
        MatchResponse::new()
    }

    fn clear(&mut self) {
        self.is_match = false;
        self.error.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MatchResponse {
        static instance: MatchResponse = MatchResponse {
            is_match: false,
            error: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MatchResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MatchResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MatchResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MatchResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:steps.MatchType)
pub enum MatchType {
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_UNKNOWN)
    MATCH_TYPE_UNKNOWN = 0,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_IS_EMPTY)
    MATCH_TYPE_IS_EMPTY = 1000,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_HAS_FIELD)
    MATCH_TYPE_HAS_FIELD = 1001,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_IS_TYPE)
    MATCH_TYPE_IS_TYPE = 1002,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_STRING_CONTAINS_ANY)
    MATCH_TYPE_STRING_CONTAINS_ANY = 1003,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_STRING_CONTAINS_ALL)
    MATCH_TYPE_STRING_CONTAINS_ALL = 1004,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_STRING_EQUAL)
    MATCH_TYPE_STRING_EQUAL = 1005,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_IPV4_ADDRESS)
    MATCH_TYPE_IPV4_ADDRESS = 1006,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_IPV6_ADDRESS)
    MATCH_TYPE_IPV6_ADDRESS = 1007,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_MAC_ADDRESS)
    MATCH_TYPE_MAC_ADDRESS = 1008,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_REGEX)
    MATCH_TYPE_REGEX = 1009,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_TIMESTAMP_RFC3339)
    MATCH_TYPE_TIMESTAMP_RFC3339 = 1010,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_TIMESTAMP_UNIX_NANO)
    MATCH_TYPE_TIMESTAMP_UNIX_NANO = 1011,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_TIMESTAMP_UNIX)
    MATCH_TYPE_TIMESTAMP_UNIX = 1012,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_BOOLEAN_TRUE)
    MATCH_TYPE_BOOLEAN_TRUE = 1013,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_BOOLEAN_FALSE)
    MATCH_TYPE_BOOLEAN_FALSE = 1014,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_UUID)
    MATCH_TYPE_UUID = 1015,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_ANY)
    MATCH_TYPE_PII_ANY = 2000,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_CREDIT_CARD)
    MATCH_TYPE_PII_CREDIT_CARD = 2001,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_SSN)
    MATCH_TYPE_PII_SSN = 2002,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_EMAIL)
    MATCH_TYPE_PII_EMAIL = 2003,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_PHONE)
    MATCH_TYPE_PII_PHONE = 2004,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_DRIVER_LICENSE)
    MATCH_TYPE_PII_DRIVER_LICENSE = 2005,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_PASSPORT_ID)
    MATCH_TYPE_PII_PASSPORT_ID = 2006,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_VIN_NUMBER)
    MATCH_TYPE_PII_VIN_NUMBER = 2007,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_SERIAL_NUMBER)
    MATCH_TYPE_PII_SERIAL_NUMBER = 2008,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_LOGIN)
    MATCH_TYPE_PII_LOGIN = 2009,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_TAXPAYER_ID)
    MATCH_TYPE_PII_TAXPAYER_ID = 2010,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_ADDRESS)
    MATCH_TYPE_PII_ADDRESS = 2011,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_SIGNATURE)
    MATCH_TYPE_PII_SIGNATURE = 2012,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_GEOLOCATION)
    MATCH_TYPE_PII_GEOLOCATION = 2013,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_EDUCATION)
    MATCH_TYPE_PII_EDUCATION = 2014,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_FINANCIAL)
    MATCH_TYPE_PII_FINANCIAL = 2015,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_PII_HEALTH)
    MATCH_TYPE_PII_HEALTH = 2016,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_NUMERIC_EQUAL_TO)
    MATCH_TYPE_NUMERIC_EQUAL_TO = 3000,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_NUMERIC_GREATER_THAN)
    MATCH_TYPE_NUMERIC_GREATER_THAN = 3001,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_NUMERIC_GREATER_EQUAL)
    MATCH_TYPE_NUMERIC_GREATER_EQUAL = 3002,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_NUMERIC_LESS_THAN)
    MATCH_TYPE_NUMERIC_LESS_THAN = 3003,
    // @@protoc_insertion_point(enum_value:steps.MatchType.MATCH_TYPE_NUMERIC_LESS_EQUAL)
    MATCH_TYPE_NUMERIC_LESS_EQUAL = 3004,
}

impl ::protobuf::Enum for MatchType {
    const NAME: &'static str = "MatchType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MatchType> {
        match value {
            0 => ::std::option::Option::Some(MatchType::MATCH_TYPE_UNKNOWN),
            1000 => ::std::option::Option::Some(MatchType::MATCH_TYPE_IS_EMPTY),
            1001 => ::std::option::Option::Some(MatchType::MATCH_TYPE_HAS_FIELD),
            1002 => ::std::option::Option::Some(MatchType::MATCH_TYPE_IS_TYPE),
            1003 => ::std::option::Option::Some(MatchType::MATCH_TYPE_STRING_CONTAINS_ANY),
            1004 => ::std::option::Option::Some(MatchType::MATCH_TYPE_STRING_CONTAINS_ALL),
            1005 => ::std::option::Option::Some(MatchType::MATCH_TYPE_STRING_EQUAL),
            1006 => ::std::option::Option::Some(MatchType::MATCH_TYPE_IPV4_ADDRESS),
            1007 => ::std::option::Option::Some(MatchType::MATCH_TYPE_IPV6_ADDRESS),
            1008 => ::std::option::Option::Some(MatchType::MATCH_TYPE_MAC_ADDRESS),
            1009 => ::std::option::Option::Some(MatchType::MATCH_TYPE_REGEX),
            1010 => ::std::option::Option::Some(MatchType::MATCH_TYPE_TIMESTAMP_RFC3339),
            1011 => ::std::option::Option::Some(MatchType::MATCH_TYPE_TIMESTAMP_UNIX_NANO),
            1012 => ::std::option::Option::Some(MatchType::MATCH_TYPE_TIMESTAMP_UNIX),
            1013 => ::std::option::Option::Some(MatchType::MATCH_TYPE_BOOLEAN_TRUE),
            1014 => ::std::option::Option::Some(MatchType::MATCH_TYPE_BOOLEAN_FALSE),
            1015 => ::std::option::Option::Some(MatchType::MATCH_TYPE_UUID),
            2000 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_ANY),
            2001 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_CREDIT_CARD),
            2002 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_SSN),
            2003 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_EMAIL),
            2004 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_PHONE),
            2005 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_DRIVER_LICENSE),
            2006 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_PASSPORT_ID),
            2007 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_VIN_NUMBER),
            2008 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_SERIAL_NUMBER),
            2009 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_LOGIN),
            2010 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_TAXPAYER_ID),
            2011 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_ADDRESS),
            2012 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_SIGNATURE),
            2013 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_GEOLOCATION),
            2014 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_EDUCATION),
            2015 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_FINANCIAL),
            2016 => ::std::option::Option::Some(MatchType::MATCH_TYPE_PII_HEALTH),
            3000 => ::std::option::Option::Some(MatchType::MATCH_TYPE_NUMERIC_EQUAL_TO),
            3001 => ::std::option::Option::Some(MatchType::MATCH_TYPE_NUMERIC_GREATER_THAN),
            3002 => ::std::option::Option::Some(MatchType::MATCH_TYPE_NUMERIC_GREATER_EQUAL),
            3003 => ::std::option::Option::Some(MatchType::MATCH_TYPE_NUMERIC_LESS_THAN),
            3004 => ::std::option::Option::Some(MatchType::MATCH_TYPE_NUMERIC_LESS_EQUAL),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [MatchType] = &[
        MatchType::MATCH_TYPE_UNKNOWN,
        MatchType::MATCH_TYPE_IS_EMPTY,
        MatchType::MATCH_TYPE_HAS_FIELD,
        MatchType::MATCH_TYPE_IS_TYPE,
        MatchType::MATCH_TYPE_STRING_CONTAINS_ANY,
        MatchType::MATCH_TYPE_STRING_CONTAINS_ALL,
        MatchType::MATCH_TYPE_STRING_EQUAL,
        MatchType::MATCH_TYPE_IPV4_ADDRESS,
        MatchType::MATCH_TYPE_IPV6_ADDRESS,
        MatchType::MATCH_TYPE_MAC_ADDRESS,
        MatchType::MATCH_TYPE_REGEX,
        MatchType::MATCH_TYPE_TIMESTAMP_RFC3339,
        MatchType::MATCH_TYPE_TIMESTAMP_UNIX_NANO,
        MatchType::MATCH_TYPE_TIMESTAMP_UNIX,
        MatchType::MATCH_TYPE_BOOLEAN_TRUE,
        MatchType::MATCH_TYPE_BOOLEAN_FALSE,
        MatchType::MATCH_TYPE_UUID,
        MatchType::MATCH_TYPE_PII_ANY,
        MatchType::MATCH_TYPE_PII_CREDIT_CARD,
        MatchType::MATCH_TYPE_PII_SSN,
        MatchType::MATCH_TYPE_PII_EMAIL,
        MatchType::MATCH_TYPE_PII_PHONE,
        MatchType::MATCH_TYPE_PII_DRIVER_LICENSE,
        MatchType::MATCH_TYPE_PII_PASSPORT_ID,
        MatchType::MATCH_TYPE_PII_VIN_NUMBER,
        MatchType::MATCH_TYPE_PII_SERIAL_NUMBER,
        MatchType::MATCH_TYPE_PII_LOGIN,
        MatchType::MATCH_TYPE_PII_TAXPAYER_ID,
        MatchType::MATCH_TYPE_PII_ADDRESS,
        MatchType::MATCH_TYPE_PII_SIGNATURE,
        MatchType::MATCH_TYPE_PII_GEOLOCATION,
        MatchType::MATCH_TYPE_PII_EDUCATION,
        MatchType::MATCH_TYPE_PII_FINANCIAL,
        MatchType::MATCH_TYPE_PII_HEALTH,
        MatchType::MATCH_TYPE_NUMERIC_EQUAL_TO,
        MatchType::MATCH_TYPE_NUMERIC_GREATER_THAN,
        MatchType::MATCH_TYPE_NUMERIC_GREATER_EQUAL,
        MatchType::MATCH_TYPE_NUMERIC_LESS_THAN,
        MatchType::MATCH_TYPE_NUMERIC_LESS_EQUAL,
    ];
}

impl ::protobuf::EnumFull for MatchType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("MatchType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            MatchType::MATCH_TYPE_UNKNOWN => 0,
            MatchType::MATCH_TYPE_IS_EMPTY => 1,
            MatchType::MATCH_TYPE_HAS_FIELD => 2,
            MatchType::MATCH_TYPE_IS_TYPE => 3,
            MatchType::MATCH_TYPE_STRING_CONTAINS_ANY => 4,
            MatchType::MATCH_TYPE_STRING_CONTAINS_ALL => 5,
            MatchType::MATCH_TYPE_STRING_EQUAL => 6,
            MatchType::MATCH_TYPE_IPV4_ADDRESS => 7,
            MatchType::MATCH_TYPE_IPV6_ADDRESS => 8,
            MatchType::MATCH_TYPE_MAC_ADDRESS => 9,
            MatchType::MATCH_TYPE_REGEX => 10,
            MatchType::MATCH_TYPE_TIMESTAMP_RFC3339 => 11,
            MatchType::MATCH_TYPE_TIMESTAMP_UNIX_NANO => 12,
            MatchType::MATCH_TYPE_TIMESTAMP_UNIX => 13,
            MatchType::MATCH_TYPE_BOOLEAN_TRUE => 14,
            MatchType::MATCH_TYPE_BOOLEAN_FALSE => 15,
            MatchType::MATCH_TYPE_UUID => 16,
            MatchType::MATCH_TYPE_PII_ANY => 17,
            MatchType::MATCH_TYPE_PII_CREDIT_CARD => 18,
            MatchType::MATCH_TYPE_PII_SSN => 19,
            MatchType::MATCH_TYPE_PII_EMAIL => 20,
            MatchType::MATCH_TYPE_PII_PHONE => 21,
            MatchType::MATCH_TYPE_PII_DRIVER_LICENSE => 22,
            MatchType::MATCH_TYPE_PII_PASSPORT_ID => 23,
            MatchType::MATCH_TYPE_PII_VIN_NUMBER => 24,
            MatchType::MATCH_TYPE_PII_SERIAL_NUMBER => 25,
            MatchType::MATCH_TYPE_PII_LOGIN => 26,
            MatchType::MATCH_TYPE_PII_TAXPAYER_ID => 27,
            MatchType::MATCH_TYPE_PII_ADDRESS => 28,
            MatchType::MATCH_TYPE_PII_SIGNATURE => 29,
            MatchType::MATCH_TYPE_PII_GEOLOCATION => 30,
            MatchType::MATCH_TYPE_PII_EDUCATION => 31,
            MatchType::MATCH_TYPE_PII_FINANCIAL => 32,
            MatchType::MATCH_TYPE_PII_HEALTH => 33,
            MatchType::MATCH_TYPE_NUMERIC_EQUAL_TO => 34,
            MatchType::MATCH_TYPE_NUMERIC_GREATER_THAN => 35,
            MatchType::MATCH_TYPE_NUMERIC_GREATER_EQUAL => 36,
            MatchType::MATCH_TYPE_NUMERIC_LESS_THAN => 37,
            MatchType::MATCH_TYPE_NUMERIC_LESS_EQUAL => 38,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for MatchType {
    fn default() -> Self {
        MatchType::MATCH_TYPE_UNKNOWN
    }
}

impl MatchType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<MatchType>("MatchType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13rules/match.proto\x12\x05rules\"\x88\x01\n\x0cMatchRequest\x12\
    \x12\n\x04data\x18\x01\x20\x01(\x0cR\x04data\x12\x12\n\x04path\x18\x02\
    \x20\x01(\tR\x04path\x12\x12\n\x04args\x18\x03\x20\x03(\tR\x04args\x12\
    \x16\n\x06negate\x18\x04\x20\x01(\x08R\x06negate\x12$\n\x04type\x18\x05\
    \x20\x01(\x0e2\x10.steps.MatchTypeR\x04type\"@\n\rMatchResponse\x12\x19\
    \n\x08is_match\x18\x01\x20\x01(\x08R\x07isMatch\x12\x14\n\x05error\x18\
    \x02\x20\x01(\tR\x05error*\xbe\t\n\tMatchType\x12\x16\n\x12MATCH_TYPE_UN\
    KNOWN\x10\0\x12\x18\n\x13MATCH_TYPE_IS_EMPTY\x10\xe8\x07\x12\x19\n\x14MA\
    TCH_TYPE_HAS_FIELD\x10\xe9\x07\x12\x17\n\x12MATCH_TYPE_IS_TYPE\x10\xea\
    \x07\x12#\n\x1eMATCH_TYPE_STRING_CONTAINS_ANY\x10\xeb\x07\x12#\n\x1eMATC\
    H_TYPE_STRING_CONTAINS_ALL\x10\xec\x07\x12\x1c\n\x17MATCH_TYPE_STRING_EQ\
    UAL\x10\xed\x07\x12\x1c\n\x17MATCH_TYPE_IPV4_ADDRESS\x10\xee\x07\x12\x1c\
    \n\x17MATCH_TYPE_IPV6_ADDRESS\x10\xef\x07\x12\x1b\n\x16MATCH_TYPE_MAC_AD\
    DRESS\x10\xf0\x07\x12\x15\n\x10MATCH_TYPE_REGEX\x10\xf1\x07\x12!\n\x1cMA\
    TCH_TYPE_TIMESTAMP_RFC3339\x10\xf2\x07\x12#\n\x1eMATCH_TYPE_TIMESTAMP_UN\
    IX_NANO\x10\xf3\x07\x12\x1e\n\x19MATCH_TYPE_TIMESTAMP_UNIX\x10\xf4\x07\
    \x12\x1c\n\x17MATCH_TYPE_BOOLEAN_TRUE\x10\xf5\x07\x12\x1d\n\x18MATCH_TYP\
    E_BOOLEAN_FALSE\x10\xf6\x07\x12\x14\n\x0fMATCH_TYPE_UUID\x10\xf7\x07\x12\
    \x17\n\x12MATCH_TYPE_PII_ANY\x10\xd0\x0f\x12\x1f\n\x1aMATCH_TYPE_PII_CRE\
    DIT_CARD\x10\xd1\x0f\x12\x17\n\x12MATCH_TYPE_PII_SSN\x10\xd2\x0f\x12\x19\
    \n\x14MATCH_TYPE_PII_EMAIL\x10\xd3\x0f\x12\x19\n\x14MATCH_TYPE_PII_PHONE\
    \x10\xd4\x0f\x12\"\n\x1dMATCH_TYPE_PII_DRIVER_LICENSE\x10\xd5\x0f\x12\
    \x1f\n\x1aMATCH_TYPE_PII_PASSPORT_ID\x10\xd6\x0f\x12\x1e\n\x19MATCH_TYPE\
    _PII_VIN_NUMBER\x10\xd7\x0f\x12!\n\x1cMATCH_TYPE_PII_SERIAL_NUMBER\x10\
    \xd8\x0f\x12\x19\n\x14MATCH_TYPE_PII_LOGIN\x10\xd9\x0f\x12\x1f\n\x1aMATC\
    H_TYPE_PII_TAXPAYER_ID\x10\xda\x0f\x12\x1b\n\x16MATCH_TYPE_PII_ADDRESS\
    \x10\xdb\x0f\x12\x1d\n\x18MATCH_TYPE_PII_SIGNATURE\x10\xdc\x0f\x12\x1f\n\
    \x1aMATCH_TYPE_PII_GEOLOCATION\x10\xdd\x0f\x12\x1d\n\x18MATCH_TYPE_PII_E\
    DUCATION\x10\xde\x0f\x12\x1d\n\x18MATCH_TYPE_PII_FINANCIAL\x10\xdf\x0f\
    \x12\x1a\n\x15MATCH_TYPE_PII_HEALTH\x10\xe0\x0f\x12\x20\n\x1bMATCH_TYPE_\
    NUMERIC_EQUAL_TO\x10\xb8\x17\x12$\n\x1fMATCH_TYPE_NUMERIC_GREATER_THAN\
    \x10\xb9\x17\x12%\n\x20MATCH_TYPE_NUMERIC_GREATER_EQUAL\x10\xba\x17\x12!\
    \n\x1cMATCH_TYPE_NUMERIC_LESS_THAN\x10\xbb\x17\x12\"\n\x1dMATCH_TYPE_NUM\
    ERIC_LESS_EQUAL\x10\xbc\x17B:Z8github.com/streamdal/snitch-protos/build/\
    go/protos/rulesJ\xfe\x1b\n\x06\x12\x04\0\0T\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n\x08\n\x01\x02\x12\x03\x02\0\x0e\n\x08\n\x01\x08\x12\x03\x04\0\
    O\n\t\n\x02\x08\x0b\x12\x03\x04\0O\n5\n\x02\x05\0\x12\x04\x06\0G\x01\")\
    \x201000-1999\x20reserved\x20for\x20core\x20match\x20types\n\n\n\n\x03\
    \x05\0\x01\x12\x03\x06\x05\x0e\n\x0b\n\x04\x05\0\x02\0\x12\x03\t\x02\x19\
    \n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\t\x02\x14\n\x0c\n\x05\x05\0\x02\0\
    \x02\x12\x03\t\x17\x18\n\x0b\n\x04\x05\0\x02\x01\x12\x03\n\x02\x1d\n\x0c\
    \n\x05\x05\0\x02\x01\x01\x12\x03\n\x02\x15\n\x0c\n\x05\x05\0\x02\x01\x02\
    \x12\x03\n\x18\x1c\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x0b\x02\x1e\n\x0c\n\
    \x05\x05\0\x02\x02\x01\x12\x03\x0b\x02\x16\n\x0c\n\x05\x05\0\x02\x02\x02\
    \x12\x03\x0b\x19\x1d\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x0c\x02\x1c\n\x0c\
    \n\x05\x05\0\x02\x03\x01\x12\x03\x0c\x02\x14\n\x0c\n\x05\x05\0\x02\x03\
    \x02\x12\x03\x0c\x17\x1b\n\x0b\n\x04\x05\0\x02\x04\x12\x03\r\x02(\n\x0c\
    \n\x05\x05\0\x02\x04\x01\x12\x03\r\x02\x20\n\x0c\n\x05\x05\0\x02\x04\x02\
    \x12\x03\r#'\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x0e\x02(\n\x0c\n\x05\x05\
    \0\x02\x05\x01\x12\x03\x0e\x02\x20\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\
    \x0e#'\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x0f\x02!\n\x0c\n\x05\x05\0\x02\
    \x06\x01\x12\x03\x0f\x02\x19\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\x0f\
    \x1c\x20\n\x0b\n\x04\x05\0\x02\x07\x12\x03\x10\x02!\n\x0c\n\x05\x05\0\
    \x02\x07\x01\x12\x03\x10\x02\x19\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\
    \x10\x1c\x20\n\x0b\n\x04\x05\0\x02\x08\x12\x03\x11\x02!\n\x0c\n\x05\x05\
    \0\x02\x08\x01\x12\x03\x11\x02\x19\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03\
    \x11\x1c\x20\n\x0b\n\x04\x05\0\x02\t\x12\x03\x12\x02\x20\n\x0c\n\x05\x05\
    \0\x02\t\x01\x12\x03\x12\x02\x18\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\x12\
    \x1b\x1f\n\x0b\n\x04\x05\0\x02\n\x12\x03\x13\x02\x1a\n\x0c\n\x05\x05\0\
    \x02\n\x01\x12\x03\x13\x02\x12\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\x13\
    \x15\x19\n\x0b\n\x04\x05\0\x02\x0b\x12\x03\x14\x02&\n\x0c\n\x05\x05\0\
    \x02\x0b\x01\x12\x03\x14\x02\x1e\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\
    \x14!%\n\x0b\n\x04\x05\0\x02\x0c\x12\x03\x15\x02(\n\x0c\n\x05\x05\0\x02\
    \x0c\x01\x12\x03\x15\x02\x20\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\x03\x15#'\
    \n\x0b\n\x04\x05\0\x02\r\x12\x03\x16\x02#\n\x0c\n\x05\x05\0\x02\r\x01\
    \x12\x03\x16\x02\x1b\n\x0c\n\x05\x05\0\x02\r\x02\x12\x03\x16\x1e\"\n\x0b\
    \n\x04\x05\0\x02\x0e\x12\x03\x17\x02!\n\x0c\n\x05\x05\0\x02\x0e\x01\x12\
    \x03\x17\x02\x19\n\x0c\n\x05\x05\0\x02\x0e\x02\x12\x03\x17\x1c\x20\n\x0b\
    \n\x04\x05\0\x02\x0f\x12\x03\x18\x02\"\n\x0c\n\x05\x05\0\x02\x0f\x01\x12\
    \x03\x18\x02\x1a\n\x0c\n\x05\x05\0\x02\x0f\x02\x12\x03\x18\x1d!\n\x0b\n\
    \x04\x05\0\x02\x10\x12\x03\x19\x02\x19\n\x0c\n\x05\x05\0\x02\x10\x01\x12\
    \x03\x19\x02\x11\n\x0c\n\x05\x05\0\x02\x10\x02\x12\x03\x19\x14\x18\nk\n\
    \x04\x05\0\x02\x11\x12\x03\x1e\x02\x1c\x1aB/\x20Payloads\x20containing\
    \x20values\x20with\x20any\x20PII\x20-\x20runs\x20all\x20PII\x20matchers\
    \n2\x1a\x20PII\x20matchers\x20(2000-2999)\n\n\x0c\n\x05\x05\0\x02\x11\
    \x01\x12\x03\x1e\x02\x14\n\x0c\n\x05\x05\0\x02\x11\x02\x12\x03\x1e\x17\
    \x1b\nC\n\x04\x05\0\x02\x12\x12\x03\x20\x02$\x1a6\x20Payloads\x20contain\
    ing\x20values\x20with\x20a\x20credit\x20card\x20number\n\n\x0c\n\x05\x05\
    \0\x02\x12\x01\x12\x03\x20\x02\x1c\n\x0c\n\x05\x05\0\x02\x12\x02\x12\x03\
    \x20\x1f#\nG\n\x04\x05\0\x02\x13\x12\x03\"\x02\x1c\x1a:\x20Payloads\x20c\
    ontaining\x20values\x20with\x20a\x20social\x20security\x20number\n\n\x0c\
    \n\x05\x05\0\x02\x13\x01\x12\x03\"\x02\x14\n\x0c\n\x05\x05\0\x02\x13\x02\
    \x12\x03\"\x17\x1b\n?\n\x04\x05\0\x02\x14\x12\x03$\x02\x1e\x1a2\x20Paylo\
    ads\x20containing\x20values\x20with\x20an\x20email\x20address\n\n\x0c\n\
    \x05\x05\0\x02\x14\x01\x12\x03$\x02\x16\n\x0c\n\x05\x05\0\x02\x14\x02\
    \x12\x03$\x19\x1d\n=\n\x04\x05\0\x02\x15\x12\x03&\x02\x1e\x1a0\x20Payloa\
    ds\x20containing\x20values\x20with\x20a\x20phone\x20number\n\n\x0c\n\x05\
    \x05\0\x02\x15\x01\x12\x03&\x02\x16\n\x0c\n\x05\x05\0\x02\x15\x02\x12\
    \x03&\x19\x1d\nA\n\x04\x05\0\x02\x16\x12\x03(\x02'\x1a4\x20Payloads\x20c\
    ontaining\x20values\x20with\x20a\x20driver's\x20license\n\n\x0c\n\x05\
    \x05\0\x02\x16\x01\x12\x03(\x02\x1f\n\x0c\n\x05\x05\0\x02\x16\x02\x12\
    \x03(\"&\n<\n\x04\x05\0\x02\x17\x12\x03*\x02$\x1a/\x20Payloads\x20contai\
    ning\x20values\x20with\x20a\x20passport\x20ID\n\n\x0c\n\x05\x05\0\x02\
    \x17\x01\x12\x03*\x02\x1c\n\x0c\n\x05\x05\0\x02\x17\x02\x12\x03*\x1f#\n;\
    \n\x04\x05\0\x02\x18\x12\x03,\x02#\x1a.\x20Payloads\x20containing\x20val\
    ues\x20with\x20a\x20VIN\x20number\n\n\x0c\n\x05\x05\0\x02\x18\x01\x12\
    \x03,\x02\x1b\n\x0c\n\x05\x05\0\x02\x18\x02\x12\x03,\x1e\"\nL\n\x04\x05\
    \0\x02\x19\x12\x03.\x02&\x1a?\x20Payloads\x20containing\x20values\x20wit\
    h\x20various\x20serial\x20number\x20formats\n\n\x0c\n\x05\x05\0\x02\x19\
    \x01\x12\x03.\x02\x1e\n\x0c\n\x05\x05\0\x02\x19\x02\x12\x03.!%\n\x8e\x01\
    \n\x04\x05\0\x02\x1a\x12\x030\x02\x1e\x1a\x80\x01\x20Payloads\x20contain\
    ing\x20fields\x20named\x20\"login\",\x20\"username\",\x20\"user\",\x20\"\
    userid\",\x20\"user_id\",\x20\"user\",\x20\"password\",\x20\"pass\",\x20\
    \"passwd\",\x20\"pwd\"\n\n\x0c\n\x05\x05\0\x02\x1a\x01\x12\x030\x02\x16\
    \n\x0c\n\x05\x05\0\x02\x1a\x02\x12\x030\x19\x1d\n^\n\x04\x05\0\x02\x1b\
    \x12\x032\x02$\x1aQ\x20Payloads\x20containing\x20fields\x20named\x20\"ta\
    xpayer_id\",\x20\"tax_id\",\x20\"taxpayerid\",\x20\"taxid\"\n\n\x0c\n\
    \x05\x05\0\x02\x1b\x01\x12\x032\x02\x1c\n\x0c\n\x05\x05\0\x02\x1b\x02\
    \x12\x032\x1f#\n}\n\x04\x05\0\x02\x1c\x12\x034\x02\x20\x1ap\x20Payloads\
    \x20containing\x20fields\x20named\x20\"address\",\x20\"street\",\x20\"ci\
    ty\",\x20\"state\",\x20\"zip\",\x20\"zipcode\",\x20\"zip_code\",\x20\"co\
    untry\"\n\n\x0c\n\x05\x05\0\x02\x1c\x01\x12\x034\x02\x18\n\x0c\n\x05\x05\
    \0\x02\x1c\x02\x12\x034\x1b\x1f\n|\n\x04\x05\0\x02\x1d\x12\x036\x02\"\
    \x1ao\x20Payloads\x20containing\x20fields\x20named\x20\"signature\",\x20\
    \"signature_image\",\x20\"signature_image_url\",\x20\"signature_image_ur\
    i\"\n\n\x0c\n\x05\x05\0\x02\x1d\x01\x12\x036\x02\x1a\n\x0c\n\x05\x05\0\
    \x02\x1d\x02\x12\x036\x1d!\ny\n\x04\x05\0\x02\x1e\x12\x038\x02$\x1al\x20\
    Payloads\x20containing\x20values\x20that\x20contain\x20GPS\x20data\x20or\
    \x20coordinates\x20like\x20\"lat\",\x20\"lon\",\x20\"latitude\",\x20\"lo\
    ngitude\"\n\n\x0c\n\x05\x05\0\x02\x1e\x01\x12\x038\x02\x1c\n\x0c\n\x05\
    \x05\0\x02\x1e\x02\x12\x038\x1f#\n]\n\x04\x05\0\x02\x1f\x12\x03:\x02\"\
    \x1aP\x20Payloads\x20containing\x20fields\x20like\x20\"school\",\x20\"un\
    iversity\",\x20\"college\",\x20\"education\"\n\n\x0c\n\x05\x05\0\x02\x1f\
    \x01\x12\x03:\x02\x1a\n\x0c\n\x05\x05\0\x02\x1f\x02\x12\x03:\x1d!\nk\n\
    \x04\x05\0\x02\x20\x12\x03<\x02\"\x1a^\x20Payloads\x20containing\x20fiel\
    ds\x20like\x20\"account\",\x20\"bank\",\x20\"credit\",\x20\"debit\",\x20\
    \"financial\",\x20\"finance\"\n\n\x0c\n\x05\x05\0\x02\x20\x01\x12\x03<\
    \x02\x1a\n\x0c\n\x05\x05\0\x02\x20\x02\x12\x03<\x1d!\nj\n\x04\x05\0\x02!\
    \x12\x03>\x02\x1f\x1a]\x20Payloads\x20containing\x20fields\x20like\x20\"\
    patient\",\x20\"health\",\x20\"healthcare\",\x20\"health\x20care\",\x20\
    \"medical\"\n\n\x0c\n\x05\x05\0\x02!\x01\x12\x03>\x02\x17\n\x0c\n\x05\
    \x05\0\x02!\x02\x12\x03>\x1a\x1e\n+\n\x04\x05\0\x02\"\x12\x03B\x02$2\x1e\
    \x20Numeric\x20matchers\x20(3000-3999)\n\n\x0c\n\x05\x05\0\x02\"\x01\x12\
    \x03B\x02\x1d\n\x0c\n\x05\x05\0\x02\"\x02\x12\x03B\x1f#\n\x0b\n\x04\x05\
    \0\x02#\x12\x03C\x02)\n\x0c\n\x05\x05\0\x02#\x01\x12\x03C\x02!\n\x0c\n\
    \x05\x05\0\x02#\x02\x12\x03C$(\n\x0b\n\x04\x05\0\x02$\x12\x03D\x02*\n\
    \x0c\n\x05\x05\0\x02$\x01\x12\x03D\x02\"\n\x0c\n\x05\x05\0\x02$\x02\x12\
    \x03D%)\n\x0b\n\x04\x05\0\x02%\x12\x03E\x02&\n\x0c\n\x05\x05\0\x02%\x01\
    \x12\x03E\x02\x1e\n\x0c\n\x05\x05\0\x02%\x02\x12\x03E!%\n\x0b\n\x04\x05\
    \0\x02&\x12\x03F\x02'\n\x0c\n\x05\x05\0\x02&\x01\x12\x03F\x02\x1f\n\x0c\
    \n\x05\x05\0\x02&\x02\x12\x03F\"&\n\n\n\x02\x04\0\x12\x04I\0O\x01\n\n\n\
    \x03\x04\0\x01\x12\x03I\x08\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03J\x02\x11\
    \n\x0c\n\x05\x04\0\x02\0\x05\x12\x03J\x02\x07\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03J\x08\x0c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03J\x0f\x10\n\x0b\
    \n\x04\x04\0\x02\x01\x12\x03K\x02\x12\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03K\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03K\t\r\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03K\x10\x11\n,\n\x04\x04\0\x02\x02\x12\x03L\x02\x1b\
    \"\x1f\x20args\x20determined\x20by\x20match_type\n\n\x0c\n\x05\x04\0\x02\
    \x02\x04\x12\x03L\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03L\x0b\x11\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03L\x12\x16\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x03L\x19\x1a\n\x0b\n\x04\x04\0\x02\x03\x12\x03M\x02\x12\n\x0c\n\
    \x05\x04\0\x02\x03\x05\x12\x03M\x02\x06\n\x0c\n\x05\x04\0\x02\x03\x01\
    \x12\x03M\x07\r\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03M\x10\x11\n\x0b\n\
    \x04\x04\0\x02\x04\x12\x03N\x02\x15\n\x0c\n\x05\x04\0\x02\x04\x06\x12\
    \x03N\x02\x0b\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03N\x0c\x10\n\x0c\n\x05\
    \x04\0\x02\x04\x03\x12\x03N\x13\x14\n\n\n\x02\x04\x01\x12\x04Q\0T\x01\n\
    \n\n\x03\x04\x01\x01\x12\x03Q\x08\x15\n\x0b\n\x04\x04\x01\x02\0\x12\x03R\
    \x02\x14\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03R\x02\x06\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03R\x07\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03R\
    \x12\x13\n\x0b\n\x04\x04\x01\x02\x01\x12\x03S\x02\x13\n\x0c\n\x05\x04\
    \x01\x02\x01\x05\x12\x03S\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\
    \x03S\t\x0e\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03S\x11\x12b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(MatchRequest::generated_message_descriptor_data());
            messages.push(MatchResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(MatchType::generated_enum_descriptor_data());
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
