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
pub struct RestLogout {
    // message fields
    id: ::std::option::Option<u32>,
    url: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RestLogout {}

impl RestLogout {
    pub fn new() -> RestLogout {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RestLogout {
        static mut instance: ::protobuf::lazy::Lazy<RestLogout> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RestLogout,
        };
        unsafe {
            instance.get(|| {
                RestLogout {
                    id: ::std::option::Option::None,
                    url: ::protobuf::SingularField::none(),
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

    // optional string url = 2;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for RestLogout {
    fn is_initialized(&self) -> bool {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url));
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
        for value in &self.url {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.url.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<RestLogout>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RestLogout {
    fn new() -> RestLogout {
        RestLogout::new()
    }

    fn descriptor_static(_: ::std::option::Option<RestLogout>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    RestLogout::has_id,
                    RestLogout::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "url",
                    RestLogout::has_url,
                    RestLogout::get_url,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RestLogout>(
                    "RestLogout",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RestLogout {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_url();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RestLogout {
    fn eq(&self, other: &RestLogout) -> bool {
        self.id == other.id &&
        self.url == other.url &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RestLogout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x72, 0x65, 0x73, 0x74, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x22, 0x25, 0x0a, 0x0a, 0x52, 0x65, 0x73, 0x74, 0x4c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x12, 0x0a,
    0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0b, 0x0a, 0x03, 0x75, 0x72,
    0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x4a, 0xf9, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x0d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x06, 0x08, 0x12, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02,
    0x1f, 0x1a, 0x24, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x49, 0x44, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1d, 0x1e, 0x0a,
    0x2a, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x1f, 0x1a, 0x1d, 0x2f, 0x20,
    0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x65, 0x62, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x75, 0x72, 0x6c, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0c, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0c, 0x1d, 0x1e,
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
