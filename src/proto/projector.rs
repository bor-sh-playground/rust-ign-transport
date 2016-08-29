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
pub struct Projector {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    texture: ::protobuf::SingularField<::std::string::String>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    fov: ::std::option::Option<f64>,
    near_clip: ::std::option::Option<f64>,
    far_clip: ::std::option::Option<f64>,
    enabled: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Projector {}

impl Projector {
    pub fn new() -> Projector {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Projector {
        static mut instance: ::protobuf::lazy::Lazy<Projector> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Projector,
        };
        unsafe {
            instance.get(|| {
                Projector {
                    name: ::protobuf::SingularField::none(),
                    texture: ::protobuf::SingularField::none(),
                    pose: ::protobuf::SingularPtrField::none(),
                    fov: ::std::option::Option::None,
                    near_clip: ::std::option::Option::None,
                    far_clip: ::std::option::Option::None,
                    enabled: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string texture = 2;

    pub fn clear_texture(&mut self) {
        self.texture.clear();
    }

    pub fn has_texture(&self) -> bool {
        self.texture.is_some()
    }

    // Param is passed by value, moved
    pub fn set_texture(&mut self, v: ::std::string::String) {
        self.texture = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_texture(&mut self) -> &mut ::std::string::String {
        if self.texture.is_none() {
            self.texture.set_default();
        };
        self.texture.as_mut().unwrap()
    }

    // Take field
    pub fn take_texture(&mut self) -> ::std::string::String {
        self.texture.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_texture(&self) -> &str {
        match self.texture.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .gazebo.msgs.Pose pose = 3;

    pub fn clear_pose(&mut self) {
        self.pose.clear();
    }

    pub fn has_pose(&self) -> bool {
        self.pose.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pose(&mut self, v: super::pose::Pose) {
        self.pose = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pose(&mut self) -> &mut super::pose::Pose {
        if self.pose.is_none() {
            self.pose.set_default();
        };
        self.pose.as_mut().unwrap()
    }

    // Take field
    pub fn take_pose(&mut self) -> super::pose::Pose {
        self.pose.take().unwrap_or_else(|| super::pose::Pose::new())
    }

    pub fn get_pose(&self) -> &super::pose::Pose {
        self.pose.as_ref().unwrap_or_else(|| super::pose::Pose::default_instance())
    }

    // optional double fov = 4;

    pub fn clear_fov(&mut self) {
        self.fov = ::std::option::Option::None;
    }

    pub fn has_fov(&self) -> bool {
        self.fov.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fov(&mut self, v: f64) {
        self.fov = ::std::option::Option::Some(v);
    }

    pub fn get_fov(&self) -> f64 {
        self.fov.unwrap_or(0.785f64)
    }

    // optional double near_clip = 5;

    pub fn clear_near_clip(&mut self) {
        self.near_clip = ::std::option::Option::None;
    }

    pub fn has_near_clip(&self) -> bool {
        self.near_clip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_near_clip(&mut self, v: f64) {
        self.near_clip = ::std::option::Option::Some(v);
    }

    pub fn get_near_clip(&self) -> f64 {
        self.near_clip.unwrap_or(0.1f64)
    }

    // optional double far_clip = 6;

    pub fn clear_far_clip(&mut self) {
        self.far_clip = ::std::option::Option::None;
    }

    pub fn has_far_clip(&self) -> bool {
        self.far_clip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_far_clip(&mut self, v: f64) {
        self.far_clip = ::std::option::Option::Some(v);
    }

    pub fn get_far_clip(&self) -> f64 {
        self.far_clip.unwrap_or(10f64)
    }

    // optional bool enabled = 7;

    pub fn clear_enabled(&mut self) {
        self.enabled = ::std::option::Option::None;
    }

    pub fn has_enabled(&self) -> bool {
        self.enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = ::std::option::Option::Some(v);
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.unwrap_or(true)
    }
}

impl ::protobuf::Message for Projector {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.texture));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.fov = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.near_clip = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.far_clip = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.enabled = ::std::option::Option::Some(tmp);
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.texture {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.fov.is_some() {
            my_size += 9;
        };
        if self.near_clip.is_some() {
            my_size += 9;
        };
        if self.far_clip.is_some() {
            my_size += 9;
        };
        if self.enabled.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.texture.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.fov {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.near_clip {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.far_clip {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.enabled {
            try!(os.write_bool(7, v));
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
        ::std::any::TypeId::of::<Projector>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Projector {
    fn new() -> Projector {
        Projector::new()
    }

    fn descriptor_static(_: ::std::option::Option<Projector>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Projector::has_name,
                    Projector::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "texture",
                    Projector::has_texture,
                    Projector::get_texture,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Projector::has_pose,
                    Projector::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "fov",
                    Projector::has_fov,
                    Projector::get_fov,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "near_clip",
                    Projector::has_near_clip,
                    Projector::get_near_clip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "far_clip",
                    Projector::has_far_clip,
                    Projector::get_far_clip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "enabled",
                    Projector::has_enabled,
                    Projector::get_enabled,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Projector>(
                    "Projector",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Projector {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_texture();
        self.clear_pose();
        self.clear_fov();
        self.clear_near_clip();
        self.clear_far_clip();
        self.clear_enabled();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Projector {
    fn eq(&self, other: &Projector) -> bool {
        self.name == other.name &&
        self.texture == other.texture &&
        self.pose == other.pose &&
        self.fov == other.fov &&
        self.near_clip == other.near_clip &&
        self.far_clip == other.far_clip &&
        self.enabled == other.enabled &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Projector {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a,
    0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa4, 0x01, 0x0a, 0x09, 0x50,
    0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x65, 0x78, 0x74, 0x75, 0x72,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x03, 0x66, 0x6f, 0x76, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x05, 0x30, 0x2e, 0x37, 0x38, 0x35, 0x12, 0x16, 0x0a, 0x09,
    0x6e, 0x65, 0x61, 0x72, 0x5f, 0x63, 0x6c, 0x69, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x3a,
    0x03, 0x30, 0x2e, 0x31, 0x12, 0x14, 0x0a, 0x08, 0x66, 0x61, 0x72, 0x5f, 0x63, 0x6c, 0x69, 0x70,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x02, 0x31, 0x30, 0x12, 0x15, 0x0a, 0x07, 0x65, 0x6e,
    0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75,
    0x65, 0x4a, 0x88, 0x05, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x12, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07,
    0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0b, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0c, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0c, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x0e, 0x02, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x22, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x08, 0x12, 0x03, 0x0e, 0x23, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x07, 0x12, 0x03, 0x0e, 0x2c, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x0f, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0f,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x08, 0x12, 0x03, 0x0f, 0x23, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x07, 0x12, 0x03, 0x0f, 0x2c, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x05, 0x12, 0x03, 0x10, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x12,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x08, 0x12, 0x03, 0x10, 0x23, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x07, 0x12, 0x03, 0x10, 0x2c, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x06, 0x12, 0x03, 0x11, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x11, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x11,
    0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x11, 0x22, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x08, 0x12, 0x03, 0x11, 0x23, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x07, 0x12, 0x03, 0x11, 0x2c, 0x30,
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
