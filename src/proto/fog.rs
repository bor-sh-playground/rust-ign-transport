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
pub struct Fog {
    // message fields
    field_type: ::std::option::Option<Fog_FogType>,
    color: ::protobuf::SingularPtrField<super::color::Color>,
    density: ::std::option::Option<f32>,
    start: ::std::option::Option<f32>,
    end: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Fog {}

impl Fog {
    pub fn new() -> Fog {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Fog {
        static mut instance: ::protobuf::lazy::Lazy<Fog> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Fog,
        };
        unsafe {
            instance.get(|| {
                Fog {
                    field_type: ::std::option::Option::None,
                    color: ::protobuf::SingularPtrField::none(),
                    density: ::std::option::Option::None,
                    start: ::std::option::Option::None,
                    end: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gazebo.msgs.Fog.FogType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Fog_FogType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Fog_FogType {
        self.field_type.unwrap_or(Fog_FogType::NONE)
    }

    // optional .gazebo.msgs.Color color = 2;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: super::color::Color) {
        self.color = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut super::color::Color {
        if self.color.is_none() {
            self.color.set_default();
        };
        self.color.as_mut().unwrap()
    }

    // Take field
    pub fn take_color(&mut self) -> super::color::Color {
        self.color.take().unwrap_or_else(|| super::color::Color::new())
    }

    pub fn get_color(&self) -> &super::color::Color {
        self.color.as_ref().unwrap_or_else(|| super::color::Color::default_instance())
    }

    // optional float density = 3;

    pub fn clear_density(&mut self) {
        self.density = ::std::option::Option::None;
    }

    pub fn has_density(&self) -> bool {
        self.density.is_some()
    }

    // Param is passed by value, moved
    pub fn set_density(&mut self, v: f32) {
        self.density = ::std::option::Option::Some(v);
    }

    pub fn get_density(&self) -> f32 {
        self.density.unwrap_or(0.)
    }

    // optional float start = 4;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: f32) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> f32 {
        self.start.unwrap_or(0.)
    }

    // optional float end = 5;

    pub fn clear_end(&mut self) {
        self.end = ::std::option::Option::None;
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: f32) {
        self.end = ::std::option::Option::Some(v);
    }

    pub fn get_end(&self) -> f32 {
        self.end.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Fog {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.color));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.density = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.start = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.end = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.color {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.density.is_some() {
            my_size += 5;
        };
        if self.start.is_some() {
            my_size += 5;
        };
        if self.end.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.color.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.density {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.start {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.end {
            try!(os.write_float(5, v));
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
        ::std::any::TypeId::of::<Fog>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Fog {
    fn new() -> Fog {
        Fog::new()
    }

    fn descriptor_static(_: ::std::option::Option<Fog>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Fog::has_field_type,
                    Fog::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "color",
                    Fog::has_color,
                    Fog::get_color,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "density",
                    Fog::has_density,
                    Fog::get_density,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "start",
                    Fog::has_start,
                    Fog::get_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "end",
                    Fog::has_end,
                    Fog::get_end,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Fog>(
                    "Fog",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Fog {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_color();
        self.clear_density();
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Fog {
    fn eq(&self, other: &Fog) -> bool {
        self.field_type == other.field_type &&
        self.color == other.color &&
        self.density == other.density &&
        self.start == other.start &&
        self.end == other.end &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Fog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Fog_FogType {
    NONE = 1,
    LINEAR = 2,
    EXPONENTIAL = 3,
    EXPONENTIAL2 = 4,
}

impl ::protobuf::ProtobufEnum for Fog_FogType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Fog_FogType> {
        match value {
            1 => ::std::option::Option::Some(Fog_FogType::NONE),
            2 => ::std::option::Option::Some(Fog_FogType::LINEAR),
            3 => ::std::option::Option::Some(Fog_FogType::EXPONENTIAL),
            4 => ::std::option::Option::Some(Fog_FogType::EXPONENTIAL2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Fog_FogType] = &[
            Fog_FogType::NONE,
            Fog_FogType::LINEAR,
            Fog_FogType::EXPONENTIAL,
            Fog_FogType::EXPONENTIAL2,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Fog_FogType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Fog_FogType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Fog_FogType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x66, 0x6f, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0b, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc1, 0x01, 0x0a, 0x03, 0x46, 0x6f, 0x67, 0x12, 0x26, 0x0a,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x46, 0x6f, 0x67, 0x2e, 0x46, 0x6f,
    0x67, 0x54, 0x79, 0x70, 0x65, 0x12, 0x21, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x2e, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x64, 0x65, 0x6e, 0x73,
    0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x12, 0x0b, 0x0a, 0x03, 0x65, 0x6e, 0x64, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x02, 0x22, 0x42, 0x0a, 0x07, 0x46, 0x6f, 0x67, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x08, 0x0a, 0x04, 0x4e, 0x4f, 0x4e, 0x45, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x4c, 0x49,
    0x4e, 0x45, 0x41, 0x52, 0x10, 0x02, 0x12, 0x0f, 0x0a, 0x0b, 0x45, 0x58, 0x50, 0x4f, 0x4e, 0x45,
    0x4e, 0x54, 0x49, 0x41, 0x4c, 0x10, 0x03, 0x12, 0x10, 0x0a, 0x0c, 0x45, 0x58, 0x50, 0x4f, 0x4e,
    0x45, 0x4e, 0x54, 0x49, 0x41, 0x4c, 0x32, 0x10, 0x04, 0x4a, 0xe6, 0x04, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x18, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x08, 0x07, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x0a, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08,
    0x0b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x02, 0x12, 0x03, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x07, 0x0e, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x13, 0x14, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0f, 0x13, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x10, 0x13, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x11, 0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x11, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x13, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x13,
    0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x13, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x14, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x14, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x14, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x14, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x15, 0x02, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x16, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x16, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x16, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x16, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x17, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x17, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x17, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x17,
    0x17, 0x18,
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
