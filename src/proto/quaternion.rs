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
pub struct Quaternion {
    // message fields
    x: ::std::option::Option<f64>,
    y: ::std::option::Option<f64>,
    z: ::std::option::Option<f64>,
    w: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Quaternion {}

impl Quaternion {
    pub fn new() -> Quaternion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Quaternion {
        static mut instance: ::protobuf::lazy::Lazy<Quaternion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Quaternion,
        };
        unsafe {
            instance.get(|| {
                Quaternion {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    z: ::std::option::Option::None,
                    w: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double x = 2;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f64) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f64 {
        self.x.unwrap_or(0.)
    }

    // required double y = 3;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f64) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f64 {
        self.y.unwrap_or(0.)
    }

    // required double z = 4;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f64) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> f64 {
        self.z.unwrap_or(0.)
    }

    // required double w = 5;

    pub fn clear_w(&mut self) {
        self.w = ::std::option::Option::None;
    }

    pub fn has_w(&self) -> bool {
        self.w.is_some()
    }

    // Param is passed by value, moved
    pub fn set_w(&mut self, v: f64) {
        self.w = ::std::option::Option::Some(v);
    }

    pub fn get_w(&self) -> f64 {
        self.w.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Quaternion {
    fn is_initialized(&self) -> bool {
        if self.x.is_none() {
            return false;
        };
        if self.y.is_none() {
            return false;
        };
        if self.z.is_none() {
            return false;
        };
        if self.w.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.x = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.y = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.z = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.w = ::std::option::Option::Some(tmp);
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
        if self.x.is_some() {
            my_size += 9;
        };
        if self.y.is_some() {
            my_size += 9;
        };
        if self.z.is_some() {
            my_size += 9;
        };
        if self.w.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.y {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.z {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.w {
            try!(os.write_double(5, v));
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
        ::std::any::TypeId::of::<Quaternion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Quaternion {
    fn new() -> Quaternion {
        Quaternion::new()
    }

    fn descriptor_static(_: ::std::option::Option<Quaternion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "x",
                    Quaternion::has_x,
                    Quaternion::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "y",
                    Quaternion::has_y,
                    Quaternion::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "z",
                    Quaternion::has_z,
                    Quaternion::get_z,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "w",
                    Quaternion::has_w,
                    Quaternion::get_w,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Quaternion>(
                    "Quaternion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Quaternion {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.clear_w();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == other.w &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Quaternion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x71, 0x75, 0x61, 0x74, 0x65, 0x72, 0x6e, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22,
    0x38, 0x0a, 0x0a, 0x51, 0x75, 0x61, 0x74, 0x65, 0x72, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x09, 0x0a,
    0x01, 0x78, 0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x01, 0x12, 0x09, 0x0a, 0x01, 0x7a, 0x18, 0x04, 0x20, 0x02, 0x28, 0x01, 0x12, 0x09,
    0x0a, 0x01, 0x77, 0x18, 0x05, 0x20, 0x02, 0x28, 0x01, 0x4a, 0xbe, 0x02, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x0d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x09, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a,
    0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x16, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0b, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c,
    0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x16, 0x17,
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
