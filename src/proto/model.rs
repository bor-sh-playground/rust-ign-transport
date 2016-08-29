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
pub struct Model {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    id: ::std::option::Option<u32>,
    is_static: ::std::option::Option<bool>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    joint: ::protobuf::RepeatedField<super::joint::Joint>,
    link: ::protobuf::RepeatedField<super::link::Link>,
    deleted: ::std::option::Option<bool>,
    visual: ::protobuf::RepeatedField<super::visual::Visual>,
    scale: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    self_collide: ::std::option::Option<bool>,
    model: ::protobuf::RepeatedField<Model>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Model {}

impl Model {
    pub fn new() -> Model {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Model {
        static mut instance: ::protobuf::lazy::Lazy<Model> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Model,
        };
        unsafe {
            instance.get(|| {
                Model {
                    name: ::protobuf::SingularField::none(),
                    id: ::std::option::Option::None,
                    is_static: ::std::option::Option::None,
                    pose: ::protobuf::SingularPtrField::none(),
                    joint: ::protobuf::RepeatedField::new(),
                    link: ::protobuf::RepeatedField::new(),
                    deleted: ::std::option::Option::None,
                    visual: ::protobuf::RepeatedField::new(),
                    scale: ::protobuf::SingularPtrField::none(),
                    self_collide: ::std::option::Option::None,
                    model: ::protobuf::RepeatedField::new(),
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

    // optional bool is_static = 3;

    pub fn clear_is_static(&mut self) {
        self.is_static = ::std::option::Option::None;
    }

    pub fn has_is_static(&self) -> bool {
        self.is_static.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_static(&mut self, v: bool) {
        self.is_static = ::std::option::Option::Some(v);
    }

    pub fn get_is_static(&self) -> bool {
        self.is_static.unwrap_or(false)
    }

    // optional .gazebo.msgs.Pose pose = 4;

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

    // repeated .gazebo.msgs.Joint joint = 5;

    pub fn clear_joint(&mut self) {
        self.joint.clear();
    }

    // Param is passed by value, moved
    pub fn set_joint(&mut self, v: ::protobuf::RepeatedField<super::joint::Joint>) {
        self.joint = v;
    }

    // Mutable pointer to the field.
    pub fn mut_joint(&mut self) -> &mut ::protobuf::RepeatedField<super::joint::Joint> {
        &mut self.joint
    }

    // Take field
    pub fn take_joint(&mut self) -> ::protobuf::RepeatedField<super::joint::Joint> {
        ::std::mem::replace(&mut self.joint, ::protobuf::RepeatedField::new())
    }

    pub fn get_joint(&self) -> &[super::joint::Joint] {
        &self.joint
    }

    // repeated .gazebo.msgs.Link link = 6;

    pub fn clear_link(&mut self) {
        self.link.clear();
    }

    // Param is passed by value, moved
    pub fn set_link(&mut self, v: ::protobuf::RepeatedField<super::link::Link>) {
        self.link = v;
    }

    // Mutable pointer to the field.
    pub fn mut_link(&mut self) -> &mut ::protobuf::RepeatedField<super::link::Link> {
        &mut self.link
    }

    // Take field
    pub fn take_link(&mut self) -> ::protobuf::RepeatedField<super::link::Link> {
        ::std::mem::replace(&mut self.link, ::protobuf::RepeatedField::new())
    }

    pub fn get_link(&self) -> &[super::link::Link] {
        &self.link
    }

    // optional bool deleted = 7;

    pub fn clear_deleted(&mut self) {
        self.deleted = ::std::option::Option::None;
    }

    pub fn has_deleted(&self) -> bool {
        self.deleted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deleted(&mut self, v: bool) {
        self.deleted = ::std::option::Option::Some(v);
    }

    pub fn get_deleted(&self) -> bool {
        self.deleted.unwrap_or(false)
    }

    // repeated .gazebo.msgs.Visual visual = 8;

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

    // optional .gazebo.msgs.Vector3d scale = 9;

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

    // optional bool self_collide = 10;

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

    // repeated .gazebo.msgs.Model model = 11;

    pub fn clear_model(&mut self) {
        self.model.clear();
    }

    // Param is passed by value, moved
    pub fn set_model(&mut self, v: ::protobuf::RepeatedField<Model>) {
        self.model = v;
    }

    // Mutable pointer to the field.
    pub fn mut_model(&mut self) -> &mut ::protobuf::RepeatedField<Model> {
        &mut self.model
    }

    // Take field
    pub fn take_model(&mut self) -> ::protobuf::RepeatedField<Model> {
        ::std::mem::replace(&mut self.model, ::protobuf::RepeatedField::new())
    }

    pub fn get_model(&self) -> &[Model] {
        &self.model
    }
}

impl ::protobuf::Message for Model {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_static = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.joint));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.link));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.deleted = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.visual));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.scale));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.self_collide = ::std::option::Option::Some(tmp);
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.model));
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
        if self.is_static.is_some() {
            my_size += 2;
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.joint {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.link {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.deleted.is_some() {
            my_size += 2;
        };
        for value in &self.visual {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.scale {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.self_collide.is_some() {
            my_size += 2;
        };
        for value in &self.model {
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
        if let Some(v) = self.is_static {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.joint {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.link {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.deleted {
            try!(os.write_bool(7, v));
        };
        for v in &self.visual {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.scale.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.self_collide {
            try!(os.write_bool(10, v));
        };
        for v in &self.model {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Model>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Model {
    fn new() -> Model {
        Model::new()
    }

    fn descriptor_static(_: ::std::option::Option<Model>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Model::has_name,
                    Model::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    Model::has_id,
                    Model::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_static",
                    Model::has_is_static,
                    Model::get_is_static,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Model::has_pose,
                    Model::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "joint",
                    Model::get_joint,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "link",
                    Model::get_link,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "deleted",
                    Model::has_deleted,
                    Model::get_deleted,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "visual",
                    Model::get_visual,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scale",
                    Model::has_scale,
                    Model::get_scale,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "self_collide",
                    Model::has_self_collide,
                    Model::get_self_collide,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "model",
                    Model::get_model,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Model>(
                    "Model",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Model {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_is_static();
        self.clear_pose();
        self.clear_joint();
        self.clear_link();
        self.clear_deleted();
        self.clear_visual();
        self.clear_scale();
        self.clear_self_collide();
        self.clear_model();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Model {
    fn eq(&self, other: &Model) -> bool {
        self.name == other.name &&
        self.id == other.id &&
        self.is_static == other.is_static &&
        self.pose == other.pose &&
        self.joint == other.joint &&
        self.link == other.link &&
        self.deleted == other.deleted &&
        self.visual == other.visual &&
        self.scale == other.scale &&
        self.self_collide == other.self_collide &&
        self.model == other.model &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Model {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0b, 0x6a, 0x6f, 0x69, 0x6e,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x6c, 0x69, 0x6e, 0x6b, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x0c, 0x76, 0x69, 0x73, 0x75, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x76,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xae, 0x02,
    0x0a, 0x05, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x11, 0x0a, 0x09, 0x69, 0x73, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x21, 0x0a, 0x05, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x05,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x2e, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x1f, 0x0a, 0x04, 0x6c, 0x69, 0x6e, 0x6b,
    0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e,
    0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4c, 0x69, 0x6e, 0x6b, 0x12, 0x0f, 0x0a, 0x07, 0x64, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x12, 0x23, 0x0a, 0x06, 0x76, 0x69,
    0x73, 0x75, 0x61, 0x6c, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x69, 0x73, 0x75, 0x61, 0x6c, 0x12,
    0x24, 0x0a, 0x05, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15,
    0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x33, 0x64, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x65, 0x6c, 0x66, 0x5f, 0x63, 0x6f,
    0x6c, 0x6c, 0x69, 0x64, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x12, 0x21, 0x0a, 0x05, 0x6d,
    0x6f, 0x64, 0x65, 0x6c, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x4a, 0xfe,
    0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x14, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x09, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0a, 0x07, 0x15, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0b, 0x07, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x0d, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0d,
    0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x02, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0f, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x10, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x12, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x11, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x11, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x11, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x11, 0x21,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x12, 0x02, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x12, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x12, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03,
    0x13, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x13, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x13, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x13, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x13, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x05, 0x12, 0x03, 0x14, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x14, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x14,
    0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x14, 0x21, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x15, 0x02, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x15, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x15, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x16,
    0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x16, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x16, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x08, 0x12, 0x03, 0x17, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04,
    0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03,
    0x17, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x17, 0x14,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x17, 0x21, 0x22, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x18, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x09, 0x05, 0x12, 0x03, 0x18, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09,
    0x01, 0x12, 0x03, 0x18, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12,
    0x03, 0x18, 0x21, 0x23, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x1b, 0x02,
    0x24, 0x1a, 0x24, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x41, 0x6e, 0x20, 0x61,
    0x72, 0x72, 0x61, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x6e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x6d,
    0x6f, 0x64, 0x65, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04,
    0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x06, 0x12, 0x03,
    0x1b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x1b, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x1b, 0x21, 0x23,
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
