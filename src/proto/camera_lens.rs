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
pub struct CameraLens {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    c1: ::std::option::Option<f64>,
    c2: ::std::option::Option<f64>,
    c3: ::std::option::Option<f64>,
    f: ::std::option::Option<f64>,
    fun: ::protobuf::SingularField<::std::string::String>,
    scale_to_hfov: ::std::option::Option<bool>,
    cutoff_angle: ::std::option::Option<f64>,
    hfov: ::std::option::Option<f64>,
    env_texture_size: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CameraLens {}

impl CameraLens {
    pub fn new() -> CameraLens {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CameraLens {
        static mut instance: ::protobuf::lazy::Lazy<CameraLens> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CameraLens,
        };
        unsafe {
            instance.get(|| {
                CameraLens {
                    field_type: ::protobuf::SingularField::none(),
                    c1: ::std::option::Option::None,
                    c2: ::std::option::Option::None,
                    c3: ::std::option::Option::None,
                    f: ::std::option::Option::None,
                    fun: ::protobuf::SingularField::none(),
                    scale_to_hfov: ::std::option::Option::None,
                    cutoff_angle: ::std::option::Option::None,
                    hfov: ::std::option::Option::None,
                    env_texture_size: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string type = 1;

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

    // optional double c1 = 2;

    pub fn clear_c1(&mut self) {
        self.c1 = ::std::option::Option::None;
    }

    pub fn has_c1(&self) -> bool {
        self.c1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c1(&mut self, v: f64) {
        self.c1 = ::std::option::Option::Some(v);
    }

    pub fn get_c1(&self) -> f64 {
        self.c1.unwrap_or(0.)
    }

    // optional double c2 = 3;

    pub fn clear_c2(&mut self) {
        self.c2 = ::std::option::Option::None;
    }

    pub fn has_c2(&self) -> bool {
        self.c2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c2(&mut self, v: f64) {
        self.c2 = ::std::option::Option::Some(v);
    }

    pub fn get_c2(&self) -> f64 {
        self.c2.unwrap_or(0.)
    }

    // optional double c3 = 4;

    pub fn clear_c3(&mut self) {
        self.c3 = ::std::option::Option::None;
    }

    pub fn has_c3(&self) -> bool {
        self.c3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c3(&mut self, v: f64) {
        self.c3 = ::std::option::Option::Some(v);
    }

    pub fn get_c3(&self) -> f64 {
        self.c3.unwrap_or(0.)
    }

    // optional double f = 5;

    pub fn clear_f(&mut self) {
        self.f = ::std::option::Option::None;
    }

    pub fn has_f(&self) -> bool {
        self.f.is_some()
    }

    // Param is passed by value, moved
    pub fn set_f(&mut self, v: f64) {
        self.f = ::std::option::Option::Some(v);
    }

    pub fn get_f(&self) -> f64 {
        self.f.unwrap_or(0.)
    }

    // optional string fun = 6;

    pub fn clear_fun(&mut self) {
        self.fun.clear();
    }

    pub fn has_fun(&self) -> bool {
        self.fun.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fun(&mut self, v: ::std::string::String) {
        self.fun = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fun(&mut self) -> &mut ::std::string::String {
        if self.fun.is_none() {
            self.fun.set_default();
        };
        self.fun.as_mut().unwrap()
    }

    // Take field
    pub fn take_fun(&mut self) -> ::std::string::String {
        self.fun.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fun(&self) -> &str {
        match self.fun.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool scale_to_hfov = 7;

    pub fn clear_scale_to_hfov(&mut self) {
        self.scale_to_hfov = ::std::option::Option::None;
    }

    pub fn has_scale_to_hfov(&self) -> bool {
        self.scale_to_hfov.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale_to_hfov(&mut self, v: bool) {
        self.scale_to_hfov = ::std::option::Option::Some(v);
    }

    pub fn get_scale_to_hfov(&self) -> bool {
        self.scale_to_hfov.unwrap_or(false)
    }

    // optional double cutoff_angle = 8;

    pub fn clear_cutoff_angle(&mut self) {
        self.cutoff_angle = ::std::option::Option::None;
    }

    pub fn has_cutoff_angle(&self) -> bool {
        self.cutoff_angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cutoff_angle(&mut self, v: f64) {
        self.cutoff_angle = ::std::option::Option::Some(v);
    }

    pub fn get_cutoff_angle(&self) -> f64 {
        self.cutoff_angle.unwrap_or(0.)
    }

    // optional double hfov = 9;

    pub fn clear_hfov(&mut self) {
        self.hfov = ::std::option::Option::None;
    }

    pub fn has_hfov(&self) -> bool {
        self.hfov.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hfov(&mut self, v: f64) {
        self.hfov = ::std::option::Option::Some(v);
    }

    pub fn get_hfov(&self) -> f64 {
        self.hfov.unwrap_or(0.)
    }

    // optional int32 env_texture_size = 10;

    pub fn clear_env_texture_size(&mut self) {
        self.env_texture_size = ::std::option::Option::None;
    }

    pub fn has_env_texture_size(&self) -> bool {
        self.env_texture_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_env_texture_size(&mut self, v: i32) {
        self.env_texture_size = ::std::option::Option::Some(v);
    }

    pub fn get_env_texture_size(&self) -> i32 {
        self.env_texture_size.unwrap_or(0)
    }
}

impl ::protobuf::Message for CameraLens {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.c1 = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.c2 = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.c3 = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.f = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fun));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.scale_to_hfov = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.cutoff_angle = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.hfov = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.env_texture_size = ::std::option::Option::Some(tmp);
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.c1.is_some() {
            my_size += 9;
        };
        if self.c2.is_some() {
            my_size += 9;
        };
        if self.c3.is_some() {
            my_size += 9;
        };
        if self.f.is_some() {
            my_size += 9;
        };
        for value in &self.fun {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        if self.scale_to_hfov.is_some() {
            my_size += 2;
        };
        if self.cutoff_angle.is_some() {
            my_size += 9;
        };
        if self.hfov.is_some() {
            my_size += 9;
        };
        for value in &self.env_texture_size {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.c1 {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.c2 {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.c3 {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.f {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.fun.as_ref() {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.scale_to_hfov {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.cutoff_angle {
            try!(os.write_double(8, v));
        };
        if let Some(v) = self.hfov {
            try!(os.write_double(9, v));
        };
        if let Some(v) = self.env_texture_size {
            try!(os.write_int32(10, v));
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
        ::std::any::TypeId::of::<CameraLens>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CameraLens {
    fn new() -> CameraLens {
        CameraLens::new()
    }

    fn descriptor_static(_: ::std::option::Option<CameraLens>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "type",
                    CameraLens::has_field_type,
                    CameraLens::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "c1",
                    CameraLens::has_c1,
                    CameraLens::get_c1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "c2",
                    CameraLens::has_c2,
                    CameraLens::get_c2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "c3",
                    CameraLens::has_c3,
                    CameraLens::get_c3,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "f",
                    CameraLens::has_f,
                    CameraLens::get_f,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fun",
                    CameraLens::has_fun,
                    CameraLens::get_fun,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "scale_to_hfov",
                    CameraLens::has_scale_to_hfov,
                    CameraLens::get_scale_to_hfov,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "cutoff_angle",
                    CameraLens::has_cutoff_angle,
                    CameraLens::get_cutoff_angle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "hfov",
                    CameraLens::has_hfov,
                    CameraLens::get_hfov,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "env_texture_size",
                    CameraLens::has_env_texture_size,
                    CameraLens::get_env_texture_size,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CameraLens>(
                    "CameraLens",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CameraLens {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_c1();
        self.clear_c2();
        self.clear_c3();
        self.clear_f();
        self.clear_fun();
        self.clear_scale_to_hfov();
        self.clear_cutoff_angle();
        self.clear_hfov();
        self.clear_env_texture_size();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CameraLens {
    fn eq(&self, other: &CameraLens) -> bool {
        self.field_type == other.field_type &&
        self.c1 == other.c1 &&
        self.c2 == other.c2 &&
        self.c3 == other.c3 &&
        self.f == other.f &&
        self.fun == other.fun &&
        self.scale_to_hfov == other.scale_to_hfov &&
        self.cutoff_angle == other.cutoff_angle &&
        self.hfov == other.hfov &&
        self.env_texture_size == other.env_texture_size &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CameraLens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f, 0x6c, 0x65, 0x6e, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x22, 0xab, 0x01, 0x0a, 0x0a, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x4c, 0x65, 0x6e, 0x73, 0x12,
    0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0a, 0x0a,
    0x02, 0x63, 0x31, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0a, 0x0a, 0x02, 0x63, 0x32, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0a, 0x0a, 0x02, 0x63, 0x33, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x01, 0x12, 0x09, 0x0a, 0x01, 0x66, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0b, 0x0a, 0x03,
    0x66, 0x75, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x12, 0x15, 0x0a, 0x0d, 0x73, 0x63, 0x61,
    0x6c, 0x65, 0x5f, 0x74, 0x6f, 0x5f, 0x68, 0x66, 0x6f, 0x76, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x14, 0x0a, 0x0c, 0x63, 0x75, 0x74, 0x6f, 0x66, 0x66, 0x5f, 0x61, 0x6e, 0x67, 0x6c, 0x65,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0c, 0x0a, 0x04, 0x68, 0x66, 0x6f, 0x76, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x01, 0x12, 0x18, 0x0a, 0x10, 0x65, 0x6e, 0x76, 0x5f, 0x74, 0x65, 0x78, 0x74,
    0x75, 0x72, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x4a, 0xf4,
    0x0c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x2d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x12, 0x0a, 0xb4, 0x02, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x1b, 0x1a, 0xa6, 0x02, 0x2f, 0x20, 0x5c, 0x62,
    0x72, 0x69, 0x65, 0x66, 0x20, 0x54, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x72, 0x6f,
    0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c,
    0x65, 0x6e, 0x73, 0x0a, 0x2f, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x70, 0x6f, 0x73,
    0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x22, 0x67, 0x6e, 0x6f, 0x6d, 0x6f, 0x6e, 0x69, 0x63, 0x61, 0x6c, 0x22, 0x2c, 0x20, 0x22,
    0x73, 0x74, 0x65, 0x72, 0x65, 0x6f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x22, 0x2c, 0x20,
    0x22, 0x65, 0x71, 0x75, 0x69, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x22, 0x2c, 0x0a, 0x2f,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x65, 0x71, 0x75, 0x69, 0x73, 0x6f, 0x6c,
    0x69, 0x64, 0x5f, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x22, 0x2c, 0x20, 0x22, 0x73, 0x74, 0x65, 0x72,
    0x65, 0x6f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x22, 0x2c, 0x20, 0x22, 0x63, 0x75, 0x73,
    0x74, 0x6f, 0x6d, 0x22, 0x2e, 0x0a, 0x2f, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x49,
    0x66, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x22, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x22,
    0x20, 0x79, 0x6f, 0x75, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x79, 0x20, 0x61, 0x74, 0x20, 0x6c, 0x65, 0x61, 0x73, 0x74, 0x20, 0x6f, 0x6e,
    0x65, 0x0a, 0x2f, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x60, 0x63, 0x31, 0x60, 0x2c, 0x20, 0x60, 0x63, 0x32, 0x60, 0x2c, 0x20, 0x60, 0x63,
    0x33, 0x60, 0x2c, 0x20, 0x60, 0x66, 0x60, 0x20, 0x6f, 0x72, 0x20, 0x60, 0x66, 0x75, 0x6e, 0x60,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x19, 0x1a, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x10, 0x02, 0x19, 0x1a, 0x25, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65,
    0x66, 0x20, 0x4c, 0x69, 0x6e, 0x65, 0x61, 0x72, 0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x20, 0x73,
    0x63, 0x61, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x10, 0x17, 0x18, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x13, 0x02, 0x19, 0x1a, 0x1e, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x41, 0x6e,
    0x67, 0x6c, 0x65, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x61, 0x63, 0x74,
    0x6f, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x13, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x12, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x17, 0x18, 0x0a, 0x2a, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x16, 0x02, 0x19, 0x1a, 0x1d, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69,
    0x65, 0x66, 0x20, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x20,
    0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x16, 0x12,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x16, 0x17, 0x18, 0x0a,
    0x84, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x18, 0x1a, 0x77, 0x2f,
    0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4c, 0x69, 0x6e, 0x65, 0x61, 0x72, 0x20, 0x73,
    0x63, 0x61, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x2c, 0x20, 0x75,
    0x6e, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x60, 0x63, 0x31, 0x60, 0x2c, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x61, 0x64, 0x6a, 0x75, 0x73, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x68, 0x66, 0x6f, 0x76, 0x0a, 0x2f, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x69, 0x66, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x5f, 0x74, 0x6f, 0x5f,
    0x66, 0x6f, 0x76, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x60, 0x74,
    0x72, 0x75, 0x65, 0x60, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1a,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1a, 0x16, 0x17, 0x0a, 0x66,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1e, 0x02, 0x1a, 0x1a, 0x59, 0x2f, 0x20, 0x5c,
    0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x6d, 0x6f, 0x64, 0x69,
    0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69,
    0x62, 0x6c, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x22,
    0x74, 0x61, 0x6e, 0x22, 0x2c, 0x20, 0x22, 0x73, 0x69, 0x6e, 0x22, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x22, 0x69, 0x64, 0x22, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x1e,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1e, 0x12, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1e, 0x18, 0x19, 0x0a, 0x38,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x21, 0x02, 0x22, 0x1a, 0x2b, 0x2f, 0x20, 0x5c,
    0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x53, 0x63, 0x61, 0x6c, 0x65, 0x20, 0x69, 0x6d, 0x61, 0x67,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x69, 0x74, 0x20, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x6f, 0x6e,
    0x74, 0x61, 0x6c, 0x20, 0x46, 0x4f, 0x56, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x21, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x21,
    0x10, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x21, 0x20, 0x21,
    0x0a, 0x85, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x25, 0x02, 0x23, 0x1a, 0x78,
    0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x45, 0x76, 0x65, 0x72, 0x79, 0x74, 0x68,
    0x69, 0x6e, 0x67, 0x20, 0x6f, 0x75, 0x74, 0x73, 0x69, 0x64, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62,
    0x65, 0x20, 0x68, 0x69, 0x64, 0x64, 0x65, 0x6e, 0x2c, 0x0a, 0x2f, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x69, 0x73, 0x20,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x63, 0x61, 0x6d,
    0x65, 0x72, 0x61, 0x27, 0x73, 0x20, 0x58, 0x20, 0x28, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64,
    0x29, 0x20, 0x61, 0x78, 0x69, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12,
    0x03, 0x25, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x25,
    0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x25, 0x21, 0x22,
    0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x28, 0x02, 0x1b, 0x1a, 0x2e, 0x2f,
    0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x48, 0x6f, 0x72, 0x69, 0x7a, 0x6f, 0x6e, 0x74,
    0x61, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x76, 0x69, 0x65, 0x77,
    0x20, 0x69, 0x6e, 0x20, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x28, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03,
    0x12, 0x03, 0x28, 0x19, 0x1a, 0x0a, 0x66, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x2c,
    0x02, 0x27, 0x1a, 0x59, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x53, 0x69, 0x7a,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x75, 0x62, 0x65, 0x20, 0x6d, 0x61, 0x70, 0x20, 0x74, 0x65,
    0x78, 0x74, 0x75, 0x72, 0x65, 0x2c, 0x0a, 0x2f, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x20, 0x72, 0x65, 0x6e, 0x64, 0x65,
    0x72, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x2c, 0x11, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x2c, 0x24, 0x26,
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
