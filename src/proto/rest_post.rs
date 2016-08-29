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
pub struct RestPost {
    // message fields
    id: ::std::option::Option<u32>,
    route: ::protobuf::SingularField<::std::string::String>,
    json: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RestPost {}

impl RestPost {
    pub fn new() -> RestPost {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RestPost {
        static mut instance: ::protobuf::lazy::Lazy<RestPost> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RestPost,
        };
        unsafe {
            instance.get(|| {
                RestPost {
                    id: ::std::option::Option::None,
                    route: ::protobuf::SingularField::none(),
                    json: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // required string route = 2;

    pub fn clear_route(&mut self) {
        self.route.clear();
    }

    pub fn has_route(&self) -> bool {
        self.route.is_some()
    }

    // Param is passed by value, moved
    pub fn set_route(&mut self, v: ::std::string::String) {
        self.route = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_route(&mut self) -> &mut ::std::string::String {
        if self.route.is_none() {
            self.route.set_default();
        };
        self.route.as_mut().unwrap()
    }

    // Take field
    pub fn take_route(&mut self) -> ::std::string::String {
        self.route.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_route(&self) -> &str {
        match self.route.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string json = 3;

    pub fn clear_json(&mut self) {
        self.json.clear();
    }

    pub fn has_json(&self) -> bool {
        self.json.is_some()
    }

    // Param is passed by value, moved
    pub fn set_json(&mut self, v: ::std::string::String) {
        self.json = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_json(&mut self) -> &mut ::std::string::String {
        if self.json.is_none() {
            self.json.set_default();
        };
        self.json.as_mut().unwrap()
    }

    // Take field
    pub fn take_json(&mut self) -> ::std::string::String {
        self.json.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_json(&self) -> &str {
        match self.json.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for RestPost {
    fn is_initialized(&self) -> bool {
        if self.route.is_none() {
            return false;
        };
        if self.json.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.route));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.json));
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
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.route {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.json {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.route.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.json.as_ref() {
            try!(os.write_string(3, &v));
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
        ::std::any::TypeId::of::<RestPost>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RestPost {
    fn new() -> RestPost {
        RestPost::new()
    }

    fn descriptor_static(_: ::std::option::Option<RestPost>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    RestPost::has_id,
                    RestPost::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "route",
                    RestPost::has_route,
                    RestPost::get_route,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "json",
                    RestPost::has_json,
                    RestPost::get_json,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RestPost>(
                    "RestPost",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RestPost {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_route();
        self.clear_json();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RestPost {
    fn eq(&self, other: &RestPost) -> bool {
        self.id == other.id &&
        self.route == other.route &&
        self.json == other.json &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RestPost {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x72, 0x65, 0x73, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0x33,
    0x0a, 0x08, 0x52, 0x65, 0x73, 0x74, 0x50, 0x6f, 0x73, 0x74, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0d, 0x0a, 0x05, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x6a, 0x73, 0x6f, 0x6e, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x09, 0x4a, 0xe3, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x11, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x07, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x10,
    0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x1c, 0x1a, 0x24, 0x2f,
    0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x49, 0x44, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x1a, 0x1b, 0x0a, 0x28, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x1c, 0x1a, 0x1b, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69,
    0x65, 0x66, 0x20, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x6f, 0x73, 0x74,
    0x20, 0x74, 0x6f, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x1a, 0x1b, 0x0a, 0x32, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x02, 0x1c, 0x1a, 0x25, 0x2f, 0x20, 0x5c, 0x62,
    0x72, 0x69, 0x65, 0x66, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x6f, 0x73,
    0x74, 0x20, 0x69, 0x6e, 0x20, 0x4a, 0x53, 0x4f, 0x4e, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x1a, 0x1b,
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
