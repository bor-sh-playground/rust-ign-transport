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
pub struct Physics {
    // message fields
    field_type: ::std::option::Option<Physics_Type>,
    solver_type: ::protobuf::SingularField<::std::string::String>,
    min_step_size: ::std::option::Option<f64>,
    precon_iters: ::std::option::Option<i32>,
    iters: ::std::option::Option<i32>,
    sor: ::std::option::Option<f64>,
    cfm: ::std::option::Option<f64>,
    erp: ::std::option::Option<f64>,
    contact_max_correcting_vel: ::std::option::Option<f64>,
    contact_surface_layer: ::std::option::Option<f64>,
    gravity: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    enable_physics: ::std::option::Option<bool>,
    real_time_factor: ::std::option::Option<f64>,
    real_time_update_rate: ::std::option::Option<f64>,
    max_step_size: ::std::option::Option<f64>,
    profile_name: ::protobuf::SingularField<::std::string::String>,
    magnetic_field: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Physics {}

impl Physics {
    pub fn new() -> Physics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Physics {
        static mut instance: ::protobuf::lazy::Lazy<Physics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Physics,
        };
        unsafe {
            instance.get(|| {
                Physics {
                    field_type: ::std::option::Option::None,
                    solver_type: ::protobuf::SingularField::none(),
                    min_step_size: ::std::option::Option::None,
                    precon_iters: ::std::option::Option::None,
                    iters: ::std::option::Option::None,
                    sor: ::std::option::Option::None,
                    cfm: ::std::option::Option::None,
                    erp: ::std::option::Option::None,
                    contact_max_correcting_vel: ::std::option::Option::None,
                    contact_surface_layer: ::std::option::Option::None,
                    gravity: ::protobuf::SingularPtrField::none(),
                    enable_physics: ::std::option::Option::None,
                    real_time_factor: ::std::option::Option::None,
                    real_time_update_rate: ::std::option::Option::None,
                    max_step_size: ::std::option::Option::None,
                    profile_name: ::protobuf::SingularField::none(),
                    magnetic_field: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gazebo.msgs.Physics.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Physics_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Physics_Type {
        self.field_type.unwrap_or(Physics_Type::ODE)
    }

    // optional string solver_type = 2;

    pub fn clear_solver_type(&mut self) {
        self.solver_type.clear();
    }

    pub fn has_solver_type(&self) -> bool {
        self.solver_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_solver_type(&mut self, v: ::std::string::String) {
        self.solver_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_solver_type(&mut self) -> &mut ::std::string::String {
        if self.solver_type.is_none() {
            self.solver_type.set_default();
        };
        self.solver_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_solver_type(&mut self) -> ::std::string::String {
        self.solver_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_solver_type(&self) -> &str {
        match self.solver_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional double min_step_size = 3;

    pub fn clear_min_step_size(&mut self) {
        self.min_step_size = ::std::option::Option::None;
    }

    pub fn has_min_step_size(&self) -> bool {
        self.min_step_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_step_size(&mut self, v: f64) {
        self.min_step_size = ::std::option::Option::Some(v);
    }

    pub fn get_min_step_size(&self) -> f64 {
        self.min_step_size.unwrap_or(0.)
    }

    // optional int32 precon_iters = 4;

    pub fn clear_precon_iters(&mut self) {
        self.precon_iters = ::std::option::Option::None;
    }

    pub fn has_precon_iters(&self) -> bool {
        self.precon_iters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_precon_iters(&mut self, v: i32) {
        self.precon_iters = ::std::option::Option::Some(v);
    }

    pub fn get_precon_iters(&self) -> i32 {
        self.precon_iters.unwrap_or(0)
    }

    // optional int32 iters = 5;

    pub fn clear_iters(&mut self) {
        self.iters = ::std::option::Option::None;
    }

    pub fn has_iters(&self) -> bool {
        self.iters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iters(&mut self, v: i32) {
        self.iters = ::std::option::Option::Some(v);
    }

    pub fn get_iters(&self) -> i32 {
        self.iters.unwrap_or(0)
    }

    // optional double sor = 6;

    pub fn clear_sor(&mut self) {
        self.sor = ::std::option::Option::None;
    }

    pub fn has_sor(&self) -> bool {
        self.sor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sor(&mut self, v: f64) {
        self.sor = ::std::option::Option::Some(v);
    }

    pub fn get_sor(&self) -> f64 {
        self.sor.unwrap_or(0.)
    }

    // optional double cfm = 7;

    pub fn clear_cfm(&mut self) {
        self.cfm = ::std::option::Option::None;
    }

    pub fn has_cfm(&self) -> bool {
        self.cfm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cfm(&mut self, v: f64) {
        self.cfm = ::std::option::Option::Some(v);
    }

    pub fn get_cfm(&self) -> f64 {
        self.cfm.unwrap_or(0.)
    }

    // optional double erp = 8;

    pub fn clear_erp(&mut self) {
        self.erp = ::std::option::Option::None;
    }

    pub fn has_erp(&self) -> bool {
        self.erp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_erp(&mut self, v: f64) {
        self.erp = ::std::option::Option::Some(v);
    }

    pub fn get_erp(&self) -> f64 {
        self.erp.unwrap_or(0.)
    }

    // optional double contact_max_correcting_vel = 9;

    pub fn clear_contact_max_correcting_vel(&mut self) {
        self.contact_max_correcting_vel = ::std::option::Option::None;
    }

    pub fn has_contact_max_correcting_vel(&self) -> bool {
        self.contact_max_correcting_vel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contact_max_correcting_vel(&mut self, v: f64) {
        self.contact_max_correcting_vel = ::std::option::Option::Some(v);
    }

    pub fn get_contact_max_correcting_vel(&self) -> f64 {
        self.contact_max_correcting_vel.unwrap_or(0.)
    }

    // optional double contact_surface_layer = 10;

    pub fn clear_contact_surface_layer(&mut self) {
        self.contact_surface_layer = ::std::option::Option::None;
    }

    pub fn has_contact_surface_layer(&self) -> bool {
        self.contact_surface_layer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contact_surface_layer(&mut self, v: f64) {
        self.contact_surface_layer = ::std::option::Option::Some(v);
    }

    pub fn get_contact_surface_layer(&self) -> f64 {
        self.contact_surface_layer.unwrap_or(0.)
    }

    // optional .gazebo.msgs.Vector3d gravity = 11;

    pub fn clear_gravity(&mut self) {
        self.gravity.clear();
    }

    pub fn has_gravity(&self) -> bool {
        self.gravity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gravity(&mut self, v: super::vector3d::Vector3d) {
        self.gravity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gravity(&mut self) -> &mut super::vector3d::Vector3d {
        if self.gravity.is_none() {
            self.gravity.set_default();
        };
        self.gravity.as_mut().unwrap()
    }

    // Take field
    pub fn take_gravity(&mut self) -> super::vector3d::Vector3d {
        self.gravity.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_gravity(&self) -> &super::vector3d::Vector3d {
        self.gravity.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // optional bool enable_physics = 12;

    pub fn clear_enable_physics(&mut self) {
        self.enable_physics = ::std::option::Option::None;
    }

    pub fn has_enable_physics(&self) -> bool {
        self.enable_physics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enable_physics(&mut self, v: bool) {
        self.enable_physics = ::std::option::Option::Some(v);
    }

    pub fn get_enable_physics(&self) -> bool {
        self.enable_physics.unwrap_or(false)
    }

    // optional double real_time_factor = 13;

    pub fn clear_real_time_factor(&mut self) {
        self.real_time_factor = ::std::option::Option::None;
    }

    pub fn has_real_time_factor(&self) -> bool {
        self.real_time_factor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_real_time_factor(&mut self, v: f64) {
        self.real_time_factor = ::std::option::Option::Some(v);
    }

    pub fn get_real_time_factor(&self) -> f64 {
        self.real_time_factor.unwrap_or(0.)
    }

    // optional double real_time_update_rate = 14;

    pub fn clear_real_time_update_rate(&mut self) {
        self.real_time_update_rate = ::std::option::Option::None;
    }

    pub fn has_real_time_update_rate(&self) -> bool {
        self.real_time_update_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_real_time_update_rate(&mut self, v: f64) {
        self.real_time_update_rate = ::std::option::Option::Some(v);
    }

    pub fn get_real_time_update_rate(&self) -> f64 {
        self.real_time_update_rate.unwrap_or(0.)
    }

    // optional double max_step_size = 15;

    pub fn clear_max_step_size(&mut self) {
        self.max_step_size = ::std::option::Option::None;
    }

    pub fn has_max_step_size(&self) -> bool {
        self.max_step_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_step_size(&mut self, v: f64) {
        self.max_step_size = ::std::option::Option::Some(v);
    }

    pub fn get_max_step_size(&self) -> f64 {
        self.max_step_size.unwrap_or(0.)
    }

    // optional string profile_name = 16;

    pub fn clear_profile_name(&mut self) {
        self.profile_name.clear();
    }

    pub fn has_profile_name(&self) -> bool {
        self.profile_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_profile_name(&mut self, v: ::std::string::String) {
        self.profile_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_profile_name(&mut self) -> &mut ::std::string::String {
        if self.profile_name.is_none() {
            self.profile_name.set_default();
        };
        self.profile_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_profile_name(&mut self) -> ::std::string::String {
        self.profile_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_profile_name(&self) -> &str {
        match self.profile_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .gazebo.msgs.Vector3d magnetic_field = 17;

    pub fn clear_magnetic_field(&mut self) {
        self.magnetic_field.clear();
    }

    pub fn has_magnetic_field(&self) -> bool {
        self.magnetic_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magnetic_field(&mut self, v: super::vector3d::Vector3d) {
        self.magnetic_field = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_magnetic_field(&mut self) -> &mut super::vector3d::Vector3d {
        if self.magnetic_field.is_none() {
            self.magnetic_field.set_default();
        };
        self.magnetic_field.as_mut().unwrap()
    }

    // Take field
    pub fn take_magnetic_field(&mut self) -> super::vector3d::Vector3d {
        self.magnetic_field.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_magnetic_field(&self) -> &super::vector3d::Vector3d {
        self.magnetic_field.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }
}

impl ::protobuf::Message for Physics {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.solver_type));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.min_step_size = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.precon_iters = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.iters = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.sor = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.cfm = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.erp = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.contact_max_correcting_vel = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.contact_surface_layer = ::std::option::Option::Some(tmp);
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gravity));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.enable_physics = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.real_time_factor = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.real_time_update_rate = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.max_step_size = ::std::option::Option::Some(tmp);
                },
                16 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.profile_name));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.magnetic_field));
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
        for value in &self.solver_type {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.min_step_size.is_some() {
            my_size += 9;
        };
        for value in &self.precon_iters {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.iters {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.sor.is_some() {
            my_size += 9;
        };
        if self.cfm.is_some() {
            my_size += 9;
        };
        if self.erp.is_some() {
            my_size += 9;
        };
        if self.contact_max_correcting_vel.is_some() {
            my_size += 9;
        };
        if self.contact_surface_layer.is_some() {
            my_size += 9;
        };
        for value in &self.gravity {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.enable_physics.is_some() {
            my_size += 2;
        };
        if self.real_time_factor.is_some() {
            my_size += 9;
        };
        if self.real_time_update_rate.is_some() {
            my_size += 9;
        };
        if self.max_step_size.is_some() {
            my_size += 9;
        };
        for value in &self.profile_name {
            my_size += ::protobuf::rt::string_size(16, &value);
        };
        for value in &self.magnetic_field {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.solver_type.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.min_step_size {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.precon_iters {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.iters {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.sor {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.cfm {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.erp {
            try!(os.write_double(8, v));
        };
        if let Some(v) = self.contact_max_correcting_vel {
            try!(os.write_double(9, v));
        };
        if let Some(v) = self.contact_surface_layer {
            try!(os.write_double(10, v));
        };
        if let Some(v) = self.gravity.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.enable_physics {
            try!(os.write_bool(12, v));
        };
        if let Some(v) = self.real_time_factor {
            try!(os.write_double(13, v));
        };
        if let Some(v) = self.real_time_update_rate {
            try!(os.write_double(14, v));
        };
        if let Some(v) = self.max_step_size {
            try!(os.write_double(15, v));
        };
        if let Some(v) = self.profile_name.as_ref() {
            try!(os.write_string(16, &v));
        };
        if let Some(v) = self.magnetic_field.as_ref() {
            try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<Physics>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Physics {
    fn new() -> Physics {
        Physics::new()
    }

    fn descriptor_static(_: ::std::option::Option<Physics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Physics::has_field_type,
                    Physics::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "solver_type",
                    Physics::has_solver_type,
                    Physics::get_solver_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "min_step_size",
                    Physics::has_min_step_size,
                    Physics::get_min_step_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "precon_iters",
                    Physics::has_precon_iters,
                    Physics::get_precon_iters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "iters",
                    Physics::has_iters,
                    Physics::get_iters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "sor",
                    Physics::has_sor,
                    Physics::get_sor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "cfm",
                    Physics::has_cfm,
                    Physics::get_cfm,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "erp",
                    Physics::has_erp,
                    Physics::get_erp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "contact_max_correcting_vel",
                    Physics::has_contact_max_correcting_vel,
                    Physics::get_contact_max_correcting_vel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "contact_surface_layer",
                    Physics::has_contact_surface_layer,
                    Physics::get_contact_surface_layer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gravity",
                    Physics::has_gravity,
                    Physics::get_gravity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "enable_physics",
                    Physics::has_enable_physics,
                    Physics::get_enable_physics,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "real_time_factor",
                    Physics::has_real_time_factor,
                    Physics::get_real_time_factor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "real_time_update_rate",
                    Physics::has_real_time_update_rate,
                    Physics::get_real_time_update_rate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "max_step_size",
                    Physics::has_max_step_size,
                    Physics::get_max_step_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "profile_name",
                    Physics::has_profile_name,
                    Physics::get_profile_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "magnetic_field",
                    Physics::has_magnetic_field,
                    Physics::get_magnetic_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Physics>(
                    "Physics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Physics {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_solver_type();
        self.clear_min_step_size();
        self.clear_precon_iters();
        self.clear_iters();
        self.clear_sor();
        self.clear_cfm();
        self.clear_erp();
        self.clear_contact_max_correcting_vel();
        self.clear_contact_surface_layer();
        self.clear_gravity();
        self.clear_enable_physics();
        self.clear_real_time_factor();
        self.clear_real_time_update_rate();
        self.clear_max_step_size();
        self.clear_profile_name();
        self.clear_magnetic_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Physics {
    fn eq(&self, other: &Physics) -> bool {
        self.field_type == other.field_type &&
        self.solver_type == other.solver_type &&
        self.min_step_size == other.min_step_size &&
        self.precon_iters == other.precon_iters &&
        self.iters == other.iters &&
        self.sor == other.sor &&
        self.cfm == other.cfm &&
        self.erp == other.erp &&
        self.contact_max_correcting_vel == other.contact_max_correcting_vel &&
        self.contact_surface_layer == other.contact_surface_layer &&
        self.gravity == other.gravity &&
        self.enable_physics == other.enable_physics &&
        self.real_time_factor == other.real_time_factor &&
        self.real_time_update_rate == other.real_time_update_rate &&
        self.max_step_size == other.max_step_size &&
        self.profile_name == other.profile_name &&
        self.magnetic_field == other.magnetic_field &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Physics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Physics_Type {
    ODE = 1,
    BULLET = 2,
    SIMBODY = 3,
    DART = 4,
}

impl ::protobuf::ProtobufEnum for Physics_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Physics_Type> {
        match value {
            1 => ::std::option::Option::Some(Physics_Type::ODE),
            2 => ::std::option::Option::Some(Physics_Type::BULLET),
            3 => ::std::option::Option::Some(Physics_Type::SIMBODY),
            4 => ::std::option::Option::Some(Physics_Type::DART),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Physics_Type] = &[
            Physics_Type::ODE,
            Physics_Type::BULLET,
            Physics_Type::SIMBODY,
            Physics_Type::DART,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Physics_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Physics_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Physics_Type {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x70, 0x68, 0x79, 0x73, 0x69, 0x63, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0e, 0x76, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xfb, 0x03, 0x0a,
    0x07, 0x50, 0x68, 0x79, 0x73, 0x69, 0x63, 0x73, 0x12, 0x2c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e,
    0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x68, 0x79, 0x73, 0x69, 0x63, 0x73, 0x2e, 0x54, 0x79, 0x70,
    0x65, 0x3a, 0x03, 0x4f, 0x44, 0x45, 0x12, 0x13, 0x0a, 0x0b, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x15, 0x0a, 0x0d, 0x6d,
    0x69, 0x6e, 0x5f, 0x73, 0x74, 0x65, 0x70, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x01, 0x12, 0x14, 0x0a, 0x0c, 0x70, 0x72, 0x65, 0x63, 0x6f, 0x6e, 0x5f, 0x69, 0x74, 0x65,
    0x72, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x72,
    0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x6f, 0x72, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x01, 0x12, 0x0b, 0x0a, 0x03, 0x63, 0x66, 0x6d, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x01, 0x12, 0x0b, 0x0a, 0x03, 0x65, 0x72, 0x70, 0x18, 0x08, 0x20, 0x01, 0x28, 0x01, 0x12, 0x22,
    0x0a, 0x1a, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6f,
    0x72, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x76, 0x65, 0x6c, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x01, 0x12, 0x1d, 0x0a, 0x15, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x5f, 0x73, 0x75,
    0x72, 0x66, 0x61, 0x63, 0x65, 0x5f, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x01, 0x12, 0x26, 0x0a, 0x07, 0x67, 0x72, 0x61, 0x76, 0x69, 0x74, 0x79, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x12, 0x16, 0x0a, 0x0e, 0x65, 0x6e, 0x61,
    0x62, 0x6c, 0x65, 0x5f, 0x70, 0x68, 0x79, 0x73, 0x69, 0x63, 0x73, 0x18, 0x0c, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x18, 0x0a, 0x10, 0x72, 0x65, 0x61, 0x6c, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x66,
    0x61, 0x63, 0x74, 0x6f, 0x72, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1d, 0x0a, 0x15, 0x72,
    0x65, 0x61, 0x6c, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f,
    0x72, 0x61, 0x74, 0x65, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x01, 0x12, 0x15, 0x0a, 0x0d, 0x6d, 0x61,
    0x78, 0x5f, 0x73, 0x74, 0x65, 0x70, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x0f, 0x20, 0x01, 0x28,
    0x01, 0x12, 0x14, 0x0a, 0x0c, 0x70, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x09, 0x12, 0x2d, 0x0a, 0x0e, 0x6d, 0x61, 0x67, 0x6e, 0x65,
    0x74, 0x69, 0x63, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x22, 0x32, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x07,
    0x0a, 0x03, 0x4f, 0x44, 0x45, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x42, 0x55, 0x4c, 0x4c, 0x45,
    0x54, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x49, 0x4d, 0x42, 0x4f, 0x44, 0x59, 0x10, 0x03,
    0x12, 0x08, 0x0a, 0x04, 0x44, 0x41, 0x52, 0x54, 0x10, 0x04, 0x4a, 0x9b, 0x0c, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x27, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x09, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09,
    0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x02, 0x11, 0x03,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x07, 0x0b, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x07, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x0a, 0x0b, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0e, 0x0d, 0x0e, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x10, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x10, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x12, 0x02, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x12, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x10,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x2f, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x12, 0x30, 0x3d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x07, 0x12, 0x03, 0x12, 0x39, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x14, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14,
    0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x2f, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x15, 0x02, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x15, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x15, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x16,
    0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x16, 0x11, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x16, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x17, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x17, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x17, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x17, 0x2f, 0x30, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x18, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x18, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x18, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x18, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x19, 0x02,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x19, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x19, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x07, 0x12, 0x03, 0x1a, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x1a,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1a, 0x2f, 0x30, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x1b, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x05, 0x12, 0x03, 0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x1b, 0x12, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x1b, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x1c, 0x02, 0x32,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1c, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x1c, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a,
    0x12, 0x03, 0x1d, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03,
    0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x1d, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x1d, 0x14, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x1d, 0x2f, 0x31, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x1e, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0b, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b,
    0x05, 0x12, 0x03, 0x1e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12,
    0x03, 0x1e, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x1e,
    0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x1f, 0x02, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x1f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x1f, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0c, 0x03, 0x12, 0x03, 0x1f, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12,
    0x03, 0x20, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x20,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x20, 0x12, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x20, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x21, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0e, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x05,
    0x12, 0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03,
    0x21, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x21, 0x2f,
    0x31, 0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x23, 0x02, 0x32, 0x1a, 0x41,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x70, 0x68, 0x79, 0x73, 0x69, 0x63, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x66, 0x69, 0x6c,
    0x65, 0x20, 0x28, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x66, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x79, 0x70, 0x65, 0x29,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x23, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x23, 0x2f, 0x31, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x10, 0x12, 0x03, 0x26, 0x02, 0x32, 0x1a, 0x18, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66,
    0x20, 0x4d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x69, 0x63, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x06, 0x12, 0x03, 0x26, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x26, 0x14, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x10, 0x03, 0x12, 0x03, 0x26, 0x2f, 0x31,
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
