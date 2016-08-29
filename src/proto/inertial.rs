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
pub struct Inertial {
    // message fields
    mass: ::std::option::Option<f64>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    ixx: ::std::option::Option<f64>,
    ixy: ::std::option::Option<f64>,
    ixz: ::std::option::Option<f64>,
    iyy: ::std::option::Option<f64>,
    iyz: ::std::option::Option<f64>,
    izz: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Inertial {}

impl Inertial {
    pub fn new() -> Inertial {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Inertial {
        static mut instance: ::protobuf::lazy::Lazy<Inertial> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Inertial,
        };
        unsafe {
            instance.get(|| {
                Inertial {
                    mass: ::std::option::Option::None,
                    pose: ::protobuf::SingularPtrField::none(),
                    ixx: ::std::option::Option::None,
                    ixy: ::std::option::Option::None,
                    ixz: ::std::option::Option::None,
                    iyy: ::std::option::Option::None,
                    iyz: ::std::option::Option::None,
                    izz: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double mass = 1;

    pub fn clear_mass(&mut self) {
        self.mass = ::std::option::Option::None;
    }

    pub fn has_mass(&self) -> bool {
        self.mass.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mass(&mut self, v: f64) {
        self.mass = ::std::option::Option::Some(v);
    }

    pub fn get_mass(&self) -> f64 {
        self.mass.unwrap_or(0.)
    }

    // optional .gazebo.msgs.Pose pose = 2;

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

    // optional double ixx = 3;

    pub fn clear_ixx(&mut self) {
        self.ixx = ::std::option::Option::None;
    }

    pub fn has_ixx(&self) -> bool {
        self.ixx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ixx(&mut self, v: f64) {
        self.ixx = ::std::option::Option::Some(v);
    }

    pub fn get_ixx(&self) -> f64 {
        self.ixx.unwrap_or(0.)
    }

    // optional double ixy = 4;

    pub fn clear_ixy(&mut self) {
        self.ixy = ::std::option::Option::None;
    }

    pub fn has_ixy(&self) -> bool {
        self.ixy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ixy(&mut self, v: f64) {
        self.ixy = ::std::option::Option::Some(v);
    }

    pub fn get_ixy(&self) -> f64 {
        self.ixy.unwrap_or(0.)
    }

    // optional double ixz = 5;

    pub fn clear_ixz(&mut self) {
        self.ixz = ::std::option::Option::None;
    }

    pub fn has_ixz(&self) -> bool {
        self.ixz.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ixz(&mut self, v: f64) {
        self.ixz = ::std::option::Option::Some(v);
    }

    pub fn get_ixz(&self) -> f64 {
        self.ixz.unwrap_or(0.)
    }

    // optional double iyy = 6;

    pub fn clear_iyy(&mut self) {
        self.iyy = ::std::option::Option::None;
    }

    pub fn has_iyy(&self) -> bool {
        self.iyy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iyy(&mut self, v: f64) {
        self.iyy = ::std::option::Option::Some(v);
    }

    pub fn get_iyy(&self) -> f64 {
        self.iyy.unwrap_or(0.)
    }

    // optional double iyz = 7;

    pub fn clear_iyz(&mut self) {
        self.iyz = ::std::option::Option::None;
    }

    pub fn has_iyz(&self) -> bool {
        self.iyz.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iyz(&mut self, v: f64) {
        self.iyz = ::std::option::Option::Some(v);
    }

    pub fn get_iyz(&self) -> f64 {
        self.iyz.unwrap_or(0.)
    }

    // optional double izz = 8;

    pub fn clear_izz(&mut self) {
        self.izz = ::std::option::Option::None;
    }

    pub fn has_izz(&self) -> bool {
        self.izz.is_some()
    }

    // Param is passed by value, moved
    pub fn set_izz(&mut self, v: f64) {
        self.izz = ::std::option::Option::Some(v);
    }

    pub fn get_izz(&self) -> f64 {
        self.izz.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Inertial {
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
                    self.mass = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.ixx = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.ixy = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.ixz = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.iyy = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.iyz = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.izz = ::std::option::Option::Some(tmp);
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
        if self.mass.is_some() {
            my_size += 9;
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.ixx.is_some() {
            my_size += 9;
        };
        if self.ixy.is_some() {
            my_size += 9;
        };
        if self.ixz.is_some() {
            my_size += 9;
        };
        if self.iyy.is_some() {
            my_size += 9;
        };
        if self.iyz.is_some() {
            my_size += 9;
        };
        if self.izz.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mass {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ixx {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.ixy {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.ixz {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.iyy {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.iyz {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.izz {
            try!(os.write_double(8, v));
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
        ::std::any::TypeId::of::<Inertial>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Inertial {
    fn new() -> Inertial {
        Inertial::new()
    }

    fn descriptor_static(_: ::std::option::Option<Inertial>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "mass",
                    Inertial::has_mass,
                    Inertial::get_mass,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Inertial::has_pose,
                    Inertial::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "ixx",
                    Inertial::has_ixx,
                    Inertial::get_ixx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "ixy",
                    Inertial::has_ixy,
                    Inertial::get_ixy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "ixz",
                    Inertial::has_ixz,
                    Inertial::get_ixz,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "iyy",
                    Inertial::has_iyy,
                    Inertial::get_iyy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "iyz",
                    Inertial::has_iyz,
                    Inertial::get_iyz,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "izz",
                    Inertial::has_izz,
                    Inertial::get_izz,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Inertial>(
                    "Inertial",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Inertial {
    fn clear(&mut self) {
        self.clear_mass();
        self.clear_pose();
        self.clear_ixx();
        self.clear_ixy();
        self.clear_ixz();
        self.clear_iyy();
        self.clear_iyz();
        self.clear_izz();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Inertial {
    fn eq(&self, other: &Inertial) -> bool {
        self.mass == other.mass &&
        self.pose == other.pose &&
        self.ixx == other.ixx &&
        self.ixy == other.ixy &&
        self.ixz == other.ixz &&
        self.iyy == other.iyy &&
        self.iyz == other.iyz &&
        self.izz == other.izz &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Inertial {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x69, 0x6e, 0x65, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x70,
    0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x87, 0x01, 0x0a, 0x08, 0x49, 0x6e,
    0x65, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x12, 0x0c, 0x0a, 0x04, 0x6d, 0x61, 0x73, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x01, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x0b, 0x0a, 0x03, 0x69, 0x78, 0x78, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x01, 0x12, 0x0b, 0x0a, 0x03, 0x69, 0x78, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x12,
    0x0b, 0x0a, 0x03, 0x69, 0x78, 0x7a, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0b, 0x0a, 0x03,
    0x69, 0x79, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0b, 0x0a, 0x03, 0x69, 0x79, 0x7a,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0b, 0x0a, 0x03, 0x69, 0x7a, 0x7a, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x01, 0x4a, 0xdd, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x13, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x07, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x13, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0c, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0c, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d,
    0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x0e, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x12,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0f, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x0f, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x10, 0x02,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x06, 0x12, 0x03, 0x11, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x11,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x11, 0x12, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x11, 0x25, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x12, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x12, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x12, 0x25, 0x26,
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
