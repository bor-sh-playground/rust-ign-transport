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
pub struct Distortion {
    // message fields
    center: ::protobuf::SingularPtrField<super::vector2d::Vector2d>,
    k1: ::std::option::Option<f64>,
    k2: ::std::option::Option<f64>,
    k3: ::std::option::Option<f64>,
    p1: ::std::option::Option<f64>,
    p2: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Distortion {}

impl Distortion {
    pub fn new() -> Distortion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Distortion {
        static mut instance: ::protobuf::lazy::Lazy<Distortion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Distortion,
        };
        unsafe {
            instance.get(|| {
                Distortion {
                    center: ::protobuf::SingularPtrField::none(),
                    k1: ::std::option::Option::None,
                    k2: ::std::option::Option::None,
                    k3: ::std::option::Option::None,
                    p1: ::std::option::Option::None,
                    p2: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gazebo.msgs.Vector2d center = 1;

    pub fn clear_center(&mut self) {
        self.center.clear();
    }

    pub fn has_center(&self) -> bool {
        self.center.is_some()
    }

    // Param is passed by value, moved
    pub fn set_center(&mut self, v: super::vector2d::Vector2d) {
        self.center = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_center(&mut self) -> &mut super::vector2d::Vector2d {
        if self.center.is_none() {
            self.center.set_default();
        };
        self.center.as_mut().unwrap()
    }

    // Take field
    pub fn take_center(&mut self) -> super::vector2d::Vector2d {
        self.center.take().unwrap_or_else(|| super::vector2d::Vector2d::new())
    }

    pub fn get_center(&self) -> &super::vector2d::Vector2d {
        self.center.as_ref().unwrap_or_else(|| super::vector2d::Vector2d::default_instance())
    }

    // optional double k1 = 2;

    pub fn clear_k1(&mut self) {
        self.k1 = ::std::option::Option::None;
    }

    pub fn has_k1(&self) -> bool {
        self.k1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_k1(&mut self, v: f64) {
        self.k1 = ::std::option::Option::Some(v);
    }

    pub fn get_k1(&self) -> f64 {
        self.k1.unwrap_or(0.)
    }

    // optional double k2 = 3;

    pub fn clear_k2(&mut self) {
        self.k2 = ::std::option::Option::None;
    }

    pub fn has_k2(&self) -> bool {
        self.k2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_k2(&mut self, v: f64) {
        self.k2 = ::std::option::Option::Some(v);
    }

    pub fn get_k2(&self) -> f64 {
        self.k2.unwrap_or(0.)
    }

    // optional double k3 = 4;

    pub fn clear_k3(&mut self) {
        self.k3 = ::std::option::Option::None;
    }

    pub fn has_k3(&self) -> bool {
        self.k3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_k3(&mut self, v: f64) {
        self.k3 = ::std::option::Option::Some(v);
    }

    pub fn get_k3(&self) -> f64 {
        self.k3.unwrap_or(0.)
    }

    // optional double p1 = 5;

    pub fn clear_p1(&mut self) {
        self.p1 = ::std::option::Option::None;
    }

    pub fn has_p1(&self) -> bool {
        self.p1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p1(&mut self, v: f64) {
        self.p1 = ::std::option::Option::Some(v);
    }

    pub fn get_p1(&self) -> f64 {
        self.p1.unwrap_or(0.)
    }

    // optional double p2 = 6;

    pub fn clear_p2(&mut self) {
        self.p2 = ::std::option::Option::None;
    }

    pub fn has_p2(&self) -> bool {
        self.p2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p2(&mut self, v: f64) {
        self.p2 = ::std::option::Option::Some(v);
    }

    pub fn get_p2(&self) -> f64 {
        self.p2.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Distortion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.center));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.k1 = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.k2 = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.k3 = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.p1 = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.p2 = ::std::option::Option::Some(tmp);
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
        for value in &self.center {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.k1.is_some() {
            my_size += 9;
        };
        if self.k2.is_some() {
            my_size += 9;
        };
        if self.k3.is_some() {
            my_size += 9;
        };
        if self.p1.is_some() {
            my_size += 9;
        };
        if self.p2.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.center.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.k1 {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.k2 {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.k3 {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.p1 {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.p2 {
            try!(os.write_double(6, v));
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
        ::std::any::TypeId::of::<Distortion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Distortion {
    fn new() -> Distortion {
        Distortion::new()
    }

    fn descriptor_static(_: ::std::option::Option<Distortion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "center",
                    Distortion::has_center,
                    Distortion::get_center,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "k1",
                    Distortion::has_k1,
                    Distortion::get_k1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "k2",
                    Distortion::has_k2,
                    Distortion::get_k2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "k3",
                    Distortion::has_k3,
                    Distortion::get_k3,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "p1",
                    Distortion::has_p1,
                    Distortion::get_p1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "p2",
                    Distortion::has_p2,
                    Distortion::get_p2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Distortion>(
                    "Distortion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Distortion {
    fn clear(&mut self) {
        self.clear_center();
        self.clear_k1();
        self.clear_k2();
        self.clear_k3();
        self.clear_p1();
        self.clear_p2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Distortion {
    fn eq(&self, other: &Distortion) -> bool {
        self.center == other.center &&
        self.k1 == other.k1 &&
        self.k2 == other.k2 &&
        self.k3 == other.k3 &&
        self.p1 == other.p1 &&
        self.p2 == other.p2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Distortion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x64, 0x69, 0x73, 0x74, 0x6f, 0x72, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a,
    0x0e, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x6f, 0x0a, 0x0a, 0x44, 0x69, 0x73, 0x74, 0x6f, 0x72, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x25, 0x0a,
    0x06, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x32, 0x64, 0x12, 0x0a, 0x0a, 0x02, 0x6b, 0x31, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01,
    0x12, 0x0a, 0x0a, 0x02, 0x6b, 0x32, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0a, 0x0a, 0x02,
    0x6b, 0x33, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x31, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x01, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x32, 0x18, 0x06, 0x20, 0x01, 0x28, 0x01,
    0x4a, 0xd3, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x10, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x17,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0a, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0a, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x14, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0b, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b,
    0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x02, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x0d, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x0e, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0e, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x0f, 0x1e, 0x1f,
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
