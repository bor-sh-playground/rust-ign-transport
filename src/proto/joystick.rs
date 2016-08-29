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
pub struct Joystick {
    // message fields
    translation: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    rotation: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    buttons: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Joystick {}

impl Joystick {
    pub fn new() -> Joystick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Joystick {
        static mut instance: ::protobuf::lazy::Lazy<Joystick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Joystick,
        };
        unsafe {
            instance.get(|| {
                Joystick {
                    translation: ::protobuf::SingularPtrField::none(),
                    rotation: ::protobuf::SingularPtrField::none(),
                    buttons: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gazebo.msgs.Vector3d translation = 1;

    pub fn clear_translation(&mut self) {
        self.translation.clear();
    }

    pub fn has_translation(&self) -> bool {
        self.translation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_translation(&mut self, v: super::vector3d::Vector3d) {
        self.translation = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_translation(&mut self) -> &mut super::vector3d::Vector3d {
        if self.translation.is_none() {
            self.translation.set_default();
        };
        self.translation.as_mut().unwrap()
    }

    // Take field
    pub fn take_translation(&mut self) -> super::vector3d::Vector3d {
        self.translation.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_translation(&self) -> &super::vector3d::Vector3d {
        self.translation.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // optional .gazebo.msgs.Vector3d rotation = 2;

    pub fn clear_rotation(&mut self) {
        self.rotation.clear();
    }

    pub fn has_rotation(&self) -> bool {
        self.rotation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rotation(&mut self, v: super::vector3d::Vector3d) {
        self.rotation = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rotation(&mut self) -> &mut super::vector3d::Vector3d {
        if self.rotation.is_none() {
            self.rotation.set_default();
        };
        self.rotation.as_mut().unwrap()
    }

    // Take field
    pub fn take_rotation(&mut self) -> super::vector3d::Vector3d {
        self.rotation.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_rotation(&self) -> &super::vector3d::Vector3d {
        self.rotation.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // repeated int32 buttons = 3;

    pub fn clear_buttons(&mut self) {
        self.buttons.clear();
    }

    // Param is passed by value, moved
    pub fn set_buttons(&mut self, v: ::std::vec::Vec<i32>) {
        self.buttons = v;
    }

    // Mutable pointer to the field.
    pub fn mut_buttons(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.buttons
    }

    // Take field
    pub fn take_buttons(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.buttons, ::std::vec::Vec::new())
    }

    pub fn get_buttons(&self) -> &[i32] {
        &self.buttons
    }
}

impl ::protobuf::Message for Joystick {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.translation));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rotation));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.buttons));
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
        for value in &self.translation {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.rotation {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.buttons {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.translation.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.rotation.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.buttons {
            try!(os.write_int32(3, *v));
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
        ::std::any::TypeId::of::<Joystick>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Joystick {
    fn new() -> Joystick {
        Joystick::new()
    }

    fn descriptor_static(_: ::std::option::Option<Joystick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "translation",
                    Joystick::has_translation,
                    Joystick::get_translation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "rotation",
                    Joystick::has_rotation,
                    Joystick::get_rotation,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "buttons",
                    Joystick::get_buttons,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Joystick>(
                    "Joystick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Joystick {
    fn clear(&mut self) {
        self.clear_translation();
        self.clear_rotation();
        self.clear_buttons();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Joystick {
    fn eq(&self, other: &Joystick) -> bool {
        self.translation == other.translation &&
        self.rotation == other.rotation &&
        self.buttons == other.buttons &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Joystick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x6a, 0x6f, 0x79, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0e, 0x76,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x70, 0x0a,
    0x08, 0x4a, 0x6f, 0x79, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x12, 0x2a, 0x0a, 0x0b, 0x74, 0x72, 0x61,
    0x6e, 0x73, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15,
    0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x33, 0x64, 0x12, 0x27, 0x0a, 0x08, 0x72, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f,
    0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x12, 0x0f,
    0x0a, 0x07, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x4a,
    0xed, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x16, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x17, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x75, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0d, 0x02, 0x24, 0x1a, 0x68, 0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x6c, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20,
    0x61, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x78, 0x2c, 0x79, 0x2c, 0x7a, 0x0a,
    0x20, 0x61, 0x78, 0x65, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x73, 0x65, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x6e, 0x6f,
    0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x2d, 0x31, 0x2e, 0x2e, 0x2e, 0x31, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0d, 0x22, 0x23, 0x0a, 0x73, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x12, 0x02, 0x24, 0x1a, 0x66, 0x20, 0x52, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d,
    0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x62, 0x6f, 0x75,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x78, 0x2c, 0x79, 0x2c, 0x7a, 0x20, 0x0a, 0x20, 0x61, 0x78,
    0x65, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x73, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x6e, 0x6f, 0x72, 0x6d, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x20, 0x2d, 0x31, 0x2e, 0x2e, 0x2e, 0x31, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x12, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x12, 0x22, 0x23, 0x0a, 0x22, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x15, 0x02, 0x24,
    0x1a, 0x15, 0x20, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x15, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x22, 0x23,
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
