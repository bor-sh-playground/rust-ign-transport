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
pub struct CameraSensor {
    // message fields
    horizontal_fov: ::std::option::Option<f64>,
    image_size: ::protobuf::SingularPtrField<super::vector2d::Vector2d>,
    image_format: ::protobuf::SingularField<::std::string::String>,
    near_clip: ::std::option::Option<f64>,
    far_clip: ::std::option::Option<f64>,
    save_enabled: ::std::option::Option<bool>,
    save_path: ::protobuf::SingularField<::std::string::String>,
    distortion: ::protobuf::SingularPtrField<super::distortion::Distortion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CameraSensor {}

impl CameraSensor {
    pub fn new() -> CameraSensor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CameraSensor {
        static mut instance: ::protobuf::lazy::Lazy<CameraSensor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CameraSensor,
        };
        unsafe {
            instance.get(|| {
                CameraSensor {
                    horizontal_fov: ::std::option::Option::None,
                    image_size: ::protobuf::SingularPtrField::none(),
                    image_format: ::protobuf::SingularField::none(),
                    near_clip: ::std::option::Option::None,
                    far_clip: ::std::option::Option::None,
                    save_enabled: ::std::option::Option::None,
                    save_path: ::protobuf::SingularField::none(),
                    distortion: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double horizontal_fov = 1;

    pub fn clear_horizontal_fov(&mut self) {
        self.horizontal_fov = ::std::option::Option::None;
    }

    pub fn has_horizontal_fov(&self) -> bool {
        self.horizontal_fov.is_some()
    }

    // Param is passed by value, moved
    pub fn set_horizontal_fov(&mut self, v: f64) {
        self.horizontal_fov = ::std::option::Option::Some(v);
    }

    pub fn get_horizontal_fov(&self) -> f64 {
        self.horizontal_fov.unwrap_or(0.)
    }

    // optional .gazebo.msgs.Vector2d image_size = 2;

    pub fn clear_image_size(&mut self) {
        self.image_size.clear();
    }

    pub fn has_image_size(&self) -> bool {
        self.image_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image_size(&mut self, v: super::vector2d::Vector2d) {
        self.image_size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image_size(&mut self) -> &mut super::vector2d::Vector2d {
        if self.image_size.is_none() {
            self.image_size.set_default();
        };
        self.image_size.as_mut().unwrap()
    }

    // Take field
    pub fn take_image_size(&mut self) -> super::vector2d::Vector2d {
        self.image_size.take().unwrap_or_else(|| super::vector2d::Vector2d::new())
    }

    pub fn get_image_size(&self) -> &super::vector2d::Vector2d {
        self.image_size.as_ref().unwrap_or_else(|| super::vector2d::Vector2d::default_instance())
    }

    // optional string image_format = 3;

    pub fn clear_image_format(&mut self) {
        self.image_format.clear();
    }

    pub fn has_image_format(&self) -> bool {
        self.image_format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image_format(&mut self, v: ::std::string::String) {
        self.image_format = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image_format(&mut self) -> &mut ::std::string::String {
        if self.image_format.is_none() {
            self.image_format.set_default();
        };
        self.image_format.as_mut().unwrap()
    }

    // Take field
    pub fn take_image_format(&mut self) -> ::std::string::String {
        self.image_format.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_image_format(&self) -> &str {
        match self.image_format.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional double near_clip = 4;

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
        self.near_clip.unwrap_or(0.)
    }

    // optional double far_clip = 5;

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
        self.far_clip.unwrap_or(0.)
    }

    // optional bool save_enabled = 6;

    pub fn clear_save_enabled(&mut self) {
        self.save_enabled = ::std::option::Option::None;
    }

    pub fn has_save_enabled(&self) -> bool {
        self.save_enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_enabled(&mut self, v: bool) {
        self.save_enabled = ::std::option::Option::Some(v);
    }

    pub fn get_save_enabled(&self) -> bool {
        self.save_enabled.unwrap_or(false)
    }

    // optional string save_path = 7;

    pub fn clear_save_path(&mut self) {
        self.save_path.clear();
    }

    pub fn has_save_path(&self) -> bool {
        self.save_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_path(&mut self, v: ::std::string::String) {
        self.save_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_save_path(&mut self) -> &mut ::std::string::String {
        if self.save_path.is_none() {
            self.save_path.set_default();
        };
        self.save_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_save_path(&mut self) -> ::std::string::String {
        self.save_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_save_path(&self) -> &str {
        match self.save_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .gazebo.msgs.Distortion distortion = 8;

    pub fn clear_distortion(&mut self) {
        self.distortion.clear();
    }

    pub fn has_distortion(&self) -> bool {
        self.distortion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distortion(&mut self, v: super::distortion::Distortion) {
        self.distortion = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_distortion(&mut self) -> &mut super::distortion::Distortion {
        if self.distortion.is_none() {
            self.distortion.set_default();
        };
        self.distortion.as_mut().unwrap()
    }

    // Take field
    pub fn take_distortion(&mut self) -> super::distortion::Distortion {
        self.distortion.take().unwrap_or_else(|| super::distortion::Distortion::new())
    }

    pub fn get_distortion(&self) -> &super::distortion::Distortion {
        self.distortion.as_ref().unwrap_or_else(|| super::distortion::Distortion::default_instance())
    }
}

impl ::protobuf::Message for CameraSensor {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.horizontal_fov = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.image_size));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.image_format));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.near_clip = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.far_clip = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.save_enabled = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.save_path));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.distortion));
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
        if self.horizontal_fov.is_some() {
            my_size += 9;
        };
        for value in &self.image_size {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.image_format {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if self.near_clip.is_some() {
            my_size += 9;
        };
        if self.far_clip.is_some() {
            my_size += 9;
        };
        if self.save_enabled.is_some() {
            my_size += 2;
        };
        for value in &self.save_path {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        for value in &self.distortion {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.horizontal_fov {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.image_size.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.image_format.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.near_clip {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.far_clip {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.save_enabled {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.save_path.as_ref() {
            try!(os.write_string(7, &v));
        };
        if let Some(v) = self.distortion.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<CameraSensor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CameraSensor {
    fn new() -> CameraSensor {
        CameraSensor::new()
    }

    fn descriptor_static(_: ::std::option::Option<CameraSensor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "horizontal_fov",
                    CameraSensor::has_horizontal_fov,
                    CameraSensor::get_horizontal_fov,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "image_size",
                    CameraSensor::has_image_size,
                    CameraSensor::get_image_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "image_format",
                    CameraSensor::has_image_format,
                    CameraSensor::get_image_format,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "near_clip",
                    CameraSensor::has_near_clip,
                    CameraSensor::get_near_clip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "far_clip",
                    CameraSensor::has_far_clip,
                    CameraSensor::get_far_clip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "save_enabled",
                    CameraSensor::has_save_enabled,
                    CameraSensor::get_save_enabled,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "save_path",
                    CameraSensor::has_save_path,
                    CameraSensor::get_save_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "distortion",
                    CameraSensor::has_distortion,
                    CameraSensor::get_distortion,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CameraSensor>(
                    "CameraSensor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CameraSensor {
    fn clear(&mut self) {
        self.clear_horizontal_fov();
        self.clear_image_size();
        self.clear_image_format();
        self.clear_near_clip();
        self.clear_far_clip();
        self.clear_save_enabled();
        self.clear_save_path();
        self.clear_distortion();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CameraSensor {
    fn eq(&self, other: &CameraSensor) -> bool {
        self.horizontal_fov == other.horizontal_fov &&
        self.image_size == other.image_size &&
        self.image_format == other.image_format &&
        self.near_clip == other.near_clip &&
        self.far_clip == other.far_clip &&
        self.save_enabled == other.save_enabled &&
        self.save_path == other.save_path &&
        self.distortion == other.distortion &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CameraSensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x1a, 0x0e, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x10, 0x64, 0x69, 0x73, 0x74, 0x6f, 0x72, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0xe2, 0x01, 0x0a, 0x0c, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x53, 0x65,
    0x6e, 0x73, 0x6f, 0x72, 0x12, 0x16, 0x0a, 0x0e, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x6f, 0x6e, 0x74,
    0x61, 0x6c, 0x5f, 0x66, 0x6f, 0x76, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x12, 0x29, 0x0a, 0x0a,
    0x69, 0x6d, 0x61, 0x67, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x64, 0x12, 0x14, 0x0a, 0x0c, 0x69, 0x6d, 0x61, 0x67, 0x65,
    0x5f, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a,
    0x09, 0x6e, 0x65, 0x61, 0x72, 0x5f, 0x63, 0x6c, 0x69, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01,
    0x12, 0x10, 0x0a, 0x08, 0x66, 0x61, 0x72, 0x5f, 0x63, 0x6c, 0x69, 0x70, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x01, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x61, 0x76, 0x65, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c,
    0x65, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x61, 0x76, 0x65,
    0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x12, 0x2b, 0x0a, 0x0a, 0x64,
    0x69, 0x73, 0x74, 0x6f, 0x72, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x17, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x44, 0x69,
    0x73, 0x74, 0x6f, 0x72, 0x74, 0x69, 0x6f, 0x6e, 0x4a, 0xe8, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x13, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x07, 0x07, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x13, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b,
    0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0c, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0c, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x0e, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x12,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x23, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0f, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x0f, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x10, 0x02,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x06, 0x12, 0x03, 0x11, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x11,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x11, 0x12, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x11, 0x23, 0x24, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x12, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x12, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x12, 0x23, 0x24,
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
