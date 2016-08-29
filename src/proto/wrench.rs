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
pub struct Wrench {
    // message fields
    force: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    torque: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    force_offset: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Wrench {}

impl Wrench {
    pub fn new() -> Wrench {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Wrench {
        static mut instance: ::protobuf::lazy::Lazy<Wrench> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Wrench,
        };
        unsafe {
            instance.get(|| {
                Wrench {
                    force: ::protobuf::SingularPtrField::none(),
                    torque: ::protobuf::SingularPtrField::none(),
                    force_offset: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Vector3d force = 1;

    pub fn clear_force(&mut self) {
        self.force.clear();
    }

    pub fn has_force(&self) -> bool {
        self.force.is_some()
    }

    // Param is passed by value, moved
    pub fn set_force(&mut self, v: super::vector3d::Vector3d) {
        self.force = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_force(&mut self) -> &mut super::vector3d::Vector3d {
        if self.force.is_none() {
            self.force.set_default();
        };
        self.force.as_mut().unwrap()
    }

    // Take field
    pub fn take_force(&mut self) -> super::vector3d::Vector3d {
        self.force.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_force(&self) -> &super::vector3d::Vector3d {
        self.force.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // required .gazebo.msgs.Vector3d torque = 2;

    pub fn clear_torque(&mut self) {
        self.torque.clear();
    }

    pub fn has_torque(&self) -> bool {
        self.torque.is_some()
    }

    // Param is passed by value, moved
    pub fn set_torque(&mut self, v: super::vector3d::Vector3d) {
        self.torque = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_torque(&mut self) -> &mut super::vector3d::Vector3d {
        if self.torque.is_none() {
            self.torque.set_default();
        };
        self.torque.as_mut().unwrap()
    }

    // Take field
    pub fn take_torque(&mut self) -> super::vector3d::Vector3d {
        self.torque.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_torque(&self) -> &super::vector3d::Vector3d {
        self.torque.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // optional .gazebo.msgs.Vector3d force_offset = 3;

    pub fn clear_force_offset(&mut self) {
        self.force_offset.clear();
    }

    pub fn has_force_offset(&self) -> bool {
        self.force_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_force_offset(&mut self, v: super::vector3d::Vector3d) {
        self.force_offset = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_force_offset(&mut self) -> &mut super::vector3d::Vector3d {
        if self.force_offset.is_none() {
            self.force_offset.set_default();
        };
        self.force_offset.as_mut().unwrap()
    }

    // Take field
    pub fn take_force_offset(&mut self) -> super::vector3d::Vector3d {
        self.force_offset.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_force_offset(&self) -> &super::vector3d::Vector3d {
        self.force_offset.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }
}

impl ::protobuf::Message for Wrench {
    fn is_initialized(&self) -> bool {
        if self.force.is_none() {
            return false;
        };
        if self.torque.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.force));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.torque));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.force_offset));
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
        for value in &self.force {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.torque {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.force_offset {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.force.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.torque.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.force_offset.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Wrench>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Wrench {
    fn new() -> Wrench {
        Wrench::new()
    }

    fn descriptor_static(_: ::std::option::Option<Wrench>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "force",
                    Wrench::has_force,
                    Wrench::get_force,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "torque",
                    Wrench::has_torque,
                    Wrench::get_torque,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "force_offset",
                    Wrench::has_force_offset,
                    Wrench::get_force_offset,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Wrench>(
                    "Wrench",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Wrench {
    fn clear(&mut self) {
        self.clear_force();
        self.clear_torque();
        self.clear_force_offset();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Wrench {
    fn eq(&self, other: &Wrench) -> bool {
        self.force == other.force &&
        self.torque == other.torque &&
        self.force_offset == other.force_offset &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Wrench {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x77, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0e, 0x76, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x82, 0x01, 0x0a, 0x06,
    0x57, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x12, 0x24, 0x0a, 0x05, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x12, 0x25, 0x0a, 0x06,
    0x74, 0x6f, 0x72, 0x71, 0x75, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x33, 0x64, 0x12, 0x2b, 0x0a, 0x0c, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x5f, 0x6f, 0x66, 0x66,
    0x73, 0x65, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65,
    0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64,
    0x4a, 0x84, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x17,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0b, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x14, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x0c, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0c, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c,
    0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0d, 0x23, 0x24,
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
