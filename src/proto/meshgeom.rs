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
pub struct MeshGeom {
    // message fields
    filename: ::protobuf::SingularField<::std::string::String>,
    scale: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    submesh: ::protobuf::SingularField<::std::string::String>,
    center_submesh: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MeshGeom {}

impl MeshGeom {
    pub fn new() -> MeshGeom {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MeshGeom {
        static mut instance: ::protobuf::lazy::Lazy<MeshGeom> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MeshGeom,
        };
        unsafe {
            instance.get(|| {
                MeshGeom {
                    filename: ::protobuf::SingularField::none(),
                    scale: ::protobuf::SingularPtrField::none(),
                    submesh: ::protobuf::SingularField::none(),
                    center_submesh: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string filename = 1;

    pub fn clear_filename(&mut self) {
        self.filename.clear();
    }

    pub fn has_filename(&self) -> bool {
        self.filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        if self.filename.is_none() {
            self.filename.set_default();
        };
        self.filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        self.filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_filename(&self) -> &str {
        match self.filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .gazebo.msgs.Vector3d scale = 2;

    pub fn clear_scale(&mut self) {
        self.scale.clear();
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: super::vector3d::Vector3d) {
        self.scale = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scale(&mut self) -> &mut super::vector3d::Vector3d {
        if self.scale.is_none() {
            self.scale.set_default();
        };
        self.scale.as_mut().unwrap()
    }

    // Take field
    pub fn take_scale(&mut self) -> super::vector3d::Vector3d {
        self.scale.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_scale(&self) -> &super::vector3d::Vector3d {
        self.scale.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // optional string submesh = 3;

    pub fn clear_submesh(&mut self) {
        self.submesh.clear();
    }

    pub fn has_submesh(&self) -> bool {
        self.submesh.is_some()
    }

    // Param is passed by value, moved
    pub fn set_submesh(&mut self, v: ::std::string::String) {
        self.submesh = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_submesh(&mut self) -> &mut ::std::string::String {
        if self.submesh.is_none() {
            self.submesh.set_default();
        };
        self.submesh.as_mut().unwrap()
    }

    // Take field
    pub fn take_submesh(&mut self) -> ::std::string::String {
        self.submesh.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_submesh(&self) -> &str {
        match self.submesh.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool center_submesh = 4;

    pub fn clear_center_submesh(&mut self) {
        self.center_submesh = ::std::option::Option::None;
    }

    pub fn has_center_submesh(&self) -> bool {
        self.center_submesh.is_some()
    }

    // Param is passed by value, moved
    pub fn set_center_submesh(&mut self, v: bool) {
        self.center_submesh = ::std::option::Option::Some(v);
    }

    pub fn get_center_submesh(&self) -> bool {
        self.center_submesh.unwrap_or(false)
    }
}

impl ::protobuf::Message for MeshGeom {
    fn is_initialized(&self) -> bool {
        if self.filename.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.scale));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.submesh));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.center_submesh = ::std::option::Option::Some(tmp);
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
        for value in &self.filename {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.scale {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.submesh {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if self.center_submesh.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.filename.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.scale.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.submesh.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.center_submesh {
            try!(os.write_bool(4, v));
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
        ::std::any::TypeId::of::<MeshGeom>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MeshGeom {
    fn new() -> MeshGeom {
        MeshGeom::new()
    }

    fn descriptor_static(_: ::std::option::Option<MeshGeom>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "filename",
                    MeshGeom::has_filename,
                    MeshGeom::get_filename,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scale",
                    MeshGeom::has_scale,
                    MeshGeom::get_scale,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "submesh",
                    MeshGeom::has_submesh,
                    MeshGeom::get_submesh,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "center_submesh",
                    MeshGeom::has_center_submesh,
                    MeshGeom::get_center_submesh,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MeshGeom>(
                    "MeshGeom",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MeshGeom {
    fn clear(&mut self) {
        self.clear_filename();
        self.clear_scale();
        self.clear_submesh();
        self.clear_center_submesh();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MeshGeom {
    fn eq(&self, other: &MeshGeom) -> bool {
        self.filename == other.filename &&
        self.scale == other.scale &&
        self.submesh == other.submesh &&
        self.center_submesh == other.center_submesh &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MeshGeom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x6d, 0x65, 0x73, 0x68, 0x67, 0x65, 0x6f, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0e, 0x76,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x6b, 0x0a,
    0x08, 0x4d, 0x65, 0x73, 0x68, 0x47, 0x65, 0x6f, 0x6d, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x69, 0x6c,
    0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x24, 0x0a, 0x05, 0x73,
    0x63, 0x61, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33,
    0x64, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75, 0x62, 0x6d, 0x65, 0x73, 0x68, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x12, 0x16, 0x0a, 0x0e, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x73, 0x75, 0x62,
    0x6d, 0x65, 0x73, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x4a, 0xc9, 0x02, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x0f, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x09, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09,
    0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0b, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x0c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x0b, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x14, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x0d, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x1d,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x02, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x0e, 0x21, 0x22,
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
