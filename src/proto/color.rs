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
pub struct Color {
    // message fields
    r: ::std::option::Option<f32>,
    g: ::std::option::Option<f32>,
    b: ::std::option::Option<f32>,
    a: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Color {}

impl Color {
    pub fn new() -> Color {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Color {
        static mut instance: ::protobuf::lazy::Lazy<Color> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Color,
        };
        unsafe {
            instance.get(|| {
                Color {
                    r: ::std::option::Option::None,
                    g: ::std::option::Option::None,
                    b: ::std::option::Option::None,
                    a: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required float r = 2;

    pub fn clear_r(&mut self) {
        self.r = ::std::option::Option::None;
    }

    pub fn has_r(&self) -> bool {
        self.r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r(&mut self, v: f32) {
        self.r = ::std::option::Option::Some(v);
    }

    pub fn get_r(&self) -> f32 {
        self.r.unwrap_or(0.)
    }

    // required float g = 3;

    pub fn clear_g(&mut self) {
        self.g = ::std::option::Option::None;
    }

    pub fn has_g(&self) -> bool {
        self.g.is_some()
    }

    // Param is passed by value, moved
    pub fn set_g(&mut self, v: f32) {
        self.g = ::std::option::Option::Some(v);
    }

    pub fn get_g(&self) -> f32 {
        self.g.unwrap_or(0.)
    }

    // required float b = 4;

    pub fn clear_b(&mut self) {
        self.b = ::std::option::Option::None;
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: f32) {
        self.b = ::std::option::Option::Some(v);
    }

    pub fn get_b(&self) -> f32 {
        self.b.unwrap_or(0.)
    }

    // optional float a = 5;

    pub fn clear_a(&mut self) {
        self.a = ::std::option::Option::None;
    }

    pub fn has_a(&self) -> bool {
        self.a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a(&mut self, v: f32) {
        self.a = ::std::option::Option::Some(v);
    }

    pub fn get_a(&self) -> f32 {
        self.a.unwrap_or(1f32)
    }
}

impl ::protobuf::Message for Color {
    fn is_initialized(&self) -> bool {
        if self.r.is_none() {
            return false;
        };
        if self.g.is_none() {
            return false;
        };
        if self.b.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.r = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.g = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.b = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.a = ::std::option::Option::Some(tmp);
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
        if self.r.is_some() {
            my_size += 5;
        };
        if self.g.is_some() {
            my_size += 5;
        };
        if self.b.is_some() {
            my_size += 5;
        };
        if self.a.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.r {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.g {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.b {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.a {
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
        ::std::any::TypeId::of::<Color>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Color {
    fn new() -> Color {
        Color::new()
    }

    fn descriptor_static(_: ::std::option::Option<Color>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "r",
                    Color::has_r,
                    Color::get_r,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "g",
                    Color::has_g,
                    Color::get_g,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "b",
                    Color::has_b,
                    Color::get_b,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "a",
                    Color::has_a,
                    Color::get_a,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Color>(
                    "Color",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Color {
    fn clear(&mut self) {
        self.clear_r();
        self.clear_g();
        self.clear_b();
        self.clear_a();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r &&
        self.g == other.g &&
        self.b == other.b &&
        self.a == other.a &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0x36, 0x0a, 0x05, 0x43, 0x6f,
    0x6c, 0x6f, 0x72, 0x12, 0x09, 0x0a, 0x01, 0x72, 0x18, 0x02, 0x20, 0x02, 0x28, 0x02, 0x12, 0x09,
    0x0a, 0x01, 0x67, 0x18, 0x03, 0x20, 0x02, 0x28, 0x02, 0x12, 0x09, 0x0a, 0x01, 0x62, 0x18, 0x04,
    0x20, 0x02, 0x28, 0x02, 0x12, 0x0c, 0x0a, 0x01, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x3a,
    0x01, 0x31, 0x4a, 0xda, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0d, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07,
    0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x09, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x09, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x0b, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x11, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x15, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x0c, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x0c, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x08, 0x12, 0x03, 0x0c, 0x17,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x07, 0x12, 0x03, 0x0c, 0x22, 0x25,
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
