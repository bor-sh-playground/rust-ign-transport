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
pub struct Sensor {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    id: ::std::option::Option<u32>,
    parent: ::protobuf::SingularField<::std::string::String>,
    parent_id: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::string::String>,
    always_on: ::std::option::Option<bool>,
    update_rate: ::std::option::Option<f64>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    camera: ::protobuf::SingularPtrField<super::camerasensor::CameraSensor>,
    ray: ::protobuf::SingularPtrField<super::raysensor::RaySensor>,
    contact: ::protobuf::SingularPtrField<super::contactsensor::ContactSensor>,
    visualize: ::std::option::Option<bool>,
    topic: ::protobuf::SingularField<::std::string::String>,
    logical_camera: ::protobuf::SingularPtrField<super::logical_camera_sensor::LogicalCameraSensor>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Sensor {}

impl Sensor {
    pub fn new() -> Sensor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Sensor {
        static mut instance: ::protobuf::lazy::Lazy<Sensor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Sensor,
        };
        unsafe {
            instance.get(|| {
                Sensor {
                    name: ::protobuf::SingularField::none(),
                    id: ::std::option::Option::None,
                    parent: ::protobuf::SingularField::none(),
                    parent_id: ::std::option::Option::None,
                    field_type: ::protobuf::SingularField::none(),
                    always_on: ::std::option::Option::None,
                    update_rate: ::std::option::Option::None,
                    pose: ::protobuf::SingularPtrField::none(),
                    camera: ::protobuf::SingularPtrField::none(),
                    ray: ::protobuf::SingularPtrField::none(),
                    contact: ::protobuf::SingularPtrField::none(),
                    visualize: ::std::option::Option::None,
                    topic: ::protobuf::SingularField::none(),
                    logical_camera: ::protobuf::SingularPtrField::none(),
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

    // required string parent = 3;

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

    // optional uint32 parent_id = 4;

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

    // required string type = 5;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool always_on = 6;

    pub fn clear_always_on(&mut self) {
        self.always_on = ::std::option::Option::None;
    }

    pub fn has_always_on(&self) -> bool {
        self.always_on.is_some()
    }

    // Param is passed by value, moved
    pub fn set_always_on(&mut self, v: bool) {
        self.always_on = ::std::option::Option::Some(v);
    }

    pub fn get_always_on(&self) -> bool {
        self.always_on.unwrap_or(false)
    }

    // optional double update_rate = 7;

    pub fn clear_update_rate(&mut self) {
        self.update_rate = ::std::option::Option::None;
    }

    pub fn has_update_rate(&self) -> bool {
        self.update_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_rate(&mut self, v: f64) {
        self.update_rate = ::std::option::Option::Some(v);
    }

    pub fn get_update_rate(&self) -> f64 {
        self.update_rate.unwrap_or(0.)
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

    // optional .gazebo.msgs.CameraSensor camera = 9;

    pub fn clear_camera(&mut self) {
        self.camera.clear();
    }

    pub fn has_camera(&self) -> bool {
        self.camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera(&mut self, v: super::camerasensor::CameraSensor) {
        self.camera = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_camera(&mut self) -> &mut super::camerasensor::CameraSensor {
        if self.camera.is_none() {
            self.camera.set_default();
        };
        self.camera.as_mut().unwrap()
    }

    // Take field
    pub fn take_camera(&mut self) -> super::camerasensor::CameraSensor {
        self.camera.take().unwrap_or_else(|| super::camerasensor::CameraSensor::new())
    }

    pub fn get_camera(&self) -> &super::camerasensor::CameraSensor {
        self.camera.as_ref().unwrap_or_else(|| super::camerasensor::CameraSensor::default_instance())
    }

    // optional .gazebo.msgs.RaySensor ray = 10;

    pub fn clear_ray(&mut self) {
        self.ray.clear();
    }

    pub fn has_ray(&self) -> bool {
        self.ray.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ray(&mut self, v: super::raysensor::RaySensor) {
        self.ray = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ray(&mut self) -> &mut super::raysensor::RaySensor {
        if self.ray.is_none() {
            self.ray.set_default();
        };
        self.ray.as_mut().unwrap()
    }

    // Take field
    pub fn take_ray(&mut self) -> super::raysensor::RaySensor {
        self.ray.take().unwrap_or_else(|| super::raysensor::RaySensor::new())
    }

    pub fn get_ray(&self) -> &super::raysensor::RaySensor {
        self.ray.as_ref().unwrap_or_else(|| super::raysensor::RaySensor::default_instance())
    }

    // optional .gazebo.msgs.ContactSensor contact = 11;

    pub fn clear_contact(&mut self) {
        self.contact.clear();
    }

    pub fn has_contact(&self) -> bool {
        self.contact.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contact(&mut self, v: super::contactsensor::ContactSensor) {
        self.contact = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contact(&mut self) -> &mut super::contactsensor::ContactSensor {
        if self.contact.is_none() {
            self.contact.set_default();
        };
        self.contact.as_mut().unwrap()
    }

    // Take field
    pub fn take_contact(&mut self) -> super::contactsensor::ContactSensor {
        self.contact.take().unwrap_or_else(|| super::contactsensor::ContactSensor::new())
    }

    pub fn get_contact(&self) -> &super::contactsensor::ContactSensor {
        self.contact.as_ref().unwrap_or_else(|| super::contactsensor::ContactSensor::default_instance())
    }

    // optional bool visualize = 12;

    pub fn clear_visualize(&mut self) {
        self.visualize = ::std::option::Option::None;
    }

    pub fn has_visualize(&self) -> bool {
        self.visualize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visualize(&mut self, v: bool) {
        self.visualize = ::std::option::Option::Some(v);
    }

    pub fn get_visualize(&self) -> bool {
        self.visualize.unwrap_or(false)
    }

    // optional string topic = 13;

    pub fn clear_topic(&mut self) {
        self.topic.clear();
    }

    pub fn has_topic(&self) -> bool {
        self.topic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_topic(&mut self, v: ::std::string::String) {
        self.topic = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_topic(&mut self) -> &mut ::std::string::String {
        if self.topic.is_none() {
            self.topic.set_default();
        };
        self.topic.as_mut().unwrap()
    }

    // Take field
    pub fn take_topic(&mut self) -> ::std::string::String {
        self.topic.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_topic(&self) -> &str {
        match self.topic.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .gazebo.msgs.LogicalCameraSensor logical_camera = 14;

    pub fn clear_logical_camera(&mut self) {
        self.logical_camera.clear();
    }

    pub fn has_logical_camera(&self) -> bool {
        self.logical_camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logical_camera(&mut self, v: super::logical_camera_sensor::LogicalCameraSensor) {
        self.logical_camera = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_logical_camera(&mut self) -> &mut super::logical_camera_sensor::LogicalCameraSensor {
        if self.logical_camera.is_none() {
            self.logical_camera.set_default();
        };
        self.logical_camera.as_mut().unwrap()
    }

    // Take field
    pub fn take_logical_camera(&mut self) -> super::logical_camera_sensor::LogicalCameraSensor {
        self.logical_camera.take().unwrap_or_else(|| super::logical_camera_sensor::LogicalCameraSensor::new())
    }

    pub fn get_logical_camera(&self) -> &super::logical_camera_sensor::LogicalCameraSensor {
        self.logical_camera.as_ref().unwrap_or_else(|| super::logical_camera_sensor::LogicalCameraSensor::default_instance())
    }
}

impl ::protobuf::Message for Sensor {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.parent.is_none() {
            return false;
        };
        if self.field_type.is_none() {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.parent));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.parent_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.always_on = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.update_rate = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.camera));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ray));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contact));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.visualize = ::std::option::Option::Some(tmp);
                },
                13 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.topic));
                },
                14 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.logical_camera));
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
        for value in &self.parent {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.parent_id {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if self.always_on.is_some() {
            my_size += 2;
        };
        if self.update_rate.is_some() {
            my_size += 9;
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.camera {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.ray {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.contact {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.visualize.is_some() {
            my_size += 2;
        };
        for value in &self.topic {
            my_size += ::protobuf::rt::string_size(13, &value);
        };
        for value in &self.logical_camera {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.parent.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.parent_id {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.always_on {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.update_rate {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.camera.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ray.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.contact.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.visualize {
            try!(os.write_bool(12, v));
        };
        if let Some(v) = self.topic.as_ref() {
            try!(os.write_string(13, &v));
        };
        if let Some(v) = self.logical_camera.as_ref() {
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
        ::std::any::TypeId::of::<Sensor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Sensor {
    fn new() -> Sensor {
        Sensor::new()
    }

    fn descriptor_static(_: ::std::option::Option<Sensor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Sensor::has_name,
                    Sensor::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    Sensor::has_id,
                    Sensor::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "parent",
                    Sensor::has_parent,
                    Sensor::get_parent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "parent_id",
                    Sensor::has_parent_id,
                    Sensor::get_parent_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "type",
                    Sensor::has_field_type,
                    Sensor::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "always_on",
                    Sensor::has_always_on,
                    Sensor::get_always_on,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "update_rate",
                    Sensor::has_update_rate,
                    Sensor::get_update_rate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Sensor::has_pose,
                    Sensor::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "camera",
                    Sensor::has_camera,
                    Sensor::get_camera,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ray",
                    Sensor::has_ray,
                    Sensor::get_ray,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "contact",
                    Sensor::has_contact,
                    Sensor::get_contact,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "visualize",
                    Sensor::has_visualize,
                    Sensor::get_visualize,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "topic",
                    Sensor::has_topic,
                    Sensor::get_topic,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "logical_camera",
                    Sensor::has_logical_camera,
                    Sensor::get_logical_camera,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Sensor>(
                    "Sensor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Sensor {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_parent();
        self.clear_parent_id();
        self.clear_field_type();
        self.clear_always_on();
        self.clear_update_rate();
        self.clear_pose();
        self.clear_camera();
        self.clear_ray();
        self.clear_contact();
        self.clear_visualize();
        self.clear_topic();
        self.clear_logical_camera();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Sensor {
    fn eq(&self, other: &Sensor) -> bool {
        self.name == other.name &&
        self.id == other.id &&
        self.parent == other.parent &&
        self.parent_id == other.parent_id &&
        self.field_type == other.field_type &&
        self.always_on == other.always_on &&
        self.update_rate == other.update_rate &&
        self.pose == other.pose &&
        self.camera == other.camera &&
        self.ray == other.ray &&
        self.contact == other.contact &&
        self.visualize == other.visualize &&
        self.topic == other.topic &&
        self.logical_camera == other.logical_camera &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Sensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x70, 0x6f, 0x73,
    0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x12, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x73,
    0x65, 0x6e, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f, 0x72, 0x61, 0x79,
    0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x63, 0x74, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1b, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72,
    0x61, 0x5f, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xf5,
    0x02, 0x0a, 0x06, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x05,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x5f, 0x6f,
    0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1f, 0x0a, 0x04,
    0x70, 0x6f, 0x73, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x29, 0x0a,
    0x06, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x43, 0x61, 0x6d, 0x65,
    0x72, 0x61, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x12, 0x23, 0x0a, 0x03, 0x72, 0x61, 0x79, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x52, 0x61, 0x79, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x12, 0x2b, 0x0a,
    0x07, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
    0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x43, 0x6f, 0x6e,
    0x74, 0x61, 0x63, 0x74, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x12, 0x11, 0x0a, 0x09, 0x76, 0x69,
    0x73, 0x75, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0d, 0x0a,
    0x05, 0x74, 0x6f, 0x70, 0x69, 0x63, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x09, 0x12, 0x38, 0x0a, 0x0e,
    0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x18, 0x0e,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x2e, 0x4c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x4a, 0xd9, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1e,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x07,
    0x1b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x08, 0x07, 0x18, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x09, 0x07, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0a,
    0x07, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x1e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x24, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0f, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0f, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x02,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x11, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x11,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x12, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x11, 0x24, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x12, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x12, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x12, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x13, 0x02, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x13, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x13, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06,
    0x12, 0x03, 0x14, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x14, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x14, 0x12, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x14, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x15, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x06, 0x12, 0x03, 0x15, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x15, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x15,
    0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x16, 0x02, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x16, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x16, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x16, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12,
    0x03, 0x17, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x17,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12, 0x03, 0x17, 0x0b, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x17, 0x15, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x17, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x18, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x06,
    0x12, 0x03, 0x18, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x18, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x18, 0x24,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x19, 0x02, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0b, 0x01, 0x12, 0x03, 0x19, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b,
    0x03, 0x12, 0x03, 0x19, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03,
    0x1a, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x1a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x1a, 0x24, 0x26, 0x0a, 0x3d, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x0d, 0x12, 0x03, 0x1d, 0x02, 0x33, 0x1a, 0x30, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69,
    0x65, 0x66, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x20, 0x63, 0x61, 0x6d, 0x65,
    0x72, 0x61, 0x20, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0d, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d,
    0x06, 0x12, 0x03, 0x1d, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12,
    0x03, 0x1d, 0x1f, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x1d,
    0x30, 0x32,
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
