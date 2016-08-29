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
pub struct Scene {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    ambient: ::protobuf::SingularPtrField<super::color::Color>,
    background: ::protobuf::SingularPtrField<super::color::Color>,
    sky: ::protobuf::SingularPtrField<super::sky::Sky>,
    shadows: ::std::option::Option<bool>,
    fog: ::protobuf::SingularPtrField<super::fog::Fog>,
    grid: ::std::option::Option<bool>,
    model: ::protobuf::RepeatedField<super::model::Model>,
    light: ::protobuf::RepeatedField<super::light::Light>,
    joint: ::protobuf::RepeatedField<super::joint::Joint>,
    origin_visual: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Scene {}

impl Scene {
    pub fn new() -> Scene {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Scene {
        static mut instance: ::protobuf::lazy::Lazy<Scene> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Scene,
        };
        unsafe {
            instance.get(|| {
                Scene {
                    name: ::protobuf::SingularField::none(),
                    ambient: ::protobuf::SingularPtrField::none(),
                    background: ::protobuf::SingularPtrField::none(),
                    sky: ::protobuf::SingularPtrField::none(),
                    shadows: ::std::option::Option::None,
                    fog: ::protobuf::SingularPtrField::none(),
                    grid: ::std::option::Option::None,
                    model: ::protobuf::RepeatedField::new(),
                    light: ::protobuf::RepeatedField::new(),
                    joint: ::protobuf::RepeatedField::new(),
                    origin_visual: ::std::option::Option::None,
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

    // optional .gazebo.msgs.Color ambient = 2;

    pub fn clear_ambient(&mut self) {
        self.ambient.clear();
    }

    pub fn has_ambient(&self) -> bool {
        self.ambient.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ambient(&mut self, v: super::color::Color) {
        self.ambient = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ambient(&mut self) -> &mut super::color::Color {
        if self.ambient.is_none() {
            self.ambient.set_default();
        };
        self.ambient.as_mut().unwrap()
    }

    // Take field
    pub fn take_ambient(&mut self) -> super::color::Color {
        self.ambient.take().unwrap_or_else(|| super::color::Color::new())
    }

    pub fn get_ambient(&self) -> &super::color::Color {
        self.ambient.as_ref().unwrap_or_else(|| super::color::Color::default_instance())
    }

    // optional .gazebo.msgs.Color background = 3;

    pub fn clear_background(&mut self) {
        self.background.clear();
    }

    pub fn has_background(&self) -> bool {
        self.background.is_some()
    }

    // Param is passed by value, moved
    pub fn set_background(&mut self, v: super::color::Color) {
        self.background = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_background(&mut self) -> &mut super::color::Color {
        if self.background.is_none() {
            self.background.set_default();
        };
        self.background.as_mut().unwrap()
    }

    // Take field
    pub fn take_background(&mut self) -> super::color::Color {
        self.background.take().unwrap_or_else(|| super::color::Color::new())
    }

    pub fn get_background(&self) -> &super::color::Color {
        self.background.as_ref().unwrap_or_else(|| super::color::Color::default_instance())
    }

    // optional .gazebo.msgs.Sky sky = 4;

    pub fn clear_sky(&mut self) {
        self.sky.clear();
    }

    pub fn has_sky(&self) -> bool {
        self.sky.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sky(&mut self, v: super::sky::Sky) {
        self.sky = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sky(&mut self) -> &mut super::sky::Sky {
        if self.sky.is_none() {
            self.sky.set_default();
        };
        self.sky.as_mut().unwrap()
    }

    // Take field
    pub fn take_sky(&mut self) -> super::sky::Sky {
        self.sky.take().unwrap_or_else(|| super::sky::Sky::new())
    }

    pub fn get_sky(&self) -> &super::sky::Sky {
        self.sky.as_ref().unwrap_or_else(|| super::sky::Sky::default_instance())
    }

    // optional bool shadows = 5;

    pub fn clear_shadows(&mut self) {
        self.shadows = ::std::option::Option::None;
    }

    pub fn has_shadows(&self) -> bool {
        self.shadows.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shadows(&mut self, v: bool) {
        self.shadows = ::std::option::Option::Some(v);
    }

    pub fn get_shadows(&self) -> bool {
        self.shadows.unwrap_or(true)
    }

    // optional .gazebo.msgs.Fog fog = 6;

    pub fn clear_fog(&mut self) {
        self.fog.clear();
    }

    pub fn has_fog(&self) -> bool {
        self.fog.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fog(&mut self, v: super::fog::Fog) {
        self.fog = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fog(&mut self) -> &mut super::fog::Fog {
        if self.fog.is_none() {
            self.fog.set_default();
        };
        self.fog.as_mut().unwrap()
    }

    // Take field
    pub fn take_fog(&mut self) -> super::fog::Fog {
        self.fog.take().unwrap_or_else(|| super::fog::Fog::new())
    }

    pub fn get_fog(&self) -> &super::fog::Fog {
        self.fog.as_ref().unwrap_or_else(|| super::fog::Fog::default_instance())
    }

    // optional bool grid = 7;

    pub fn clear_grid(&mut self) {
        self.grid = ::std::option::Option::None;
    }

    pub fn has_grid(&self) -> bool {
        self.grid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_grid(&mut self, v: bool) {
        self.grid = ::std::option::Option::Some(v);
    }

    pub fn get_grid(&self) -> bool {
        self.grid.unwrap_or(false)
    }

    // repeated .gazebo.msgs.Model model = 8;

    pub fn clear_model(&mut self) {
        self.model.clear();
    }

    // Param is passed by value, moved
    pub fn set_model(&mut self, v: ::protobuf::RepeatedField<super::model::Model>) {
        self.model = v;
    }

    // Mutable pointer to the field.
    pub fn mut_model(&mut self) -> &mut ::protobuf::RepeatedField<super::model::Model> {
        &mut self.model
    }

    // Take field
    pub fn take_model(&mut self) -> ::protobuf::RepeatedField<super::model::Model> {
        ::std::mem::replace(&mut self.model, ::protobuf::RepeatedField::new())
    }

    pub fn get_model(&self) -> &[super::model::Model] {
        &self.model
    }

    // repeated .gazebo.msgs.Light light = 9;

    pub fn clear_light(&mut self) {
        self.light.clear();
    }

    // Param is passed by value, moved
    pub fn set_light(&mut self, v: ::protobuf::RepeatedField<super::light::Light>) {
        self.light = v;
    }

    // Mutable pointer to the field.
    pub fn mut_light(&mut self) -> &mut ::protobuf::RepeatedField<super::light::Light> {
        &mut self.light
    }

    // Take field
    pub fn take_light(&mut self) -> ::protobuf::RepeatedField<super::light::Light> {
        ::std::mem::replace(&mut self.light, ::protobuf::RepeatedField::new())
    }

    pub fn get_light(&self) -> &[super::light::Light] {
        &self.light
    }

    // repeated .gazebo.msgs.Joint joint = 10;

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

    // optional bool origin_visual = 11;

    pub fn clear_origin_visual(&mut self) {
        self.origin_visual = ::std::option::Option::None;
    }

    pub fn has_origin_visual(&self) -> bool {
        self.origin_visual.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_visual(&mut self, v: bool) {
        self.origin_visual = ::std::option::Option::Some(v);
    }

    pub fn get_origin_visual(&self) -> bool {
        self.origin_visual.unwrap_or(false)
    }
}

impl ::protobuf::Message for Scene {
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ambient));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.background));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sky));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.shadows = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fog));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.grid = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.model));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.light));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.joint));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.origin_visual = ::std::option::Option::Some(tmp);
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
        for value in &self.ambient {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.background {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sky {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.shadows.is_some() {
            my_size += 2;
        };
        for value in &self.fog {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.grid.is_some() {
            my_size += 2;
        };
        for value in &self.model {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.light {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.joint {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.origin_visual.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.ambient.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.background.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.sky.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.shadows {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.fog.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.grid {
            try!(os.write_bool(7, v));
        };
        for v in &self.model {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.light {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.joint {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.origin_visual {
            try!(os.write_bool(11, v));
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
        ::std::any::TypeId::of::<Scene>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Scene {
    fn new() -> Scene {
        Scene::new()
    }

    fn descriptor_static(_: ::std::option::Option<Scene>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Scene::has_name,
                    Scene::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ambient",
                    Scene::has_ambient,
                    Scene::get_ambient,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "background",
                    Scene::has_background,
                    Scene::get_background,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "sky",
                    Scene::has_sky,
                    Scene::get_sky,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "shadows",
                    Scene::has_shadows,
                    Scene::get_shadows,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fog",
                    Scene::has_fog,
                    Scene::get_fog,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "grid",
                    Scene::has_grid,
                    Scene::get_grid,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "model",
                    Scene::get_model,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "light",
                    Scene::get_light,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "joint",
                    Scene::get_joint,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "origin_visual",
                    Scene::has_origin_visual,
                    Scene::get_origin_visual,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Scene>(
                    "Scene",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Scene {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_ambient();
        self.clear_background();
        self.clear_sky();
        self.clear_shadows();
        self.clear_fog();
        self.clear_grid();
        self.clear_model();
        self.clear_light();
        self.clear_joint();
        self.clear_origin_visual();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Scene {
    fn eq(&self, other: &Scene) -> bool {
        self.name == other.name &&
        self.ambient == other.ambient &&
        self.background == other.background &&
        self.sky == other.sky &&
        self.shadows == other.shadows &&
        self.fog == other.fog &&
        self.grid == other.grid &&
        self.model == other.model &&
        self.light == other.light &&
        self.joint == other.joint &&
        self.origin_visual == other.origin_visual &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Scene {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x73, 0x63, 0x65, 0x6e, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0b, 0x63, 0x6f, 0x6c, 0x6f,
    0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x09, 0x66, 0x6f, 0x67, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x09, 0x73, 0x6b, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0b, 0x6c,
    0x69, 0x67, 0x68, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0b, 0x6a, 0x6f, 0x69, 0x6e,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0b, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc5, 0x02, 0x0a, 0x05, 0x53, 0x63, 0x65, 0x6e, 0x65, 0x12, 0x0c,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x23, 0x0a, 0x07,
    0x61, 0x6d, 0x62, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x43, 0x6f, 0x6c, 0x6f,
    0x72, 0x12, 0x26, 0x0a, 0x0a, 0x62, 0x61, 0x63, 0x6b, 0x67, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x1d, 0x0a, 0x03, 0x73, 0x6b, 0x79,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e,
    0x6d, 0x73, 0x67, 0x73, 0x2e, 0x53, 0x6b, 0x79, 0x12, 0x15, 0x0a, 0x07, 0x73, 0x68, 0x61, 0x64,
    0x6f, 0x77, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x12,
    0x1d, 0x0a, 0x03, 0x66, 0x6f, 0x67, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x46, 0x6f, 0x67, 0x12, 0x0c,
    0x0a, 0x04, 0x67, 0x72, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x12, 0x21, 0x0a, 0x05,
    0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x12,
    0x21, 0x0a, 0x05, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x18, 0x09, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12,
    0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4c, 0x69, 0x67,
    0x68, 0x74, 0x12, 0x21, 0x0a, 0x05, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x0a, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x15, 0x0a, 0x0d, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f,
    0x76, 0x69, 0x73, 0x75, 0x61, 0x6c, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x4a, 0xac, 0x07, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x1e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08,
    0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x14, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09,
    0x07, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0a, 0x07, 0x14, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x0b, 0x07, 0x14, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0c, 0x07, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0e, 0x00, 0x1e, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x21, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x11, 0x02, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x11, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x11, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x12,
    0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x13, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x13, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x13, 0x0f,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x13, 0x21, 0x22, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x14, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x14, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x14, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x08, 0x12, 0x03, 0x14,
    0x23, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x07, 0x12, 0x03, 0x14, 0x2e, 0x32,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x15, 0x02, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x15, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x15, 0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x15, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x16,
    0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x16, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x16, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x18, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04,
    0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03,
    0x18, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x18, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x18, 0x21, 0x22, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x19, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x06, 0x12, 0x03, 0x19, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x19, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12,
    0x03, 0x19, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x1a, 0x02,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12, 0x03, 0x1a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1a, 0x21, 0x23, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0a, 0x12, 0x03, 0x1d, 0x02, 0x24, 0x1a, 0x2b, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66,
    0x20, 0x53, 0x68, 0x6f, 0x77, 0x2f, 0x68, 0x69, 0x64, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64,
    0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x6f,
    0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x1d, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x1d, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x1d, 0x10, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x1d, 0x21, 0x23,
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
