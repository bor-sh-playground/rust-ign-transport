// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Publish {
    // message fields
    topic: ::protobuf::SingularField<::std::string::String>,
    msg_type: ::protobuf::SingularField<::std::string::String>,
    host: ::protobuf::SingularField<::std::string::String>,
    port: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Publish {}

impl Publish {
    pub fn new() -> Publish {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Publish {
        static mut instance: ::protobuf::lazy::Lazy<Publish> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Publish,
        };
        unsafe {
            instance.get(|| {
                Publish {
                    topic: ::protobuf::SingularField::none(),
                    msg_type: ::protobuf::SingularField::none(),
                    host: ::protobuf::SingularField::none(),
                    port: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string topic = 1;

    pub fn clear_topic(&mut self) {
        self.topic.clear();
    }

    pub fn has_topic(&self) -> bool {
        self.topic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_topic(&mut self, v: ::std::string::String) {
        self.topic = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_topic(&mut self) -> &mut ::std::string::String {
        if self.topic.is_none() {
            self.topic.set_default();
        };
        self.topic.as_mut().unwrap()
    }

    // Take field
    pub fn take_topic(&mut self) -> ::std::string::String {
        self.topic.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_topic(&self) -> &str {
        match self.topic.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string msg_type = 2;

    pub fn clear_msg_type(&mut self) {
        self.msg_type.clear();
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: ::std::string::String) {
        self.msg_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_type(&mut self) -> &mut ::std::string::String {
        if self.msg_type.is_none() {
            self.msg_type.set_default();
        };
        self.msg_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg_type(&mut self) -> ::std::string::String {
        self.msg_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg_type(&self) -> &str {
        match self.msg_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string host = 3;

    pub fn clear_host(&mut self) {
        self.host.clear();
    }

    pub fn has_host(&self) -> bool {
        self.host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host(&mut self) -> &mut ::std::string::String {
        if self.host.is_none() {
            self.host.set_default();
        };
        self.host.as_mut().unwrap()
    }

    // Take field
    pub fn take_host(&mut self) -> ::std::string::String {
        self.host.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_host(&self) -> &str {
        match self.host.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint32 port = 4;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(0)
    }
}

impl ::protobuf::Message for Publish {
    fn is_initialized(&self) -> bool {
        if self.topic.is_none() {
            return false;
        };
        if self.msg_type.is_none() {
            return false;
        };
        if self.host.is_none() {
            return false;
        };
        if self.port.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.topic));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg_type));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.host));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.port = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.topic {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.msg_type {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.host {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.port {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.topic.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.msg_type.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.host.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.port {
            try!(os.write_uint32(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Publish>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Publish {
    fn new() -> Publish {
        Publish::new()
    }

    fn descriptor_static(_: ::std::option::Option<Publish>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "topic",
                    Publish::has_topic,
                    Publish::get_topic,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "msg_type",
                    Publish::has_msg_type,
                    Publish::get_msg_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "host",
                    Publish::has_host,
                    Publish::get_host,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "port",
                    Publish::has_port,
                    Publish::get_port,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Publish>(
                    "Publish",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Publish {
    fn clear(&mut self) {
        self.clear_topic();
        self.clear_msg_type();
        self.clear_host();
        self.clear_port();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Publish {
    fn eq(&self, other: &Publish) -> bool {
        self.topic == other.topic &&
        self.msg_type == other.msg_type &&
        self.host == other.host &&
        self.port == other.port &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Publish {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0x46, 0x0a, 0x07,
    0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x6f, 0x70, 0x69, 0x63,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x68, 0x6f, 0x73, 0x74,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x04,
    0x20, 0x02, 0x28, 0x0d, 0x4a, 0xbe, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0d, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x07, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x09, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x0a, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x1d, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x0c, 0x1d, 0x1e,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
