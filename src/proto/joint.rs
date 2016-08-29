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
pub struct Joint {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    id: ::std::option::Option<u32>,
    angle: ::std::vec::Vec<f64>,
    field_type: ::std::option::Option<Joint_Type>,
    parent: ::protobuf::SingularField<::std::string::String>,
    parent_id: ::std::option::Option<u32>,
    child: ::protobuf::SingularField<::std::string::String>,
    child_id: ::std::option::Option<u32>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    axis1: ::protobuf::SingularPtrField<super::axis::Axis>,
    axis2: ::protobuf::SingularPtrField<super::axis::Axis>,
    cfm: ::std::option::Option<f64>,
    bounce: ::std::option::Option<f64>,
    velocity: ::std::option::Option<f64>,
    fudge_factor: ::std::option::Option<f64>,
    limit_cfm: ::std::option::Option<f64>,
    limit_erp: ::std::option::Option<f64>,
    suspension_cfm: ::std::option::Option<f64>,
    suspension_erp: ::std::option::Option<f64>,
    gearbox: ::protobuf::SingularPtrField<Joint_Gearbox>,
    screw: ::protobuf::SingularPtrField<Joint_Screw>,
    sensor: ::protobuf::RepeatedField<super::sensor::Sensor>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Joint {}

impl Joint {
    pub fn new() -> Joint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Joint {
        static mut instance: ::protobuf::lazy::Lazy<Joint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Joint,
        };
        unsafe {
            instance.get(|| {
                Joint {
                    name: ::protobuf::SingularField::none(),
                    id: ::std::option::Option::None,
                    angle: ::std::vec::Vec::new(),
                    field_type: ::std::option::Option::None,
                    parent: ::protobuf::SingularField::none(),
                    parent_id: ::std::option::Option::None,
                    child: ::protobuf::SingularField::none(),
                    child_id: ::std::option::Option::None,
                    pose: ::protobuf::SingularPtrField::none(),
                    axis1: ::protobuf::SingularPtrField::none(),
                    axis2: ::protobuf::SingularPtrField::none(),
                    cfm: ::std::option::Option::None,
                    bounce: ::std::option::Option::None,
                    velocity: ::std::option::Option::None,
                    fudge_factor: ::std::option::Option::None,
                    limit_cfm: ::std::option::Option::None,
                    limit_erp: ::std::option::Option::None,
                    suspension_cfm: ::std::option::Option::None,
                    suspension_erp: ::std::option::Option::None,
                    gearbox: ::protobuf::SingularPtrField::none(),
                    screw: ::protobuf::SingularPtrField::none(),
                    sensor: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 id = 2;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // repeated double angle = 3;

    pub fn clear_angle(&mut self) {
        self.angle.clear();
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: ::std::vec::Vec<f64>) {
        self.angle = v;
    }

    // Mutable pointer to the field.
    pub fn mut_angle(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.angle
    }

    // Take field
    pub fn take_angle(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.angle, ::std::vec::Vec::new())
    }

    pub fn get_angle(&self) -> &[f64] {
        &self.angle
    }

    // optional .gazebo.msgs.Joint.Type type = 4;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Joint_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Joint_Type {
        self.field_type.unwrap_or(Joint_Type::REVOLUTE)
    }

    // optional string parent = 5;

    pub fn clear_parent(&mut self) {
        self.parent.clear();
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent(&mut self, v: ::std::string::String) {
        self.parent = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent(&mut self) -> &mut ::std::string::String {
        if self.parent.is_none() {
            self.parent.set_default();
        };
        self.parent.as_mut().unwrap()
    }

    // Take field
    pub fn take_parent(&mut self) -> ::std::string::String {
        self.parent.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_parent(&self) -> &str {
        match self.parent.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 parent_id = 6;

    pub fn clear_parent_id(&mut self) {
        self.parent_id = ::std::option::Option::None;
    }

    pub fn has_parent_id(&self) -> bool {
        self.parent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_id(&mut self, v: u32) {
        self.parent_id = ::std::option::Option::Some(v);
    }

    pub fn get_parent_id(&self) -> u32 {
        self.parent_id.unwrap_or(0)
    }

    // optional string child = 7;

    pub fn clear_child(&mut self) {
        self.child.clear();
    }

    pub fn has_child(&self) -> bool {
        self.child.is_some()
    }

    // Param is passed by value, moved
    pub fn set_child(&mut self, v: ::std::string::String) {
        self.child = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_child(&mut self) -> &mut ::std::string::String {
        if self.child.is_none() {
            self.child.set_default();
        };
        self.child.as_mut().unwrap()
    }

    // Take field
    pub fn take_child(&mut self) -> ::std::string::String {
        self.child.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_child(&self) -> &str {
        match self.child.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 child_id = 8;

    pub fn clear_child_id(&mut self) {
        self.child_id = ::std::option::Option::None;
    }

    pub fn has_child_id(&self) -> bool {
        self.child_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_child_id(&mut self, v: u32) {
        self.child_id = ::std::option::Option::Some(v);
    }

    pub fn get_child_id(&self) -> u32 {
        self.child_id.unwrap_or(0)
    }

    // optional .gazebo.msgs.Pose pose = 9;

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

    // optional .gazebo.msgs.Axis axis1 = 10;

    pub fn clear_axis1(&mut self) {
        self.axis1.clear();
    }

    pub fn has_axis1(&self) -> bool {
        self.axis1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_axis1(&mut self, v: super::axis::Axis) {
        self.axis1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_axis1(&mut self) -> &mut super::axis::Axis {
        if self.axis1.is_none() {
            self.axis1.set_default();
        };
        self.axis1.as_mut().unwrap()
    }

    // Take field
    pub fn take_axis1(&mut self) -> super::axis::Axis {
        self.axis1.take().unwrap_or_else(|| super::axis::Axis::new())
    }

    pub fn get_axis1(&self) -> &super::axis::Axis {
        self.axis1.as_ref().unwrap_or_else(|| super::axis::Axis::default_instance())
    }

    // optional .gazebo.msgs.Axis axis2 = 11;

    pub fn clear_axis2(&mut self) {
        self.axis2.clear();
    }

    pub fn has_axis2(&self) -> bool {
        self.axis2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_axis2(&mut self, v: super::axis::Axis) {
        self.axis2 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_axis2(&mut self) -> &mut super::axis::Axis {
        if self.axis2.is_none() {
            self.axis2.set_default();
        };
        self.axis2.as_mut().unwrap()
    }

    // Take field
    pub fn take_axis2(&mut self) -> super::axis::Axis {
        self.axis2.take().unwrap_or_else(|| super::axis::Axis::new())
    }

    pub fn get_axis2(&self) -> &super::axis::Axis {
        self.axis2.as_ref().unwrap_or_else(|| super::axis::Axis::default_instance())
    }

    // optional double cfm = 12;

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

    // optional double bounce = 13;

    pub fn clear_bounce(&mut self) {
        self.bounce = ::std::option::Option::None;
    }

    pub fn has_bounce(&self) -> bool {
        self.bounce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bounce(&mut self, v: f64) {
        self.bounce = ::std::option::Option::Some(v);
    }

    pub fn get_bounce(&self) -> f64 {
        self.bounce.unwrap_or(0.)
    }

    // optional double velocity = 14;

    pub fn clear_velocity(&mut self) {
        self.velocity = ::std::option::Option::None;
    }

    pub fn has_velocity(&self) -> bool {
        self.velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity(&mut self, v: f64) {
        self.velocity = ::std::option::Option::Some(v);
    }

    pub fn get_velocity(&self) -> f64 {
        self.velocity.unwrap_or(0.)
    }

    // optional double fudge_factor = 15;

    pub fn clear_fudge_factor(&mut self) {
        self.fudge_factor = ::std::option::Option::None;
    }

    pub fn has_fudge_factor(&self) -> bool {
        self.fudge_factor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fudge_factor(&mut self, v: f64) {
        self.fudge_factor = ::std::option::Option::Some(v);
    }

    pub fn get_fudge_factor(&self) -> f64 {
        self.fudge_factor.unwrap_or(0.)
    }

    // optional double limit_cfm = 16;

    pub fn clear_limit_cfm(&mut self) {
        self.limit_cfm = ::std::option::Option::None;
    }

    pub fn has_limit_cfm(&self) -> bool {
        self.limit_cfm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit_cfm(&mut self, v: f64) {
        self.limit_cfm = ::std::option::Option::Some(v);
    }

    pub fn get_limit_cfm(&self) -> f64 {
        self.limit_cfm.unwrap_or(0.)
    }

    // optional double limit_erp = 17;

    pub fn clear_limit_erp(&mut self) {
        self.limit_erp = ::std::option::Option::None;
    }

    pub fn has_limit_erp(&self) -> bool {
        self.limit_erp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit_erp(&mut self, v: f64) {
        self.limit_erp = ::std::option::Option::Some(v);
    }

    pub fn get_limit_erp(&self) -> f64 {
        self.limit_erp.unwrap_or(0.)
    }

    // optional double suspension_cfm = 18;

    pub fn clear_suspension_cfm(&mut self) {
        self.suspension_cfm = ::std::option::Option::None;
    }

    pub fn has_suspension_cfm(&self) -> bool {
        self.suspension_cfm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suspension_cfm(&mut self, v: f64) {
        self.suspension_cfm = ::std::option::Option::Some(v);
    }

    pub fn get_suspension_cfm(&self) -> f64 {
        self.suspension_cfm.unwrap_or(0.)
    }

    // optional double suspension_erp = 19;

    pub fn clear_suspension_erp(&mut self) {
        self.suspension_erp = ::std::option::Option::None;
    }

    pub fn has_suspension_erp(&self) -> bool {
        self.suspension_erp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suspension_erp(&mut self, v: f64) {
        self.suspension_erp = ::std::option::Option::Some(v);
    }

    pub fn get_suspension_erp(&self) -> f64 {
        self.suspension_erp.unwrap_or(0.)
    }

    // optional .gazebo.msgs.Joint.Gearbox gearbox = 20;

    pub fn clear_gearbox(&mut self) {
        self.gearbox.clear();
    }

    pub fn has_gearbox(&self) -> bool {
        self.gearbox.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gearbox(&mut self, v: Joint_Gearbox) {
        self.gearbox = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gearbox(&mut self) -> &mut Joint_Gearbox {
        if self.gearbox.is_none() {
            self.gearbox.set_default();
        };
        self.gearbox.as_mut().unwrap()
    }

    // Take field
    pub fn take_gearbox(&mut self) -> Joint_Gearbox {
        self.gearbox.take().unwrap_or_else(|| Joint_Gearbox::new())
    }

    pub fn get_gearbox(&self) -> &Joint_Gearbox {
        self.gearbox.as_ref().unwrap_or_else(|| Joint_Gearbox::default_instance())
    }

    // optional .gazebo.msgs.Joint.Screw screw = 21;

    pub fn clear_screw(&mut self) {
        self.screw.clear();
    }

    pub fn has_screw(&self) -> bool {
        self.screw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_screw(&mut self, v: Joint_Screw) {
        self.screw = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_screw(&mut self) -> &mut Joint_Screw {
        if self.screw.is_none() {
            self.screw.set_default();
        };
        self.screw.as_mut().unwrap()
    }

    // Take field
    pub fn take_screw(&mut self) -> Joint_Screw {
        self.screw.take().unwrap_or_else(|| Joint_Screw::new())
    }

    pub fn get_screw(&self) -> &Joint_Screw {
        self.screw.as_ref().unwrap_or_else(|| Joint_Screw::default_instance())
    }

    // repeated .gazebo.msgs.Sensor sensor = 22;

    pub fn clear_sensor(&mut self) {
        self.sensor.clear();
    }

    // Param is passed by value, moved
    pub fn set_sensor(&mut self, v: ::protobuf::RepeatedField<super::sensor::Sensor>) {
        self.sensor = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sensor(&mut self) -> &mut ::protobuf::RepeatedField<super::sensor::Sensor> {
        &mut self.sensor
    }

    // Take field
    pub fn take_sensor(&mut self) -> ::protobuf::RepeatedField<super::sensor::Sensor> {
        ::std::mem::replace(&mut self.sensor, ::protobuf::RepeatedField::new())
    }

    pub fn get_sensor(&self) -> &[super::sensor::Sensor] {
        &self.sensor
    }
}

impl ::protobuf::Message for Joint {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.angle));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.parent));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.parent_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.child));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.child_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.axis1));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.axis2));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.cfm = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.bounce = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.velocity = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.fudge_factor = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.limit_cfm = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.limit_erp = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.suspension_cfm = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.suspension_erp = ::std::option::Option::Some(tmp);
                },
                20 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gearbox));
                },
                21 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.screw));
                },
                22 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sensor));
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 9 * self.angle.len() as u32;
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in &self.parent {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.parent_id {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.child {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        for value in &self.child_id {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.axis1 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.axis2 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.cfm.is_some() {
            my_size += 9;
        };
        if self.bounce.is_some() {
            my_size += 9;
        };
        if self.velocity.is_some() {
            my_size += 9;
        };
        if self.fudge_factor.is_some() {
            my_size += 9;
        };
        if self.limit_cfm.is_some() {
            my_size += 10;
        };
        if self.limit_erp.is_some() {
            my_size += 10;
        };
        if self.suspension_cfm.is_some() {
            my_size += 10;
        };
        if self.suspension_erp.is_some() {
            my_size += 10;
        };
        for value in &self.gearbox {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.screw {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sensor {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.id {
            try!(os.write_uint32(2, v));
        };
        for v in &self.angle {
            try!(os.write_double(3, *v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(4, v.value()));
        };
        if let Some(v) = self.parent.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.parent_id {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.child.as_ref() {
            try!(os.write_string(7, &v));
        };
        if let Some(v) = self.child_id {
            try!(os.write_uint32(8, v));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.axis1.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.axis2.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cfm {
            try!(os.write_double(12, v));
        };
        if let Some(v) = self.bounce {
            try!(os.write_double(13, v));
        };
        if let Some(v) = self.velocity {
            try!(os.write_double(14, v));
        };
        if let Some(v) = self.fudge_factor {
            try!(os.write_double(15, v));
        };
        if let Some(v) = self.limit_cfm {
            try!(os.write_double(16, v));
        };
        if let Some(v) = self.limit_erp {
            try!(os.write_double(17, v));
        };
        if let Some(v) = self.suspension_cfm {
            try!(os.write_double(18, v));
        };
        if let Some(v) = self.suspension_erp {
            try!(os.write_double(19, v));
        };
        if let Some(v) = self.gearbox.as_ref() {
            try!(os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.screw.as_ref() {
            try!(os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.sensor {
            try!(os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Joint>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Joint {
    fn new() -> Joint {
        Joint::new()
    }

    fn descriptor_static(_: ::std::option::Option<Joint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Joint::has_name,
                    Joint::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    Joint::has_id,
                    Joint::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "angle",
                    Joint::get_angle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Joint::has_field_type,
                    Joint::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "parent",
                    Joint::has_parent,
                    Joint::get_parent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "parent_id",
                    Joint::has_parent_id,
                    Joint::get_parent_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "child",
                    Joint::has_child,
                    Joint::get_child,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "child_id",
                    Joint::has_child_id,
                    Joint::get_child_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Joint::has_pose,
                    Joint::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "axis1",
                    Joint::has_axis1,
                    Joint::get_axis1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "axis2",
                    Joint::has_axis2,
                    Joint::get_axis2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "cfm",
                    Joint::has_cfm,
                    Joint::get_cfm,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "bounce",
                    Joint::has_bounce,
                    Joint::get_bounce,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "velocity",
                    Joint::has_velocity,
                    Joint::get_velocity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "fudge_factor",
                    Joint::has_fudge_factor,
                    Joint::get_fudge_factor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "limit_cfm",
                    Joint::has_limit_cfm,
                    Joint::get_limit_cfm,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "limit_erp",
                    Joint::has_limit_erp,
                    Joint::get_limit_erp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "suspension_cfm",
                    Joint::has_suspension_cfm,
                    Joint::get_suspension_cfm,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "suspension_erp",
                    Joint::has_suspension_erp,
                    Joint::get_suspension_erp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gearbox",
                    Joint::has_gearbox,
                    Joint::get_gearbox,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "screw",
                    Joint::has_screw,
                    Joint::get_screw,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "sensor",
                    Joint::get_sensor,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Joint>(
                    "Joint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Joint {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_angle();
        self.clear_field_type();
        self.clear_parent();
        self.clear_parent_id();
        self.clear_child();
        self.clear_child_id();
        self.clear_pose();
        self.clear_axis1();
        self.clear_axis2();
        self.clear_cfm();
        self.clear_bounce();
        self.clear_velocity();
        self.clear_fudge_factor();
        self.clear_limit_cfm();
        self.clear_limit_erp();
        self.clear_suspension_cfm();
        self.clear_suspension_erp();
        self.clear_gearbox();
        self.clear_screw();
        self.clear_sensor();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Joint {
    fn eq(&self, other: &Joint) -> bool {
        self.name == other.name &&
        self.id == other.id &&
        self.angle == other.angle &&
        self.field_type == other.field_type &&
        self.parent == other.parent &&
        self.parent_id == other.parent_id &&
        self.child == other.child &&
        self.child_id == other.child_id &&
        self.pose == other.pose &&
        self.axis1 == other.axis1 &&
        self.axis2 == other.axis2 &&
        self.cfm == other.cfm &&
        self.bounce == other.bounce &&
        self.velocity == other.velocity &&
        self.fudge_factor == other.fudge_factor &&
        self.limit_cfm == other.limit_cfm &&
        self.limit_erp == other.limit_erp &&
        self.suspension_cfm == other.suspension_cfm &&
        self.suspension_erp == other.suspension_erp &&
        self.gearbox == other.gearbox &&
        self.screw == other.screw &&
        self.sensor == other.sensor &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Joint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Joint_Gearbox {
    // message fields
    gearbox_reference_body: ::protobuf::SingularField<::std::string::String>,
    gearbox_ratio: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Joint_Gearbox {}

impl Joint_Gearbox {
    pub fn new() -> Joint_Gearbox {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Joint_Gearbox {
        static mut instance: ::protobuf::lazy::Lazy<Joint_Gearbox> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Joint_Gearbox,
        };
        unsafe {
            instance.get(|| {
                Joint_Gearbox {
                    gearbox_reference_body: ::protobuf::SingularField::none(),
                    gearbox_ratio: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string gearbox_reference_body = 1;

    pub fn clear_gearbox_reference_body(&mut self) {
        self.gearbox_reference_body.clear();
    }

    pub fn has_gearbox_reference_body(&self) -> bool {
        self.gearbox_reference_body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gearbox_reference_body(&mut self, v: ::std::string::String) {
        self.gearbox_reference_body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gearbox_reference_body(&mut self) -> &mut ::std::string::String {
        if self.gearbox_reference_body.is_none() {
            self.gearbox_reference_body.set_default();
        };
        self.gearbox_reference_body.as_mut().unwrap()
    }

    // Take field
    pub fn take_gearbox_reference_body(&mut self) -> ::std::string::String {
        self.gearbox_reference_body.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gearbox_reference_body(&self) -> &str {
        match self.gearbox_reference_body.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required double gearbox_ratio = 2;

    pub fn clear_gearbox_ratio(&mut self) {
        self.gearbox_ratio = ::std::option::Option::None;
    }

    pub fn has_gearbox_ratio(&self) -> bool {
        self.gearbox_ratio.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gearbox_ratio(&mut self, v: f64) {
        self.gearbox_ratio = ::std::option::Option::Some(v);
    }

    pub fn get_gearbox_ratio(&self) -> f64 {
        self.gearbox_ratio.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Joint_Gearbox {
    fn is_initialized(&self) -> bool {
        if self.gearbox_reference_body.is_none() {
            return false;
        };
        if self.gearbox_ratio.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gearbox_reference_body));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.gearbox_ratio = ::std::option::Option::Some(tmp);
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
        for value in &self.gearbox_reference_body {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.gearbox_ratio.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gearbox_reference_body.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.gearbox_ratio {
            try!(os.write_double(2, v));
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
        ::std::any::TypeId::of::<Joint_Gearbox>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Joint_Gearbox {
    fn new() -> Joint_Gearbox {
        Joint_Gearbox::new()
    }

    fn descriptor_static(_: ::std::option::Option<Joint_Gearbox>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "gearbox_reference_body",
                    Joint_Gearbox::has_gearbox_reference_body,
                    Joint_Gearbox::get_gearbox_reference_body,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "gearbox_ratio",
                    Joint_Gearbox::has_gearbox_ratio,
                    Joint_Gearbox::get_gearbox_ratio,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Joint_Gearbox>(
                    "Joint_Gearbox",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Joint_Gearbox {
    fn clear(&mut self) {
        self.clear_gearbox_reference_body();
        self.clear_gearbox_ratio();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Joint_Gearbox {
    fn eq(&self, other: &Joint_Gearbox) -> bool {
        self.gearbox_reference_body == other.gearbox_reference_body &&
        self.gearbox_ratio == other.gearbox_ratio &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Joint_Gearbox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Joint_Screw {
    // message fields
    thread_pitch: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Joint_Screw {}

impl Joint_Screw {
    pub fn new() -> Joint_Screw {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Joint_Screw {
        static mut instance: ::protobuf::lazy::Lazy<Joint_Screw> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Joint_Screw,
        };
        unsafe {
            instance.get(|| {
                Joint_Screw {
                    thread_pitch: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double thread_pitch = 1;

    pub fn clear_thread_pitch(&mut self) {
        self.thread_pitch = ::std::option::Option::None;
    }

    pub fn has_thread_pitch(&self) -> bool {
        self.thread_pitch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_thread_pitch(&mut self, v: f64) {
        self.thread_pitch = ::std::option::Option::Some(v);
    }

    pub fn get_thread_pitch(&self) -> f64 {
        self.thread_pitch.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Joint_Screw {
    fn is_initialized(&self) -> bool {
        if self.thread_pitch.is_none() {
            return false;
        };
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
                    self.thread_pitch = ::std::option::Option::Some(tmp);
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
        if self.thread_pitch.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.thread_pitch {
            try!(os.write_double(1, v));
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
        ::std::any::TypeId::of::<Joint_Screw>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Joint_Screw {
    fn new() -> Joint_Screw {
        Joint_Screw::new()
    }

    fn descriptor_static(_: ::std::option::Option<Joint_Screw>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "thread_pitch",
                    Joint_Screw::has_thread_pitch,
                    Joint_Screw::get_thread_pitch,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Joint_Screw>(
                    "Joint_Screw",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Joint_Screw {
    fn clear(&mut self) {
        self.clear_thread_pitch();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Joint_Screw {
    fn eq(&self, other: &Joint_Screw) -> bool {
        self.thread_pitch == other.thread_pitch &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Joint_Screw {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Joint_Type {
    REVOLUTE = 1,
    REVOLUTE2 = 2,
    PRISMATIC = 3,
    UNIVERSAL = 4,
    BALL = 5,
    SCREW = 6,
    GEARBOX = 7,
    FIXED = 8,
}

impl ::protobuf::ProtobufEnum for Joint_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Joint_Type> {
        match value {
            1 => ::std::option::Option::Some(Joint_Type::REVOLUTE),
            2 => ::std::option::Option::Some(Joint_Type::REVOLUTE2),
            3 => ::std::option::Option::Some(Joint_Type::PRISMATIC),
            4 => ::std::option::Option::Some(Joint_Type::UNIVERSAL),
            5 => ::std::option::Option::Some(Joint_Type::BALL),
            6 => ::std::option::Option::Some(Joint_Type::SCREW),
            7 => ::std::option::Option::Some(Joint_Type::GEARBOX),
            8 => ::std::option::Option::Some(Joint_Type::FIXED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Joint_Type] = &[
            Joint_Type::REVOLUTE,
            Joint_Type::REVOLUTE2,
            Joint_Type::PRISMATIC,
            Joint_Type::UNIVERSAL,
            Joint_Type::BALL,
            Joint_Type::SCREW,
            Joint_Type::GEARBOX,
            Joint_Type::FIXED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Joint_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Joint_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Joint_Type {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x61, 0x78, 0x69, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x0c, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0xe7, 0x05, 0x0a, 0x05, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x0d, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x01, 0x12, 0x25, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x17, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0e, 0x0a, 0x06, 0x70, 0x61,
    0x72, 0x65, 0x6e, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x61,
    0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0d, 0x0a,
    0x05, 0x63, 0x68, 0x69, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08,
    0x63, 0x68, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1f,
    0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12,
    0x20, 0x0a, 0x05, 0x61, 0x78, 0x69, 0x73, 0x31, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x41, 0x78, 0x69,
    0x73, 0x12, 0x20, 0x0a, 0x05, 0x61, 0x78, 0x69, 0x73, 0x32, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x41,
    0x78, 0x69, 0x73, 0x12, 0x0b, 0x0a, 0x03, 0x63, 0x66, 0x6d, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x01,
    0x12, 0x0e, 0x0a, 0x06, 0x62, 0x6f, 0x75, 0x6e, 0x63, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x01,
    0x12, 0x10, 0x0a, 0x08, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x0e, 0x20, 0x01,
    0x28, 0x01, 0x12, 0x14, 0x0a, 0x0c, 0x66, 0x75, 0x64, 0x67, 0x65, 0x5f, 0x66, 0x61, 0x63, 0x74,
    0x6f, 0x72, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x6c, 0x69, 0x6d, 0x69,
    0x74, 0x5f, 0x63, 0x66, 0x6d, 0x18, 0x10, 0x20, 0x01, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x5f, 0x65, 0x72, 0x70, 0x18, 0x11, 0x20, 0x01, 0x28, 0x01, 0x12, 0x16,
    0x0a, 0x0e, 0x73, 0x75, 0x73, 0x70, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x66, 0x6d,
    0x18, 0x12, 0x20, 0x01, 0x28, 0x01, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x75, 0x73, 0x70, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x72, 0x70, 0x18, 0x13, 0x20, 0x01, 0x28, 0x01, 0x12, 0x2b,
    0x0a, 0x07, 0x67, 0x65, 0x61, 0x72, 0x62, 0x6f, 0x78, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4a, 0x6f,
    0x69, 0x6e, 0x74, 0x2e, 0x47, 0x65, 0x61, 0x72, 0x62, 0x6f, 0x78, 0x12, 0x27, 0x0a, 0x05, 0x73,
    0x63, 0x72, 0x65, 0x77, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x2e, 0x53,
    0x63, 0x72, 0x65, 0x77, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x18, 0x16,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x2e, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x1a, 0x40, 0x0a, 0x07, 0x47, 0x65, 0x61,
    0x72, 0x62, 0x6f, 0x78, 0x12, 0x1e, 0x0a, 0x16, 0x67, 0x65, 0x61, 0x72, 0x62, 0x6f, 0x78, 0x5f,
    0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x15, 0x0a, 0x0d, 0x67, 0x65, 0x61, 0x72, 0x62, 0x6f, 0x78, 0x5f,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x1a, 0x1d, 0x0a, 0x05, 0x53,
    0x63, 0x72, 0x65, 0x77, 0x12, 0x14, 0x0a, 0x0c, 0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x5f, 0x70,
    0x69, 0x74, 0x63, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x22, 0x6e, 0x0a, 0x04, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x45, 0x56, 0x4f, 0x4c, 0x55, 0x54, 0x45, 0x10, 0x01,
    0x12, 0x0d, 0x0a, 0x09, 0x52, 0x45, 0x56, 0x4f, 0x4c, 0x55, 0x54, 0x45, 0x32, 0x10, 0x02, 0x12,
    0x0d, 0x0a, 0x09, 0x50, 0x52, 0x49, 0x53, 0x4d, 0x41, 0x54, 0x49, 0x43, 0x10, 0x03, 0x12, 0x0d,
    0x0a, 0x09, 0x55, 0x4e, 0x49, 0x56, 0x45, 0x52, 0x53, 0x41, 0x4c, 0x10, 0x04, 0x12, 0x08, 0x0a,
    0x04, 0x42, 0x41, 0x4c, 0x4c, 0x10, 0x05, 0x12, 0x09, 0x0a, 0x05, 0x53, 0x43, 0x52, 0x45, 0x57,
    0x10, 0x06, 0x12, 0x0b, 0x0a, 0x07, 0x47, 0x45, 0x41, 0x52, 0x42, 0x4f, 0x58, 0x10, 0x07, 0x12,
    0x09, 0x0a, 0x05, 0x46, 0x49, 0x58, 0x45, 0x44, 0x10, 0x08, 0x4a, 0xde, 0x12, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x42, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x08, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09, 0x07, 0x15,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x42, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00,
    0x12, 0x04, 0x0e, 0x02, 0x15, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12,
    0x03, 0x0e, 0x0a, 0x11, 0x0a, 0x3a, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x11, 0x04, 0x2f, 0x1a, 0x2b, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x47, 0x65,
    0x61, 0x72, 0x62, 0x6f, 0x78, 0x20, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x66, 0x65,
    0x72, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x6c, 0x69, 0x6e, 0x6b, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x0d, 0x13,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x14, 0x2a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x2d, 0x2e,
    0x0a, 0x27, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x14, 0x04, 0x26, 0x1a,
    0x18, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x47, 0x65, 0x61, 0x72, 0x62, 0x6f,
    0x78, 0x20, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x14, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x14, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03,
    0x01, 0x12, 0x04, 0x17, 0x02, 0x1b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x01, 0x01,
    0x12, 0x03, 0x17, 0x0a, 0x0f, 0x0a, 0x32, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x1a, 0x04, 0x25, 0x1a, 0x23, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x53,
    0x63, 0x72, 0x65, 0x77, 0x20, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x72, 0x65, 0x61,
    0x64, 0x20, 0x70, 0x69, 0x74, 0x63, 0x68, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x14, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04,
    0x00, 0x12, 0x04, 0x1d, 0x02, 0x27, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x1d, 0x07, 0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x1f, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x1f, 0x10, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x20, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x20, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x20, 0x10, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x21,
    0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21,
    0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x21,
    0x10, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x22, 0x04,
    0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x22, 0x04,
    0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x22, 0x10,
    0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x23, 0x04, 0x12,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x23, 0x04, 0x08,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x23, 0x10, 0x11,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x24, 0x04, 0x12, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x24, 0x04, 0x09, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x24, 0x10, 0x11, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x25, 0x04, 0x12, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x25, 0x04, 0x0b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x25, 0x10, 0x11, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x26, 0x04, 0x12, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x26, 0x04, 0x09, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x26, 0x10, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x29, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29,
    0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x2a, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x2b, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2b, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x12, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x2c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x2c, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2c, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x2d, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x2d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x2e, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x2e, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2e, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x06, 0x12, 0x03, 0x2f, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x2f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x2f,
    0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2f, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x30, 0x02, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x30, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x30, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x30, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x31,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x31, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x31, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x31, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x09, 0x12, 0x03, 0x32, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04,
    0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12, 0x03,
    0x32, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x32, 0x10,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x32, 0x23, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x33, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0a, 0x06, 0x12, 0x03, 0x33, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x33, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12,
    0x03, 0x33, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x35, 0x02,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x35, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x35, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x35, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0c, 0x12, 0x03, 0x36, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12,
    0x03, 0x36, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x36,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x36, 0x12, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x36, 0x23, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x37, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0d, 0x05, 0x12, 0x03, 0x37, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01,
    0x12, 0x03, 0x37, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03,
    0x37, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x38, 0x02, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x38, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x38, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0e, 0x03, 0x12, 0x03, 0x38, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0f,
    0x12, 0x03, 0x39, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x04, 0x12, 0x03,
    0x39, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x39, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x39, 0x12, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x39, 0x23, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x10, 0x12, 0x03, 0x3a, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x10, 0x04, 0x12, 0x03, 0x3a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10,
    0x05, 0x12, 0x03, 0x3a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x01, 0x12,
    0x03, 0x3a, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x03, 0x12, 0x03, 0x3a,
    0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x11, 0x12, 0x03, 0x3b, 0x02, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x04, 0x12, 0x03, 0x3b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x11, 0x05, 0x12, 0x03, 0x3b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x3b, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x11, 0x03, 0x12, 0x03, 0x3b, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x12, 0x12,
    0x03, 0x3c, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x04, 0x12, 0x03, 0x3c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x05, 0x12, 0x03, 0x3c, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x3c, 0x12, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x03, 0x12, 0x03, 0x3c, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x13, 0x12, 0x03, 0x3e, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x13, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x06,
    0x12, 0x03, 0x3e, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x01, 0x12, 0x03,
    0x3e, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x03, 0x12, 0x03, 0x3e, 0x23,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x14, 0x12, 0x03, 0x3f, 0x02, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x04, 0x12, 0x03, 0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x14, 0x06, 0x12, 0x03, 0x3f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x14, 0x01, 0x12, 0x03, 0x3f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14,
    0x03, 0x12, 0x03, 0x3f, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x15, 0x12, 0x03,
    0x41, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x04, 0x12, 0x03, 0x41, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x06, 0x12, 0x03, 0x41, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x01, 0x12, 0x03, 0x41, 0x12, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x15, 0x03, 0x12, 0x03, 0x41, 0x23, 0x25,
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
