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
pub struct Link {
    // message fields
    id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    self_collide: ::std::option::Option<bool>,
    gravity: ::std::option::Option<bool>,
    kinematic: ::std::option::Option<bool>,
    enabled: ::std::option::Option<bool>,
    inertial: ::protobuf::SingularPtrField<super::inertial::Inertial>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    visual: ::protobuf::RepeatedField<super::visual::Visual>,
    collision: ::protobuf::RepeatedField<super::collision::Collision>,
    sensor: ::protobuf::RepeatedField<super::sensor::Sensor>,
    projector: ::protobuf::RepeatedField<super::projector::Projector>,
    canonical: ::std::option::Option<bool>,
    battery: ::protobuf::RepeatedField<super::battery::Battery>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Link {}

impl Link {
    pub fn new() -> Link {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Link {
        static mut instance: ::protobuf::lazy::Lazy<Link> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Link,
        };
        unsafe {
            instance.get(|| {
                Link {
                    id: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    self_collide: ::std::option::Option::None,
                    gravity: ::std::option::Option::None,
                    kinematic: ::std::option::Option::None,
                    enabled: ::std::option::Option::None,
                    inertial: ::protobuf::SingularPtrField::none(),
                    pose: ::protobuf::SingularPtrField::none(),
                    visual: ::protobuf::RepeatedField::new(),
                    collision: ::protobuf::RepeatedField::new(),
                    sensor: ::protobuf::RepeatedField::new(),
                    projector: ::protobuf::RepeatedField::new(),
                    canonical: ::std::option::Option::None,
                    battery: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 id = 1;

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

    // required string name = 2;

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

    // optional bool self_collide = 3;

    pub fn clear_self_collide(&mut self) {
        self.self_collide = ::std::option::Option::None;
    }

    pub fn has_self_collide(&self) -> bool {
        self.self_collide.is_some()
    }

    // Param is passed by value, moved
    pub fn set_self_collide(&mut self, v: bool) {
        self.self_collide = ::std::option::Option::Some(v);
    }

    pub fn get_self_collide(&self) -> bool {
        self.self_collide.unwrap_or(false)
    }

    // optional bool gravity = 4;

    pub fn clear_gravity(&mut self) {
        self.gravity = ::std::option::Option::None;
    }

    pub fn has_gravity(&self) -> bool {
        self.gravity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gravity(&mut self, v: bool) {
        self.gravity = ::std::option::Option::Some(v);
    }

    pub fn get_gravity(&self) -> bool {
        self.gravity.unwrap_or(false)
    }

    // optional bool kinematic = 5;

    pub fn clear_kinematic(&mut self) {
        self.kinematic = ::std::option::Option::None;
    }

    pub fn has_kinematic(&self) -> bool {
        self.kinematic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kinematic(&mut self, v: bool) {
        self.kinematic = ::std::option::Option::Some(v);
    }

    pub fn get_kinematic(&self) -> bool {
        self.kinematic.unwrap_or(false)
    }

    // optional bool enabled = 6;

    pub fn clear_enabled(&mut self) {
        self.enabled = ::std::option::Option::None;
    }

    pub fn has_enabled(&self) -> bool {
        self.enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = ::std::option::Option::Some(v);
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.unwrap_or(false)
    }

    // optional .gazebo.msgs.Inertial inertial = 7;

    pub fn clear_inertial(&mut self) {
        self.inertial.clear();
    }

    pub fn has_inertial(&self) -> bool {
        self.inertial.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inertial(&mut self, v: super::inertial::Inertial) {
        self.inertial = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inertial(&mut self) -> &mut super::inertial::Inertial {
        if self.inertial.is_none() {
            self.inertial.set_default();
        };
        self.inertial.as_mut().unwrap()
    }

    // Take field
    pub fn take_inertial(&mut self) -> super::inertial::Inertial {
        self.inertial.take().unwrap_or_else(|| super::inertial::Inertial::new())
    }

    pub fn get_inertial(&self) -> &super::inertial::Inertial {
        self.inertial.as_ref().unwrap_or_else(|| super::inertial::Inertial::default_instance())
    }

    // optional .gazebo.msgs.Pose pose = 8;

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

    // repeated .gazebo.msgs.Visual visual = 9;

    pub fn clear_visual(&mut self) {
        self.visual.clear();
    }

    // Param is passed by value, moved
    pub fn set_visual(&mut self, v: ::protobuf::RepeatedField<super::visual::Visual>) {
        self.visual = v;
    }

    // Mutable pointer to the field.
    pub fn mut_visual(&mut self) -> &mut ::protobuf::RepeatedField<super::visual::Visual> {
        &mut self.visual
    }

    // Take field
    pub fn take_visual(&mut self) -> ::protobuf::RepeatedField<super::visual::Visual> {
        ::std::mem::replace(&mut self.visual, ::protobuf::RepeatedField::new())
    }

    pub fn get_visual(&self) -> &[super::visual::Visual] {
        &self.visual
    }

    // repeated .gazebo.msgs.Collision collision = 10;

    pub fn clear_collision(&mut self) {
        self.collision.clear();
    }

    // Param is passed by value, moved
    pub fn set_collision(&mut self, v: ::protobuf::RepeatedField<super::collision::Collision>) {
        self.collision = v;
    }

    // Mutable pointer to the field.
    pub fn mut_collision(&mut self) -> &mut ::protobuf::RepeatedField<super::collision::Collision> {
        &mut self.collision
    }

    // Take field
    pub fn take_collision(&mut self) -> ::protobuf::RepeatedField<super::collision::Collision> {
        ::std::mem::replace(&mut self.collision, ::protobuf::RepeatedField::new())
    }

    pub fn get_collision(&self) -> &[super::collision::Collision] {
        &self.collision
    }

    // repeated .gazebo.msgs.Sensor sensor = 11;

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

    // repeated .gazebo.msgs.Projector projector = 12;

    pub fn clear_projector(&mut self) {
        self.projector.clear();
    }

    // Param is passed by value, moved
    pub fn set_projector(&mut self, v: ::protobuf::RepeatedField<super::projector::Projector>) {
        self.projector = v;
    }

    // Mutable pointer to the field.
    pub fn mut_projector(&mut self) -> &mut ::protobuf::RepeatedField<super::projector::Projector> {
        &mut self.projector
    }

    // Take field
    pub fn take_projector(&mut self) -> ::protobuf::RepeatedField<super::projector::Projector> {
        ::std::mem::replace(&mut self.projector, ::protobuf::RepeatedField::new())
    }

    pub fn get_projector(&self) -> &[super::projector::Projector] {
        &self.projector
    }

    // optional bool canonical = 13;

    pub fn clear_canonical(&mut self) {
        self.canonical = ::std::option::Option::None;
    }

    pub fn has_canonical(&self) -> bool {
        self.canonical.is_some()
    }

    // Param is passed by value, moved
    pub fn set_canonical(&mut self, v: bool) {
        self.canonical = ::std::option::Option::Some(v);
    }

    pub fn get_canonical(&self) -> bool {
        self.canonical.unwrap_or(false)
    }

    // repeated .gazebo.msgs.Battery battery = 14;

    pub fn clear_battery(&mut self) {
        self.battery.clear();
    }

    // Param is passed by value, moved
    pub fn set_battery(&mut self, v: ::protobuf::RepeatedField<super::battery::Battery>) {
        self.battery = v;
    }

    // Mutable pointer to the field.
    pub fn mut_battery(&mut self) -> &mut ::protobuf::RepeatedField<super::battery::Battery> {
        &mut self.battery
    }

    // Take field
    pub fn take_battery(&mut self) -> ::protobuf::RepeatedField<super::battery::Battery> {
        ::std::mem::replace(&mut self.battery, ::protobuf::RepeatedField::new())
    }

    pub fn get_battery(&self) -> &[super::battery::Battery] {
        &self.battery
    }
}

impl ::protobuf::Message for Link {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.self_collide = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.gravity = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.kinematic = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.enabled = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inertial));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.visual));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.collision));
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sensor));
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.projector));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.canonical = ::std::option::Option::Some(tmp);
                },
                14 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.battery));
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
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.self_collide.is_some() {
            my_size += 2;
        };
        if self.gravity.is_some() {
            my_size += 2;
        };
        if self.kinematic.is_some() {
            my_size += 2;
        };
        if self.enabled.is_some() {
            my_size += 2;
        };
        for value in &self.inertial {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.visual {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.collision {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sensor {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.projector {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.canonical.is_some() {
            my_size += 2;
        };
        for value in &self.battery {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.self_collide {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.gravity {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.kinematic {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.enabled {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.inertial.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.visual {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.collision {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.sensor {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.projector {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.canonical {
            try!(os.write_bool(13, v));
        };
        for v in &self.battery {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Link>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Link {
    fn new() -> Link {
        Link::new()
    }

    fn descriptor_static(_: ::std::option::Option<Link>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    Link::has_id,
                    Link::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Link::has_name,
                    Link::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "self_collide",
                    Link::has_self_collide,
                    Link::get_self_collide,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "gravity",
                    Link::has_gravity,
                    Link::get_gravity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "kinematic",
                    Link::has_kinematic,
                    Link::get_kinematic,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "enabled",
                    Link::has_enabled,
                    Link::get_enabled,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "inertial",
                    Link::has_inertial,
                    Link::get_inertial,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Link::has_pose,
                    Link::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "visual",
                    Link::get_visual,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "collision",
                    Link::get_collision,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "sensor",
                    Link::get_sensor,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "projector",
                    Link::get_projector,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "canonical",
                    Link::has_canonical,
                    Link::get_canonical,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "battery",
                    Link::get_battery,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Link>(
                    "Link",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Link {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_self_collide();
        self.clear_gravity();
        self.clear_kinematic();
        self.clear_enabled();
        self.clear_inertial();
        self.clear_pose();
        self.clear_visual();
        self.clear_collision();
        self.clear_sensor();
        self.clear_projector();
        self.clear_canonical();
        self.clear_battery();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Link {
    fn eq(&self, other: &Link) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.self_collide == other.self_collide &&
        self.gravity == other.gravity &&
        self.kinematic == other.kinematic &&
        self.enabled == other.enabled &&
        self.inertial == other.inertial &&
        self.pose == other.pose &&
        self.visual == other.visual &&
        self.collision == other.collision &&
        self.sensor == other.sensor &&
        self.projector == other.projector &&
        self.canonical == other.canonical &&
        self.battery == other.battery &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Link {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0a, 0x6c, 0x69, 0x6e, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0e, 0x69, 0x6e, 0x65, 0x72, 0x74,
    0x69, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f, 0x63, 0x6f, 0x6c, 0x6c, 0x69,
    0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x76, 0x69, 0x73, 0x75,
    0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x0d, 0x62, 0x61, 0x74, 0x74, 0x65, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x8f, 0x03, 0x0a, 0x04, 0x4c, 0x69, 0x6e, 0x6b, 0x12, 0x0a, 0x0a, 0x02, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x65, 0x6c, 0x66, 0x5f, 0x63, 0x6f,
    0x6c, 0x6c, 0x69, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0f, 0x0a, 0x07, 0x67,
    0x72, 0x61, 0x76, 0x69, 0x74, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x12, 0x11, 0x0a, 0x09,
    0x6b, 0x69, 0x6e, 0x65, 0x6d, 0x61, 0x74, 0x69, 0x63, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x12,
    0x0f, 0x0a, 0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x27, 0x0a, 0x08, 0x69, 0x6e, 0x65, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x49, 0x6e, 0x65, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f, 0x73,
    0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f,
    0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x23, 0x0a, 0x06, 0x76, 0x69,
    0x73, 0x75, 0x61, 0x6c, 0x18, 0x09, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x69, 0x73, 0x75, 0x61, 0x6c, 0x12,
    0x29, 0x0a, 0x09, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x16, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x43, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x65,
    0x6e, 0x73, 0x6f, 0x72, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x12,
    0x29, 0x0a, 0x09, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x18, 0x0c, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x16, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x61,
    0x6e, 0x6f, 0x6e, 0x69, 0x63, 0x61, 0x6c, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x12, 0x25, 0x0a,
    0x07, 0x62, 0x61, 0x74, 0x74, 0x65, 0x72, 0x79, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14,
    0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x42, 0x61, 0x74,
    0x74, 0x65, 0x72, 0x79, 0x4a, 0x88, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x21, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x07, 0x07, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x18, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09, 0x07, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x0a, 0x07, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0b, 0x07, 0x18,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x0d, 0x07, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0f, 0x00,
    0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x0c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x11, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x11, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x13, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x13, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x10, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x22, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x14, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x14, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x14, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x14,
    0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x15, 0x02, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x15, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x15, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x16, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x16,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x16, 0x10, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x16, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x17, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06,
    0x12, 0x03, 0x17, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x17, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x17, 0x22,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x18, 0x02, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x18, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x18, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x18, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03,
    0x19, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x19, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x19, 0x12, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x19, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x09, 0x12, 0x03, 0x1a, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09,
    0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12,
    0x03, 0x1a, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1a,
    0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1a, 0x22, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x1b, 0x02, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03,
    0x12, 0x03, 0x1b, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x1c,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x1c, 0x0b, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x1c, 0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x1c, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0c, 0x12, 0x03, 0x1d, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04,
    0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03,
    0x1d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x1d, 0x10,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x1d, 0x22, 0x24, 0x0a,
    0x56, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x20, 0x02, 0x25, 0x1a, 0x49, 0x2f, 0x20,
    0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x41, 0x6e, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x20, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x61, 0x74,
    0x74, 0x65, 0x72, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x72, 0x65, 0x20,
    0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x6c, 0x69, 0x6e, 0x6b, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04,
    0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x06, 0x12, 0x03,
    0x20, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x20, 0x13,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x20, 0x22, 0x24,
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
