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
pub struct Tactile {
    // message fields
    collision_name: ::protobuf::RepeatedField<::std::string::String>,
    collision_id: ::std::vec::Vec<u32>,
    pressure: ::std::vec::Vec<f64>,
    time: ::protobuf::SingularPtrField<super::time::Time>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tactile {}

impl Tactile {
    pub fn new() -> Tactile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tactile {
        static mut instance: ::protobuf::lazy::Lazy<Tactile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tactile,
        };
        unsafe {
            instance.get(|| {
                Tactile {
                    collision_name: ::protobuf::RepeatedField::new(),
                    collision_id: ::std::vec::Vec::new(),
                    pressure: ::std::vec::Vec::new(),
                    time: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string collision_name = 1;

    pub fn clear_collision_name(&mut self) {
        self.collision_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_collision_name(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.collision_name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_collision_name(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.collision_name
    }

    // Take field
    pub fn take_collision_name(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.collision_name, ::protobuf::RepeatedField::new())
    }

    pub fn get_collision_name(&self) -> &[::std::string::String] {
        &self.collision_name
    }

    // repeated uint32 collision_id = 2;

    pub fn clear_collision_id(&mut self) {
        self.collision_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_collision_id(&mut self, v: ::std::vec::Vec<u32>) {
        self.collision_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_collision_id(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.collision_id
    }

    // Take field
    pub fn take_collision_id(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.collision_id, ::std::vec::Vec::new())
    }

    pub fn get_collision_id(&self) -> &[u32] {
        &self.collision_id
    }

    // repeated double pressure = 3;

    pub fn clear_pressure(&mut self) {
        self.pressure.clear();
    }

    // Param is passed by value, moved
    pub fn set_pressure(&mut self, v: ::std::vec::Vec<f64>) {
        self.pressure = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pressure(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.pressure
    }

    // Take field
    pub fn take_pressure(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.pressure, ::std::vec::Vec::new())
    }

    pub fn get_pressure(&self) -> &[f64] {
        &self.pressure
    }

    // required .gazebo.msgs.Time time = 4;

    pub fn clear_time(&mut self) {
        self.time.clear();
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: super::time::Time) {
        self.time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_time(&mut self) -> &mut super::time::Time {
        if self.time.is_none() {
            self.time.set_default();
        };
        self.time.as_mut().unwrap()
    }

    // Take field
    pub fn take_time(&mut self) -> super::time::Time {
        self.time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_time(&self) -> &super::time::Time {
        self.time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }
}

impl ::protobuf::Message for Tactile {
    fn is_initialized(&self) -> bool {
        if self.time.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.collision_name));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.collision_id));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.pressure));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.time));
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
        for value in &self.collision_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.collision_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 9 * self.pressure.len() as u32;
        for value in &self.time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.collision_name {
            try!(os.write_string(1, &v));
        };
        for v in &self.collision_id {
            try!(os.write_uint32(2, *v));
        };
        for v in &self.pressure {
            try!(os.write_double(3, *v));
        };
        if let Some(v) = self.time.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<Tactile>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tactile {
    fn new() -> Tactile {
        Tactile::new()
    }

    fn descriptor_static(_: ::std::option::Option<Tactile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "collision_name",
                    Tactile::get_collision_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "collision_id",
                    Tactile::get_collision_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "pressure",
                    Tactile::get_pressure,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "time",
                    Tactile::has_time,
                    Tactile::get_time,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Tactile>(
                    "Tactile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Tactile {
    fn clear(&mut self) {
        self.clear_collision_name();
        self.clear_collision_id();
        self.clear_pressure();
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Tactile {
    fn eq(&self, other: &Tactile) -> bool {
        self.collision_name == other.collision_name &&
        self.collision_id == other.collision_id &&
        self.pressure == other.pressure &&
        self.time == other.time &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Tactile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x74, 0x61, 0x63, 0x74, 0x69, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x74, 0x69,
    0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x6a, 0x0a, 0x07, 0x54, 0x61, 0x63, 0x74,
    0x69, 0x6c, 0x65, 0x12, 0x16, 0x0a, 0x0e, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f, 0x6e,
    0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x63,
    0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0d, 0x12, 0x10, 0x0a, 0x08, 0x70, 0x72, 0x65, 0x73, 0x73, 0x75, 0x72, 0x65, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x01, 0x12, 0x1f, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x54, 0x69, 0x6d, 0x65, 0x4a, 0xc9, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x06, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0e, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0a, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x0c, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x0d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d,
    0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x23, 0x24,
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
