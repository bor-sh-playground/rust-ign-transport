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
pub struct IMU {
    // message fields
    stamp: ::protobuf::SingularPtrField<super::time::Time>,
    entity_name: ::protobuf::SingularField<::std::string::String>,
    orientation: ::protobuf::SingularPtrField<super::quaternion::Quaternion>,
    angular_velocity: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    linear_acceleration: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IMU {}

impl IMU {
    pub fn new() -> IMU {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IMU {
        static mut instance: ::protobuf::lazy::Lazy<IMU> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IMU,
        };
        unsafe {
            instance.get(|| {
                IMU {
                    stamp: ::protobuf::SingularPtrField::none(),
                    entity_name: ::protobuf::SingularField::none(),
                    orientation: ::protobuf::SingularPtrField::none(),
                    angular_velocity: ::protobuf::SingularPtrField::none(),
                    linear_acceleration: ::protobuf::SingularPtrField::none(),
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

    // required string entity_name = 2;

    pub fn clear_entity_name(&mut self) {
        self.entity_name.clear();
    }

    pub fn has_entity_name(&self) -> bool {
        self.entity_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_name(&mut self, v: ::std::string::String) {
        self.entity_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity_name(&mut self) -> &mut ::std::string::String {
        if self.entity_name.is_none() {
            self.entity_name.set_default();
        };
        self.entity_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity_name(&mut self) -> ::std::string::String {
        self.entity_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_entity_name(&self) -> &str {
        match self.entity_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .gazebo.msgs.Quaternion orientation = 3;

    pub fn clear_orientation(&mut self) {
        self.orientation.clear();
    }

    pub fn has_orientation(&self) -> bool {
        self.orientation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_orientation(&mut self, v: super::quaternion::Quaternion) {
        self.orientation = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_orientation(&mut self) -> &mut super::quaternion::Quaternion {
        if self.orientation.is_none() {
            self.orientation.set_default();
        };
        self.orientation.as_mut().unwrap()
    }

    // Take field
    pub fn take_orientation(&mut self) -> super::quaternion::Quaternion {
        self.orientation.take().unwrap_or_else(|| super::quaternion::Quaternion::new())
    }

    pub fn get_orientation(&self) -> &super::quaternion::Quaternion {
        self.orientation.as_ref().unwrap_or_else(|| super::quaternion::Quaternion::default_instance())
    }

    // required .gazebo.msgs.Vector3d angular_velocity = 4;

    pub fn clear_angular_velocity(&mut self) {
        self.angular_velocity.clear();
    }

    pub fn has_angular_velocity(&self) -> bool {
        self.angular_velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angular_velocity(&mut self, v: super::vector3d::Vector3d) {
        self.angular_velocity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angular_velocity(&mut self) -> &mut super::vector3d::Vector3d {
        if self.angular_velocity.is_none() {
            self.angular_velocity.set_default();
        };
        self.angular_velocity.as_mut().unwrap()
    }

    // Take field
    pub fn take_angular_velocity(&mut self) -> super::vector3d::Vector3d {
        self.angular_velocity.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_angular_velocity(&self) -> &super::vector3d::Vector3d {
        self.angular_velocity.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // required .gazebo.msgs.Vector3d linear_acceleration = 5;

    pub fn clear_linear_acceleration(&mut self) {
        self.linear_acceleration.clear();
    }

    pub fn has_linear_acceleration(&self) -> bool {
        self.linear_acceleration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_linear_acceleration(&mut self, v: super::vector3d::Vector3d) {
        self.linear_acceleration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_linear_acceleration(&mut self) -> &mut super::vector3d::Vector3d {
        if self.linear_acceleration.is_none() {
            self.linear_acceleration.set_default();
        };
        self.linear_acceleration.as_mut().unwrap()
    }

    // Take field
    pub fn take_linear_acceleration(&mut self) -> super::vector3d::Vector3d {
        self.linear_acceleration.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_linear_acceleration(&self) -> &super::vector3d::Vector3d {
        self.linear_acceleration.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }
}

impl ::protobuf::Message for IMU {
    fn is_initialized(&self) -> bool {
        if self.stamp.is_none() {
            return false;
        };
        if self.entity_name.is_none() {
            return false;
        };
        if self.orientation.is_none() {
            return false;
        };
        if self.angular_velocity.is_none() {
            return false;
        };
        if self.linear_acceleration.is_none() {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.entity_name));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.orientation));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angular_velocity));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.linear_acceleration));
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
        for value in &self.entity_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.orientation {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.angular_velocity {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.linear_acceleration {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.entity_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.orientation.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.angular_velocity.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.linear_acceleration.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<IMU>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IMU {
    fn new() -> IMU {
        IMU::new()
    }

    fn descriptor_static(_: ::std::option::Option<IMU>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stamp",
                    IMU::has_stamp,
                    IMU::get_stamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "entity_name",
                    IMU::has_entity_name,
                    IMU::get_entity_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "orientation",
                    IMU::has_orientation,
                    IMU::get_orientation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "angular_velocity",
                    IMU::has_angular_velocity,
                    IMU::get_angular_velocity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "linear_acceleration",
                    IMU::has_linear_acceleration,
                    IMU::get_linear_acceleration,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IMU>(
                    "IMU",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IMU {
    fn clear(&mut self) {
        self.clear_stamp();
        self.clear_entity_name();
        self.clear_orientation();
        self.clear_angular_velocity();
        self.clear_linear_acceleration();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IMU {
    fn eq(&self, other: &IMU) -> bool {
        self.stamp == other.stamp &&
        self.entity_name == other.entity_name &&
        self.orientation == other.orientation &&
        self.angular_velocity == other.angular_velocity &&
        self.linear_acceleration == other.linear_acceleration &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IMU {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x69, 0x6d, 0x75, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x10, 0x71, 0x75, 0x61, 0x74, 0x65, 0x72, 0x6e, 0x69, 0x6f, 0x6e,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xcf, 0x01, 0x0a, 0x03, 0x49, 0x4d, 0x55, 0x12, 0x20,
    0x0a, 0x05, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65,
    0x12, 0x13, 0x0a, 0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x2c, 0x0a, 0x0b, 0x6f, 0x72, 0x69, 0x65, 0x6e, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x51, 0x75, 0x61, 0x74, 0x65, 0x72, 0x6e,
    0x69, 0x6f, 0x6e, 0x12, 0x2f, 0x0a, 0x10, 0x61, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x5f, 0x76,
    0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x15, 0x2e,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x33, 0x64, 0x12, 0x32, 0x0a, 0x13, 0x6c, 0x69, 0x6e, 0x65, 0x61, 0x72, 0x5f, 0x61,
    0x63, 0x63, 0x65, 0x6c, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x4a, 0xa4, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x12, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x08, 0x07, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09, 0x07, 0x19, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0b, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0d, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x10, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e,
    0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x2a, 0x2b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x02, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0f, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0f, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x10,
    0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x10, 0x0b, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x14, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x11, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03,
    0x11, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x11, 0x14,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x11, 0x2a, 0x2b,
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
