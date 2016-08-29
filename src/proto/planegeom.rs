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
pub struct PlaneGeom {
    // message fields
    normal: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    size: ::protobuf::SingularPtrField<super::vector2d::Vector2d>,
    d: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlaneGeom {}

impl PlaneGeom {
    pub fn new() -> PlaneGeom {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlaneGeom {
        static mut instance: ::protobuf::lazy::Lazy<PlaneGeom> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlaneGeom,
        };
        unsafe {
            instance.get(|| {
                PlaneGeom {
                    normal: ::protobuf::SingularPtrField::none(),
                    size: ::protobuf::SingularPtrField::none(),
                    d: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Vector3d normal = 1;

    pub fn clear_normal(&mut self) {
        self.normal.clear();
    }

    pub fn has_normal(&self) -> bool {
        self.normal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normal(&mut self, v: super::vector3d::Vector3d) {
        self.normal = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_normal(&mut self) -> &mut super::vector3d::Vector3d {
        if self.normal.is_none() {
            self.normal.set_default();
        };
        self.normal.as_mut().unwrap()
    }

    // Take field
    pub fn take_normal(&mut self) -> super::vector3d::Vector3d {
        self.normal.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_normal(&self) -> &super::vector3d::Vector3d {
        self.normal.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // required .gazebo.msgs.Vector2d size = 2;

    pub fn clear_size(&mut self) {
        self.size.clear();
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: super::vector2d::Vector2d) {
        self.size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_size(&mut self) -> &mut super::vector2d::Vector2d {
        if self.size.is_none() {
            self.size.set_default();
        };
        self.size.as_mut().unwrap()
    }

    // Take field
    pub fn take_size(&mut self) -> super::vector2d::Vector2d {
        self.size.take().unwrap_or_else(|| super::vector2d::Vector2d::new())
    }

    pub fn get_size(&self) -> &super::vector2d::Vector2d {
        self.size.as_ref().unwrap_or_else(|| super::vector2d::Vector2d::default_instance())
    }

    // optional double d = 3;

    pub fn clear_d(&mut self) {
        self.d = ::std::option::Option::None;
    }

    pub fn has_d(&self) -> bool {
        self.d.is_some()
    }

    // Param is passed by value, moved
    pub fn set_d(&mut self, v: f64) {
        self.d = ::std::option::Option::Some(v);
    }

    pub fn get_d(&self) -> f64 {
        self.d.unwrap_or(0f64)
    }
}

impl ::protobuf::Message for PlaneGeom {
    fn is_initialized(&self) -> bool {
        if self.normal.is_none() {
            return false;
        };
        if self.size.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.normal));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.size));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.d = ::std::option::Option::Some(tmp);
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
        for value in &self.normal {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.size {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.d.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.normal.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.size.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.d {
            try!(os.write_double(3, v));
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
        ::std::any::TypeId::of::<PlaneGeom>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlaneGeom {
    fn new() -> PlaneGeom {
        PlaneGeom::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlaneGeom>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "normal",
                    PlaneGeom::has_normal,
                    PlaneGeom::get_normal,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "size",
                    PlaneGeom::has_size,
                    PlaneGeom::get_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "d",
                    PlaneGeom::has_d,
                    PlaneGeom::get_d,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlaneGeom>(
                    "PlaneGeom",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlaneGeom {
    fn clear(&mut self) {
        self.clear_normal();
        self.clear_size();
        self.clear_d();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlaneGeom {
    fn eq(&self, other: &PlaneGeom) -> bool {
        self.normal == other.normal &&
        self.size == other.size &&
        self.d == other.d &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlaneGeom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x67, 0x65, 0x6f, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0e,
    0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e,
    0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x65,
    0x0a, 0x09, 0x50, 0x6c, 0x61, 0x6e, 0x65, 0x47, 0x65, 0x6f, 0x6d, 0x12, 0x25, 0x0a, 0x06, 0x6e,
    0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x33, 0x64, 0x12, 0x23, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x64, 0x12, 0x0c, 0x0a, 0x01, 0x64, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x01, 0x3a, 0x01, 0x30, 0x4a, 0xab, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0f, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x07, 0x07, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x17,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0c, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0c, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x14, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x0d, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0d, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d,
    0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x02, 0x2e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0e, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08,
    0x12, 0x03, 0x0e, 0x20, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x07, 0x12, 0x03,
    0x0e, 0x2b, 0x2c,
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
