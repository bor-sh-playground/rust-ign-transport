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
pub struct Magnetometer {
    // message fields
    time: ::protobuf::SingularPtrField<super::time::Time>,
    field_tesla: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Magnetometer {}

impl Magnetometer {
    pub fn new() -> Magnetometer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Magnetometer {
        static mut instance: ::protobuf::lazy::Lazy<Magnetometer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Magnetometer,
        };
        unsafe {
            instance.get(|| {
                Magnetometer {
                    time: ::protobuf::SingularPtrField::none(),
                    field_tesla: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Time time = 1;

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

    // required .gazebo.msgs.Vector3d field_tesla = 2;

    pub fn clear_field_tesla(&mut self) {
        self.field_tesla.clear();
    }

    pub fn has_field_tesla(&self) -> bool {
        self.field_tesla.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_tesla(&mut self, v: super::vector3d::Vector3d) {
        self.field_tesla = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_tesla(&mut self) -> &mut super::vector3d::Vector3d {
        if self.field_tesla.is_none() {
            self.field_tesla.set_default();
        };
        self.field_tesla.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_tesla(&mut self) -> super::vector3d::Vector3d {
        self.field_tesla.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_field_tesla(&self) -> &super::vector3d::Vector3d {
        self.field_tesla.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }
}

impl ::protobuf::Message for Magnetometer {
    fn is_initialized(&self) -> bool {
        if self.time.is_none() {
            return false;
        };
        if self.field_tesla.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.time));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field_tesla));
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
        for value in &self.time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.field_tesla {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.field_tesla.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Magnetometer>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Magnetometer {
    fn new() -> Magnetometer {
        Magnetometer::new()
    }

    fn descriptor_static(_: ::std::option::Option<Magnetometer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "time",
                    Magnetometer::has_time,
                    Magnetometer::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "field_tesla",
                    Magnetometer::has_field_tesla,
                    Magnetometer::get_field_tesla,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Magnetometer>(
                    "Magnetometer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Magnetometer {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_field_tesla();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Magnetometer {
    fn eq(&self, other: &Magnetometer) -> bool {
        self.time == other.time &&
        self.field_tesla == other.field_tesla &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Magnetometer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x6d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x6f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x1a, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x76,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5b, 0x0a,
    0x0c, 0x4d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x6f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x12, 0x1f, 0x0a,
    0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x2a,
    0x0a, 0x0b, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x74, 0x65, 0x73, 0x6c, 0x61, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x4a, 0x9c, 0x03, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x11, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x07, 0x07, 0x17, 0x0a, 0x50, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x11,
    0x01, 0x1a, 0x44, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x65, 0x6e, 0x63, 0x61, 0x70, 0x73, 0x75,
    0x6c, 0x61, 0x74, 0x65, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x20, 0x6d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x6f,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x0a, 0x08, 0x14, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x29,
    0x1a, 0x46, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x47, 0x6c, 0x6f, 0x62, 0x61,
    0x6c, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x61, 0x74, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x69, 0x63, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x20, 0x73, 0x74, 0x72, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20, 0x77, 0x61, 0x73, 0x20,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x0d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d,
    0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x27, 0x28,
    0x0a, 0x4f, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x10, 0x02, 0x29, 0x1a, 0x42, 0x2f,
    0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x69, 0x63,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x73, 0x74, 0x72, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20,
    0x28, 0x69, 0x6e, 0x20, 0x54, 0x65, 0x73, 0x6c, 0x61, 0x29, 0x20, 0x61, 0x6c, 0x6f, 0x6e, 0x67,
    0x20, 0x62, 0x6f, 0x64, 0x79, 0x2d, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x20, 0x61, 0x78, 0x69, 0x73,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x10, 0x0b, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x27, 0x28,
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
