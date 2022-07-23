// This file is generated by rust-protobuf 2.27.1. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `monitor.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct PingRequest {
    // message fields
    pub request_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PingRequest {
    fn default() -> &'a PingRequest {
        <PingRequest as ::protobuf::Message>::default_instance()
    }
}

impl PingRequest {
    pub fn new() -> PingRequest {
        ::std::default::Default::default()
    }

    // .google.protobuf.Timestamp request_time = 1;


    pub fn get_request_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.request_time.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_request_time(&mut self) {
        self.request_time.clear();
    }

    pub fn has_request_time(&self) -> bool {
        self.request_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.request_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.request_time.is_none() {
            self.request_time.set_default();
        }
        self.request_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_request_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.request_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }
}

impl ::protobuf::Message for PingRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.request_time {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request_time)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.request_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.request_time.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PingRequest {
        PingRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "request_time",
                |m: &PingRequest| { &m.request_time },
                |m: &mut PingRequest| { &mut m.request_time },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PingRequest>(
                "PingRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PingRequest {
        static instance: ::protobuf::rt::LazyV2<PingRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PingRequest::new)
    }
}

impl ::protobuf::Clear for PingRequest {
    fn clear(&mut self) {
        self.request_time.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PingResponse {
    // message fields
    pub response_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub version: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PingResponse {
    fn default() -> &'a PingResponse {
        <PingResponse as ::protobuf::Message>::default_instance()
    }
}

impl PingResponse {
    pub fn new() -> PingResponse {
        ::std::default::Default::default()
    }

    // .google.protobuf.Timestamp response_time = 1;


    pub fn get_response_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.response_time.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_response_time(&mut self) {
        self.response_time.clear();
    }

    pub fn has_response_time(&self) -> bool {
        self.response_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.response_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.response_time.is_none() {
            self.response_time.set_default();
        }
        self.response_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_response_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.response_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // string version = 2;


    pub fn get_version(&self) -> &str {
        &self.version
    }
    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }
}

impl ::protobuf::Message for PingResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.response_time {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response_time)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.response_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.response_time.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.version.is_empty() {
            os.write_string(2, &self.version)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PingResponse {
        PingResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "response_time",
                |m: &PingResponse| { &m.response_time },
                |m: &mut PingResponse| { &mut m.response_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "version",
                |m: &PingResponse| { &m.version },
                |m: &mut PingResponse| { &mut m.version },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PingResponse>(
                "PingResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PingResponse {
        static instance: ::protobuf::rt::LazyV2<PingResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PingResponse::new)
    }
}

impl ::protobuf::Clear for PingResponse {
    fn clear(&mut self) {
        self.response_time.clear();
        self.version.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmonitor.proto\x1a\x1fgoogle/protobuf/timestamp.proto\"L\n\x0bPingReq\
    uest\x12=\n\x0crequest_time\x18\x01\x20\x01(\x0b2\x1a.google.protobuf.Ti\
    mestampR\x0brequestTime\"i\n\x0cPingResponse\x12?\n\rresponse_time\x18\
    \x01\x20\x01(\x0b2\x1a.google.protobuf.TimestampR\x0cresponseTime\x12\
    \x18\n\x07version\x18\x02\x20\x01(\tR\x07version20\n\x07Monitor\x12%\n\
    \x04Ping\x12\x0c.PingRequest\x1a\r.PingResponse\"\0B\x18Z\x16internal/pr\
    oto/monitorb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
