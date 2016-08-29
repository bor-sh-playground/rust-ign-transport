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
pub struct Packet {
    // message fields
    stamp: ::protobuf::SingularPtrField<super::time::Time>,
    field_type: ::protobuf::SingularField<::std::string::String>,
    serialized_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Packet {}

impl Packet {
    pub fn new() -> Packet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Packet {
        static mut instance: ::protobuf::lazy::Lazy<Packet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Packet,
        };
        unsafe {
            instance.get(|| {
                Packet {
                    stamp: ::protobuf::SingularPtrField::none(),
                    field_type: ::protobuf::SingularField::none(),
                    serialized_data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Time stamp = 1;

    pub fn clear_stamp(&mut self) {
        self.stamp.clear();
    }

    pub fn has_stamp(&self) -> bool {
        self.stamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stamp(&mut self, v: super::time::Time) {
        self.stamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stamp(&mut self) -> &mut super::time::Time {
        if self.stamp.is_none() {
            self.stamp.set_default();
        };
        self.stamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_stamp(&mut self) -> super::time::Time {
        self.stamp.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_stamp(&self) -> &super::time::Time {
        self.stamp.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // required string type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required bytes serialized_data = 3;

    pub fn clear_serialized_data(&mut self) {
        self.serialized_data.clear();
    }

    pub fn has_serialized_data(&self) -> bool {
        self.serialized_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serialized_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.serialized_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serialized_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.serialized_data.is_none() {
            self.serialized_data.set_default();
        };
        self.serialized_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_serialized_data(&mut self) -> ::std::vec::Vec<u8> {
        self.serialized_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_serialized_data(&self) -> &[u8] {
        match self.serialized_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Packet {
    fn is_initialized(&self) -> bool {
        if self.stamp.is_none() {
            return false;
        };
        if self.field_type.is_none() {
            return false;
        };
        if self.serialized_data.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stamp));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.serialized_data));
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
        for value in &self.stamp {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.serialized_data {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stamp.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.serialized_data.as_ref() {
            try!(os.write_bytes(3, &v));
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
        ::std::any::TypeId::of::<Packet>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Packet {
    fn new() -> Packet {
        Packet::new()
    }

    fn descriptor_static(_: ::std::option::Option<Packet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stamp",
                    Packet::has_stamp,
                    Packet::get_stamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "type",
                    Packet::has_field_type,
                    Packet::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "serialized_data",
                    Packet::has_serialized_data,
                    Packet::get_serialized_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Packet>(
                    "Packet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Packet {
    fn clear(&mut self) {
        self.clear_stamp();
        self.clear_field_type();
        self.clear_serialized_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        self.stamp == other.stamp &&
        self.field_type == other.field_type &&
        self.serialized_data == other.serialized_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x74, 0x69, 0x6d,
    0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x51, 0x0a, 0x06, 0x50, 0x61, 0x63, 0x6b, 0x65,
    0x74, 0x12, 0x20, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54,
    0x69, 0x6d, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x17, 0x0a, 0x0f, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f,
    0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x4a, 0x84, 0x02, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x09, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09,
    0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x0c, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x0d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x0d, 0x11, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x23,
    0x24,
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
