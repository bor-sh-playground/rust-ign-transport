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
pub struct Axis {
    // message fields
    xyz: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    limit_lower: ::std::option::Option<f64>,
    limit_upper: ::std::option::Option<f64>,
    limit_effort: ::std::option::Option<f64>,
    limit_velocity: ::std::option::Option<f64>,
    damping: ::std::option::Option<f64>,
    friction: ::std::option::Option<f64>,
    use_parent_model_frame: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Axis {}

impl Axis {
    pub fn new() -> Axis {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Axis {
        static mut instance: ::protobuf::lazy::Lazy<Axis> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Axis,
        };
        unsafe {
            instance.get(|| {
                Axis {
                    xyz: ::protobuf::SingularPtrField::none(),
                    limit_lower: ::std::option::Option::None,
                    limit_upper: ::std::option::Option::None,
                    limit_effort: ::std::option::Option::None,
                    limit_velocity: ::std::option::Option::None,
                    damping: ::std::option::Option::None,
                    friction: ::std::option::Option::None,
                    use_parent_model_frame: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Vector3d xyz = 1;

    pub fn clear_xyz(&mut self) {
        self.xyz.clear();
    }

    pub fn has_xyz(&self) -> bool {
        self.xyz.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xyz(&mut self, v: super::vector3d::Vector3d) {
        self.xyz = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_xyz(&mut self) -> &mut super::vector3d::Vector3d {
        if self.xyz.is_none() {
            self.xyz.set_default();
        };
        self.xyz.as_mut().unwrap()
    }

    // Take field
    pub fn take_xyz(&mut self) -> super::vector3d::Vector3d {
        self.xyz.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_xyz(&self) -> &super::vector3d::Vector3d {
        self.xyz.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // required double limit_lower = 2;

    pub fn clear_limit_lower(&mut self) {
        self.limit_lower = ::std::option::Option::None;
    }

    pub fn has_limit_lower(&self) -> bool {
        self.limit_lower.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit_lower(&mut self, v: f64) {
        self.limit_lower = ::std::option::Option::Some(v);
    }

    pub fn get_limit_lower(&self) -> f64 {
        self.limit_lower.unwrap_or(0.)
    }

    // required double limit_upper = 3;

    pub fn clear_limit_upper(&mut self) {
        self.limit_upper = ::std::option::Option::None;
    }

    pub fn has_limit_upper(&self) -> bool {
        self.limit_upper.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit_upper(&mut self, v: f64) {
        self.limit_upper = ::std::option::Option::Some(v);
    }

    pub fn get_limit_upper(&self) -> f64 {
        self.limit_upper.unwrap_or(0.)
    }

    // required double limit_effort = 4;

    pub fn clear_limit_effort(&mut self) {
        self.limit_effort = ::std::option::Option::None;
    }

    pub fn has_limit_effort(&self) -> bool {
        self.limit_effort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit_effort(&mut self, v: f64) {
        self.limit_effort = ::std::option::Option::Some(v);
    }

    pub fn get_limit_effort(&self) -> f64 {
        self.limit_effort.unwrap_or(0.)
    }

    // required double limit_velocity = 5;

    pub fn clear_limit_velocity(&mut self) {
        self.limit_velocity = ::std::option::Option::None;
    }

    pub fn has_limit_velocity(&self) -> bool {
        self.limit_velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit_velocity(&mut self, v: f64) {
        self.limit_velocity = ::std::option::Option::Some(v);
    }

    pub fn get_limit_velocity(&self) -> f64 {
        self.limit_velocity.unwrap_or(0.)
    }

    // required double damping = 6;

    pub fn clear_damping(&mut self) {
        self.damping = ::std::option::Option::None;
    }

    pub fn has_damping(&self) -> bool {
        self.damping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damping(&mut self, v: f64) {
        self.damping = ::std::option::Option::Some(v);
    }

    pub fn get_damping(&self) -> f64 {
        self.damping.unwrap_or(0.)
    }

    // required double friction = 7;

    pub fn clear_friction(&mut self) {
        self.friction = ::std::option::Option::None;
    }

    pub fn has_friction(&self) -> bool {
        self.friction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friction(&mut self, v: f64) {
        self.friction = ::std::option::Option::Some(v);
    }

    pub fn get_friction(&self) -> f64 {
        self.friction.unwrap_or(0.)
    }

    // required bool use_parent_model_frame = 8;

    pub fn clear_use_parent_model_frame(&mut self) {
        self.use_parent_model_frame = ::std::option::Option::None;
    }

    pub fn has_use_parent_model_frame(&self) -> bool {
        self.use_parent_model_frame.is_some()
    }

    // Param is passed by value, moved
    pub fn set_use_parent_model_frame(&mut self, v: bool) {
        self.use_parent_model_frame = ::std::option::Option::Some(v);
    }

    pub fn get_use_parent_model_frame(&self) -> bool {
        self.use_parent_model_frame.unwrap_or(false)
    }
}

impl ::protobuf::Message for Axis {
    fn is_initialized(&self) -> bool {
        if self.xyz.is_none() {
            return false;
        };
        if self.limit_lower.is_none() {
            return false;
        };
        if self.limit_upper.is_none() {
            return false;
        };
        if self.limit_effort.is_none() {
            return false;
        };
        if self.limit_velocity.is_none() {
            return false;
        };
        if self.damping.is_none() {
            return false;
        };
        if self.friction.is_none() {
            return false;
        };
        if self.use_parent_model_frame.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.xyz));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.limit_lower = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.limit_upper = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.limit_effort = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.limit_velocity = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.damping = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.friction = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.use_parent_model_frame = ::std::option::Option::Some(tmp);
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
        for value in &self.xyz {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.limit_lower.is_some() {
            my_size += 9;
        };
        if self.limit_upper.is_some() {
            my_size += 9;
        };
        if self.limit_effort.is_some() {
            my_size += 9;
        };
        if self.limit_velocity.is_some() {
            my_size += 9;
        };
        if self.damping.is_some() {
            my_size += 9;
        };
        if self.friction.is_some() {
            my_size += 9;
        };
        if self.use_parent_model_frame.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.xyz.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.limit_lower {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.limit_upper {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.limit_effort {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.limit_velocity {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.damping {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.friction {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.use_parent_model_frame {
            try!(os.write_bool(8, v));
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
        ::std::any::TypeId::of::<Axis>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Axis {
    fn new() -> Axis {
        Axis::new()
    }

    fn descriptor_static(_: ::std::option::Option<Axis>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "xyz",
                    Axis::has_xyz,
                    Axis::get_xyz,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "limit_lower",
                    Axis::has_limit_lower,
                    Axis::get_limit_lower,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "limit_upper",
                    Axis::has_limit_upper,
                    Axis::get_limit_upper,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "limit_effort",
                    Axis::has_limit_effort,
                    Axis::get_limit_effort,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "limit_velocity",
                    Axis::has_limit_velocity,
                    Axis::get_limit_velocity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "damping",
                    Axis::has_damping,
                    Axis::get_damping,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "friction",
                    Axis::has_friction,
                    Axis::get_friction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "use_parent_model_frame",
                    Axis::has_use_parent_model_frame,
                    Axis::get_use_parent_model_frame,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Axis>(
                    "Axis",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Axis {
    fn clear(&mut self) {
        self.clear_xyz();
        self.clear_limit_lower();
        self.clear_limit_upper();
        self.clear_limit_effort();
        self.clear_limit_velocity();
        self.clear_damping();
        self.clear_friction();
        self.clear_use_parent_model_frame();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Axis {
    fn eq(&self, other: &Axis) -> bool {
        self.xyz == other.xyz &&
        self.limit_lower == other.limit_lower &&
        self.limit_upper == other.limit_upper &&
        self.limit_effort == other.limit_effort &&
        self.limit_velocity == other.limit_velocity &&
        self.damping == other.damping &&
        self.friction == other.friction &&
        self.use_parent_model_frame == other.use_parent_model_frame &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Axis {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0a, 0x61, 0x78, 0x69, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0e, 0x76, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc5, 0x01, 0x0a, 0x04, 0x41, 0x78,
    0x69, 0x73, 0x12, 0x22, 0x0a, 0x03, 0x78, 0x79, 0x7a, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x12, 0x13, 0x0a, 0x0b, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f,
    0x6c, 0x6f, 0x77, 0x65, 0x72, 0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x12, 0x13, 0x0a, 0x0b, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x5f, 0x75, 0x70, 0x70, 0x65, 0x72, 0x18, 0x03, 0x20, 0x02, 0x28, 0x01,
    0x12, 0x14, 0x0a, 0x0c, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f, 0x65, 0x66, 0x66, 0x6f, 0x72, 0x74,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x01, 0x12, 0x16, 0x0a, 0x0e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f,
    0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x05, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0f,
    0x0a, 0x07, 0x64, 0x61, 0x6d, 0x70, 0x69, 0x6e, 0x67, 0x18, 0x06, 0x20, 0x02, 0x28, 0x01, 0x12,
    0x10, 0x0a, 0x08, 0x66, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x02, 0x28,
    0x01, 0x12, 0x1e, 0x0a, 0x16, 0x75, 0x73, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f,
    0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x18, 0x08, 0x20, 0x02, 0x28,
    0x08, 0x4a, 0xdd, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x12, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07,
    0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0a, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0a,
    0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x14, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x29, 0x2a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0b, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0b, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x02, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x0d, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0d, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x29, 0x2a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x0e, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0e,
    0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x0f, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12,
    0x03, 0x10, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x10,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x12, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x10, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x11, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05,
    0x12, 0x03, 0x11, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x11, 0x10, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x11, 0x29,
    0x2a,
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
