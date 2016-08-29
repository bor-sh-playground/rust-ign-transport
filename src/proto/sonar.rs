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
pub struct Sonar {
    // message fields
    frame: ::protobuf::SingularField<::std::string::String>,
    world_pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    range_min: ::std::option::Option<f64>,
    range_max: ::std::option::Option<f64>,
    radius: ::std::option::Option<f64>,
    range: ::std::option::Option<f64>,
    contact: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Sonar {}

impl Sonar {
    pub fn new() -> Sonar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Sonar {
        static mut instance: ::protobuf::lazy::Lazy<Sonar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Sonar,
        };
        unsafe {
            instance.get(|| {
                Sonar {
                    frame: ::protobuf::SingularField::none(),
                    world_pose: ::protobuf::SingularPtrField::none(),
                    range_min: ::std::option::Option::None,
                    range_max: ::std::option::Option::None,
                    radius: ::std::option::Option::None,
                    range: ::std::option::Option::None,
                    contact: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string frame = 1;

    pub fn clear_frame(&mut self) {
        self.frame.clear();
    }

    pub fn has_frame(&self) -> bool {
        self.frame.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame(&mut self, v: ::std::string::String) {
        self.frame = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_frame(&mut self) -> &mut ::std::string::String {
        if self.frame.is_none() {
            self.frame.set_default();
        };
        self.frame.as_mut().unwrap()
    }

    // Take field
    pub fn take_frame(&mut self) -> ::std::string::String {
        self.frame.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_frame(&self) -> &str {
        match self.frame.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .gazebo.msgs.Pose world_pose = 2;

    pub fn clear_world_pose(&mut self) {
        self.world_pose.clear();
    }

    pub fn has_world_pose(&self) -> bool {
        self.world_pose.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_pose(&mut self, v: super::pose::Pose) {
        self.world_pose = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_pose(&mut self) -> &mut super::pose::Pose {
        if self.world_pose.is_none() {
            self.world_pose.set_default();
        };
        self.world_pose.as_mut().unwrap()
    }

    // Take field
    pub fn take_world_pose(&mut self) -> super::pose::Pose {
        self.world_pose.take().unwrap_or_else(|| super::pose::Pose::new())
    }

    pub fn get_world_pose(&self) -> &super::pose::Pose {
        self.world_pose.as_ref().unwrap_or_else(|| super::pose::Pose::default_instance())
    }

    // required double range_min = 3;

    pub fn clear_range_min(&mut self) {
        self.range_min = ::std::option::Option::None;
    }

    pub fn has_range_min(&self) -> bool {
        self.range_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_min(&mut self, v: f64) {
        self.range_min = ::std::option::Option::Some(v);
    }

    pub fn get_range_min(&self) -> f64 {
        self.range_min.unwrap_or(0.)
    }

    // required double range_max = 4;

    pub fn clear_range_max(&mut self) {
        self.range_max = ::std::option::Option::None;
    }

    pub fn has_range_max(&self) -> bool {
        self.range_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_max(&mut self, v: f64) {
        self.range_max = ::std::option::Option::Some(v);
    }

    pub fn get_range_max(&self) -> f64 {
        self.range_max.unwrap_or(0.)
    }

    // required double radius = 5;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: f64) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius(&self) -> f64 {
        self.radius.unwrap_or(0.)
    }

    // required double range = 6;

    pub fn clear_range(&mut self) {
        self.range = ::std::option::Option::None;
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: f64) {
        self.range = ::std::option::Option::Some(v);
    }

    pub fn get_range(&self) -> f64 {
        self.range.unwrap_or(0.)
    }

    // optional .gazebo.msgs.Vector3d contact = 7;

    pub fn clear_contact(&mut self) {
        self.contact.clear();
    }

    pub fn has_contact(&self) -> bool {
        self.contact.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contact(&mut self, v: super::vector3d::Vector3d) {
        self.contact = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contact(&mut self) -> &mut super::vector3d::Vector3d {
        if self.contact.is_none() {
            self.contact.set_default();
        };
        self.contact.as_mut().unwrap()
    }

    // Take field
    pub fn take_contact(&mut self) -> super::vector3d::Vector3d {
        self.contact.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_contact(&self) -> &super::vector3d::Vector3d {
        self.contact.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }
}

impl ::protobuf::Message for Sonar {
    fn is_initialized(&self) -> bool {
        if self.frame.is_none() {
            return false;
        };
        if self.world_pose.is_none() {
            return false;
        };
        if self.range_min.is_none() {
            return false;
        };
        if self.range_max.is_none() {
            return false;
        };
        if self.radius.is_none() {
            return false;
        };
        if self.range.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.frame));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.world_pose));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.range_min = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.range_max = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.radius = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.range = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contact));
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
        for value in &self.frame {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.world_pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.range_min.is_some() {
            my_size += 9;
        };
        if self.range_max.is_some() {
            my_size += 9;
        };
        if self.radius.is_some() {
            my_size += 9;
        };
        if self.range.is_some() {
            my_size += 9;
        };
        for value in &self.contact {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.frame.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.world_pose.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.range_min {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.range_max {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.radius {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.range {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.contact.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Sonar>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Sonar {
    fn new() -> Sonar {
        Sonar::new()
    }

    fn descriptor_static(_: ::std::option::Option<Sonar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "frame",
                    Sonar::has_frame,
                    Sonar::get_frame,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "world_pose",
                    Sonar::has_world_pose,
                    Sonar::get_world_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "range_min",
                    Sonar::has_range_min,
                    Sonar::get_range_min,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "range_max",
                    Sonar::has_range_max,
                    Sonar::get_range_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "radius",
                    Sonar::has_radius,
                    Sonar::get_radius,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "range",
                    Sonar::has_range,
                    Sonar::get_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "contact",
                    Sonar::has_contact,
                    Sonar::get_contact,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Sonar>(
                    "Sonar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Sonar {
    fn clear(&mut self) {
        self.clear_frame();
        self.clear_world_pose();
        self.clear_range_min();
        self.clear_range_max();
        self.clear_radius();
        self.clear_range();
        self.clear_contact();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Sonar {
    fn eq(&self, other: &Sonar) -> bool {
        self.frame == other.frame &&
        self.world_pose == other.world_pose &&
        self.range_min == other.range_min &&
        self.range_max == other.range_max &&
        self.radius == other.radius &&
        self.range == other.range &&
        self.contact == other.contact &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Sonar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x73, 0x6f, 0x6e, 0x61, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x70, 0x6f, 0x73, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xaa, 0x01, 0x0a, 0x05, 0x53, 0x6f, 0x6e, 0x61, 0x72,
    0x12, 0x0d, 0x0a, 0x05, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x25, 0x0a, 0x0a, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f,
    0x6d, 0x69, 0x6e, 0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x61, 0x6e,
    0x67, 0x65, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x04, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0e, 0x0a, 0x06,
    0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x18, 0x05, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0d, 0x0a, 0x05,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x06, 0x20, 0x02, 0x28, 0x01, 0x12, 0x26, 0x0a, 0x07, 0x63,
    0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x33, 0x64, 0x4a, 0xd3, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x14, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x06, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x07, 0x17, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x09, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0b, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c,
    0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0d, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0e,
    0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f, 0x12,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x10, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x10, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x10, 0x1e, 0x1f, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x13, 0x02,
    0x21, 0x1a, 0x2e, 0x2f, 0x20, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x13, 0x0b, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x13, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x13, 0x1f, 0x20,
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
