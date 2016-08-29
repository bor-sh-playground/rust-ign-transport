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
pub struct Surface {
    // message fields
    friction: ::protobuf::SingularPtrField<super::friction::Friction>,
    restitution_coefficient: ::std::option::Option<f64>,
    bounce_threshold: ::std::option::Option<f64>,
    soft_cfm: ::std::option::Option<f64>,
    soft_erp: ::std::option::Option<f64>,
    kp: ::std::option::Option<f64>,
    kd: ::std::option::Option<f64>,
    max_vel: ::std::option::Option<f64>,
    min_depth: ::std::option::Option<f64>,
    collide_without_contact: ::std::option::Option<bool>,
    collide_without_contact_bitmask: ::std::option::Option<u32>,
    collide_bitmask: ::std::option::Option<u32>,
    elastic_modulus: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Surface {}

impl Surface {
    pub fn new() -> Surface {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Surface {
        static mut instance: ::protobuf::lazy::Lazy<Surface> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Surface,
        };
        unsafe {
            instance.get(|| {
                Surface {
                    friction: ::protobuf::SingularPtrField::none(),
                    restitution_coefficient: ::std::option::Option::None,
                    bounce_threshold: ::std::option::Option::None,
                    soft_cfm: ::std::option::Option::None,
                    soft_erp: ::std::option::Option::None,
                    kp: ::std::option::Option::None,
                    kd: ::std::option::Option::None,
                    max_vel: ::std::option::Option::None,
                    min_depth: ::std::option::Option::None,
                    collide_without_contact: ::std::option::Option::None,
                    collide_without_contact_bitmask: ::std::option::Option::None,
                    collide_bitmask: ::std::option::Option::None,
                    elastic_modulus: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gazebo.msgs.Friction friction = 1;

    pub fn clear_friction(&mut self) {
        self.friction.clear();
    }

    pub fn has_friction(&self) -> bool {
        self.friction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friction(&mut self, v: super::friction::Friction) {
        self.friction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_friction(&mut self) -> &mut super::friction::Friction {
        if self.friction.is_none() {
            self.friction.set_default();
        };
        self.friction.as_mut().unwrap()
    }

    // Take field
    pub fn take_friction(&mut self) -> super::friction::Friction {
        self.friction.take().unwrap_or_else(|| super::friction::Friction::new())
    }

    pub fn get_friction(&self) -> &super::friction::Friction {
        self.friction.as_ref().unwrap_or_else(|| super::friction::Friction::default_instance())
    }

    // optional double restitution_coefficient = 2;

    pub fn clear_restitution_coefficient(&mut self) {
        self.restitution_coefficient = ::std::option::Option::None;
    }

    pub fn has_restitution_coefficient(&self) -> bool {
        self.restitution_coefficient.is_some()
    }

    // Param is passed by value, moved
    pub fn set_restitution_coefficient(&mut self, v: f64) {
        self.restitution_coefficient = ::std::option::Option::Some(v);
    }

    pub fn get_restitution_coefficient(&self) -> f64 {
        self.restitution_coefficient.unwrap_or(0.)
    }

    // optional double bounce_threshold = 3;

    pub fn clear_bounce_threshold(&mut self) {
        self.bounce_threshold = ::std::option::Option::None;
    }

    pub fn has_bounce_threshold(&self) -> bool {
        self.bounce_threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bounce_threshold(&mut self, v: f64) {
        self.bounce_threshold = ::std::option::Option::Some(v);
    }

    pub fn get_bounce_threshold(&self) -> f64 {
        self.bounce_threshold.unwrap_or(0.)
    }

    // optional double soft_cfm = 4;

    pub fn clear_soft_cfm(&mut self) {
        self.soft_cfm = ::std::option::Option::None;
    }

    pub fn has_soft_cfm(&self) -> bool {
        self.soft_cfm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_soft_cfm(&mut self, v: f64) {
        self.soft_cfm = ::std::option::Option::Some(v);
    }

    pub fn get_soft_cfm(&self) -> f64 {
        self.soft_cfm.unwrap_or(0.)
    }

    // optional double soft_erp = 5;

    pub fn clear_soft_erp(&mut self) {
        self.soft_erp = ::std::option::Option::None;
    }

    pub fn has_soft_erp(&self) -> bool {
        self.soft_erp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_soft_erp(&mut self, v: f64) {
        self.soft_erp = ::std::option::Option::Some(v);
    }

    pub fn get_soft_erp(&self) -> f64 {
        self.soft_erp.unwrap_or(0.)
    }

    // optional double kp = 6;

    pub fn clear_kp(&mut self) {
        self.kp = ::std::option::Option::None;
    }

    pub fn has_kp(&self) -> bool {
        self.kp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kp(&mut self, v: f64) {
        self.kp = ::std::option::Option::Some(v);
    }

    pub fn get_kp(&self) -> f64 {
        self.kp.unwrap_or(0.)
    }

    // optional double kd = 7;

    pub fn clear_kd(&mut self) {
        self.kd = ::std::option::Option::None;
    }

    pub fn has_kd(&self) -> bool {
        self.kd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kd(&mut self, v: f64) {
        self.kd = ::std::option::Option::Some(v);
    }

    pub fn get_kd(&self) -> f64 {
        self.kd.unwrap_or(0.)
    }

    // optional double max_vel = 8;

    pub fn clear_max_vel(&mut self) {
        self.max_vel = ::std::option::Option::None;
    }

    pub fn has_max_vel(&self) -> bool {
        self.max_vel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_vel(&mut self, v: f64) {
        self.max_vel = ::std::option::Option::Some(v);
    }

    pub fn get_max_vel(&self) -> f64 {
        self.max_vel.unwrap_or(0.)
    }

    // optional double min_depth = 9;

    pub fn clear_min_depth(&mut self) {
        self.min_depth = ::std::option::Option::None;
    }

    pub fn has_min_depth(&self) -> bool {
        self.min_depth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_depth(&mut self, v: f64) {
        self.min_depth = ::std::option::Option::Some(v);
    }

    pub fn get_min_depth(&self) -> f64 {
        self.min_depth.unwrap_or(0.)
    }

    // optional bool collide_without_contact = 10;

    pub fn clear_collide_without_contact(&mut self) {
        self.collide_without_contact = ::std::option::Option::None;
    }

    pub fn has_collide_without_contact(&self) -> bool {
        self.collide_without_contact.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collide_without_contact(&mut self, v: bool) {
        self.collide_without_contact = ::std::option::Option::Some(v);
    }

    pub fn get_collide_without_contact(&self) -> bool {
        self.collide_without_contact.unwrap_or(false)
    }

    // optional uint32 collide_without_contact_bitmask = 11;

    pub fn clear_collide_without_contact_bitmask(&mut self) {
        self.collide_without_contact_bitmask = ::std::option::Option::None;
    }

    pub fn has_collide_without_contact_bitmask(&self) -> bool {
        self.collide_without_contact_bitmask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collide_without_contact_bitmask(&mut self, v: u32) {
        self.collide_without_contact_bitmask = ::std::option::Option::Some(v);
    }

    pub fn get_collide_without_contact_bitmask(&self) -> u32 {
        self.collide_without_contact_bitmask.unwrap_or(0)
    }

    // optional uint32 collide_bitmask = 12;

    pub fn clear_collide_bitmask(&mut self) {
        self.collide_bitmask = ::std::option::Option::None;
    }

    pub fn has_collide_bitmask(&self) -> bool {
        self.collide_bitmask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collide_bitmask(&mut self, v: u32) {
        self.collide_bitmask = ::std::option::Option::Some(v);
    }

    pub fn get_collide_bitmask(&self) -> u32 {
        self.collide_bitmask.unwrap_or(0)
    }

    // optional double elastic_modulus = 13;

    pub fn clear_elastic_modulus(&mut self) {
        self.elastic_modulus = ::std::option::Option::None;
    }

    pub fn has_elastic_modulus(&self) -> bool {
        self.elastic_modulus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_elastic_modulus(&mut self, v: f64) {
        self.elastic_modulus = ::std::option::Option::Some(v);
    }

    pub fn get_elastic_modulus(&self) -> f64 {
        self.elastic_modulus.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Surface {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.friction));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.restitution_coefficient = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.bounce_threshold = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.soft_cfm = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.soft_erp = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.kp = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.kd = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.max_vel = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.min_depth = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.collide_without_contact = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.collide_without_contact_bitmask = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.collide_bitmask = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.elastic_modulus = ::std::option::Option::Some(tmp);
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
        for value in &self.friction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.restitution_coefficient.is_some() {
            my_size += 9;
        };
        if self.bounce_threshold.is_some() {
            my_size += 9;
        };
        if self.soft_cfm.is_some() {
            my_size += 9;
        };
        if self.soft_erp.is_some() {
            my_size += 9;
        };
        if self.kp.is_some() {
            my_size += 9;
        };
        if self.kd.is_some() {
            my_size += 9;
        };
        if self.max_vel.is_some() {
            my_size += 9;
        };
        if self.min_depth.is_some() {
            my_size += 9;
        };
        if self.collide_without_contact.is_some() {
            my_size += 2;
        };
        for value in &self.collide_without_contact_bitmask {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.collide_bitmask {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.elastic_modulus.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.friction.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.restitution_coefficient {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.bounce_threshold {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.soft_cfm {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.soft_erp {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.kp {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.kd {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.max_vel {
            try!(os.write_double(8, v));
        };
        if let Some(v) = self.min_depth {
            try!(os.write_double(9, v));
        };
        if let Some(v) = self.collide_without_contact {
            try!(os.write_bool(10, v));
        };
        if let Some(v) = self.collide_without_contact_bitmask {
            try!(os.write_uint32(11, v));
        };
        if let Some(v) = self.collide_bitmask {
            try!(os.write_uint32(12, v));
        };
        if let Some(v) = self.elastic_modulus {
            try!(os.write_double(13, v));
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
        ::std::any::TypeId::of::<Surface>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Surface {
    fn new() -> Surface {
        Surface::new()
    }

    fn descriptor_static(_: ::std::option::Option<Surface>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "friction",
                    Surface::has_friction,
                    Surface::get_friction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "restitution_coefficient",
                    Surface::has_restitution_coefficient,
                    Surface::get_restitution_coefficient,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "bounce_threshold",
                    Surface::has_bounce_threshold,
                    Surface::get_bounce_threshold,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "soft_cfm",
                    Surface::has_soft_cfm,
                    Surface::get_soft_cfm,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "soft_erp",
                    Surface::has_soft_erp,
                    Surface::get_soft_erp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "kp",
                    Surface::has_kp,
                    Surface::get_kp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "kd",
                    Surface::has_kd,
                    Surface::get_kd,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "max_vel",
                    Surface::has_max_vel,
                    Surface::get_max_vel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "min_depth",
                    Surface::has_min_depth,
                    Surface::get_min_depth,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "collide_without_contact",
                    Surface::has_collide_without_contact,
                    Surface::get_collide_without_contact,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "collide_without_contact_bitmask",
                    Surface::has_collide_without_contact_bitmask,
                    Surface::get_collide_without_contact_bitmask,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "collide_bitmask",
                    Surface::has_collide_bitmask,
                    Surface::get_collide_bitmask,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "elastic_modulus",
                    Surface::has_elastic_modulus,
                    Surface::get_elastic_modulus,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Surface>(
                    "Surface",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Surface {
    fn clear(&mut self) {
        self.clear_friction();
        self.clear_restitution_coefficient();
        self.clear_bounce_threshold();
        self.clear_soft_cfm();
        self.clear_soft_erp();
        self.clear_kp();
        self.clear_kd();
        self.clear_max_vel();
        self.clear_min_depth();
        self.clear_collide_without_contact();
        self.clear_collide_without_contact_bitmask();
        self.clear_collide_bitmask();
        self.clear_elastic_modulus();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Surface {
    fn eq(&self, other: &Surface) -> bool {
        self.friction == other.friction &&
        self.restitution_coefficient == other.restitution_coefficient &&
        self.bounce_threshold == other.bounce_threshold &&
        self.soft_cfm == other.soft_cfm &&
        self.soft_erp == other.soft_erp &&
        self.kp == other.kp &&
        self.kd == other.kd &&
        self.max_vel == other.max_vel &&
        self.min_depth == other.min_depth &&
        self.collide_without_contact == other.collide_without_contact &&
        self.collide_without_contact_bitmask == other.collide_without_contact_bitmask &&
        self.collide_bitmask == other.collide_bitmask &&
        self.elastic_modulus == other.elastic_modulus &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Surface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x73, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0e, 0x66, 0x72,
    0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc9, 0x02, 0x0a,
    0x07, 0x53, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x12, 0x27, 0x0a, 0x08, 0x66, 0x72, 0x69, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x46, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x1f, 0x0a, 0x17, 0x72, 0x65, 0x73, 0x74, 0x69, 0x74, 0x75, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x63, 0x6f, 0x65, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x01, 0x12, 0x18, 0x0a, 0x10, 0x62, 0x6f, 0x75, 0x6e, 0x63, 0x65, 0x5f, 0x74, 0x68, 0x72,
    0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x12, 0x10, 0x0a, 0x08,
    0x73, 0x6f, 0x66, 0x74, 0x5f, 0x63, 0x66, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x12, 0x10,
    0x0a, 0x08, 0x73, 0x6f, 0x66, 0x74, 0x5f, 0x65, 0x72, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01,
    0x12, 0x0a, 0x0a, 0x02, 0x6b, 0x70, 0x18, 0x06, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0a, 0x0a, 0x02,
    0x6b, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x61, 0x78, 0x5f,
    0x76, 0x65, 0x6c, 0x18, 0x08, 0x20, 0x01, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x6d, 0x69, 0x6e,
    0x5f, 0x64, 0x65, 0x70, 0x74, 0x68, 0x18, 0x09, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1f, 0x0a, 0x17,
    0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x64, 0x65, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x5f,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x12, 0x27, 0x0a,
    0x1f, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x64, 0x65, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74,
    0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x5f, 0x62, 0x69, 0x74, 0x6d, 0x61, 0x73, 0x6b,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x17, 0x0a, 0x0f, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x64,
    0x65, 0x5f, 0x62, 0x69, 0x74, 0x6d, 0x61, 0x73, 0x6b, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x17, 0x0a, 0x0f, 0x65, 0x6c, 0x61, 0x73, 0x74, 0x69, 0x63, 0x5f, 0x6d, 0x6f, 0x64, 0x75, 0x6c,
    0x75, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x01, 0x4a, 0xb6, 0x07, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x18, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x09, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0b, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0b, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c,
    0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x12,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x2e, 0x2f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x0e, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x0e, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x10, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x10,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x12, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x2e, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x11, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x11, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x11, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x12, 0x02, 0x30,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x12, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x12, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08,
    0x12, 0x03, 0x13, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03,
    0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x13, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x13, 0x12, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x13, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x14, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x09, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09,
    0x05, 0x12, 0x03, 0x14, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12,
    0x03, 0x14, 0x10, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x14,
    0x2e, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x15, 0x02, 0x37, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x15, 0x12, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x03, 0x12, 0x03, 0x15, 0x34, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12,
    0x03, 0x16, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x16,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x16, 0x12, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x16, 0x2e, 0x30, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x17, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0c, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05,
    0x12, 0x03, 0x17, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x17, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x17, 0x2e,
    0x30,
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
