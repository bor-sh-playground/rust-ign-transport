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
pub struct Light {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<Light_LightType>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    diffuse: ::protobuf::SingularPtrField<super::color::Color>,
    specular: ::protobuf::SingularPtrField<super::color::Color>,
    attenuation_constant: ::std::option::Option<f32>,
    attenuation_linear: ::std::option::Option<f32>,
    attenuation_quadratic: ::std::option::Option<f32>,
    direction: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    range: ::std::option::Option<f32>,
    cast_shadows: ::std::option::Option<bool>,
    spot_inner_angle: ::std::option::Option<f32>,
    spot_outer_angle: ::std::option::Option<f32>,
    spot_falloff: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Light {}

impl Light {
    pub fn new() -> Light {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Light {
        static mut instance: ::protobuf::lazy::Lazy<Light> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Light,
        };
        unsafe {
            instance.get(|| {
                Light {
                    name: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    pose: ::protobuf::SingularPtrField::none(),
                    diffuse: ::protobuf::SingularPtrField::none(),
                    specular: ::protobuf::SingularPtrField::none(),
                    attenuation_constant: ::std::option::Option::None,
                    attenuation_linear: ::std::option::Option::None,
                    attenuation_quadratic: ::std::option::Option::None,
                    direction: ::protobuf::SingularPtrField::none(),
                    range: ::std::option::Option::None,
                    cast_shadows: ::std::option::Option::None,
                    spot_inner_angle: ::std::option::Option::None,
                    spot_outer_angle: ::std::option::Option::None,
                    spot_falloff: ::std::option::Option::None,
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

    // optional .gazebo.msgs.Light.LightType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Light_LightType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Light_LightType {
        self.field_type.unwrap_or(Light_LightType::POINT)
    }

    // optional .gazebo.msgs.Pose pose = 3;

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

    // optional .gazebo.msgs.Color diffuse = 4;

    pub fn clear_diffuse(&mut self) {
        self.diffuse.clear();
    }

    pub fn has_diffuse(&self) -> bool {
        self.diffuse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diffuse(&mut self, v: super::color::Color) {
        self.diffuse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diffuse(&mut self) -> &mut super::color::Color {
        if self.diffuse.is_none() {
            self.diffuse.set_default();
        };
        self.diffuse.as_mut().unwrap()
    }

    // Take field
    pub fn take_diffuse(&mut self) -> super::color::Color {
        self.diffuse.take().unwrap_or_else(|| super::color::Color::new())
    }

    pub fn get_diffuse(&self) -> &super::color::Color {
        self.diffuse.as_ref().unwrap_or_else(|| super::color::Color::default_instance())
    }

    // optional .gazebo.msgs.Color specular = 5;

    pub fn clear_specular(&mut self) {
        self.specular.clear();
    }

    pub fn has_specular(&self) -> bool {
        self.specular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_specular(&mut self, v: super::color::Color) {
        self.specular = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_specular(&mut self) -> &mut super::color::Color {
        if self.specular.is_none() {
            self.specular.set_default();
        };
        self.specular.as_mut().unwrap()
    }

    // Take field
    pub fn take_specular(&mut self) -> super::color::Color {
        self.specular.take().unwrap_or_else(|| super::color::Color::new())
    }

    pub fn get_specular(&self) -> &super::color::Color {
        self.specular.as_ref().unwrap_or_else(|| super::color::Color::default_instance())
    }

    // optional float attenuation_constant = 6;

    pub fn clear_attenuation_constant(&mut self) {
        self.attenuation_constant = ::std::option::Option::None;
    }

    pub fn has_attenuation_constant(&self) -> bool {
        self.attenuation_constant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attenuation_constant(&mut self, v: f32) {
        self.attenuation_constant = ::std::option::Option::Some(v);
    }

    pub fn get_attenuation_constant(&self) -> f32 {
        self.attenuation_constant.unwrap_or(0.)
    }

    // optional float attenuation_linear = 7;

    pub fn clear_attenuation_linear(&mut self) {
        self.attenuation_linear = ::std::option::Option::None;
    }

    pub fn has_attenuation_linear(&self) -> bool {
        self.attenuation_linear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attenuation_linear(&mut self, v: f32) {
        self.attenuation_linear = ::std::option::Option::Some(v);
    }

    pub fn get_attenuation_linear(&self) -> f32 {
        self.attenuation_linear.unwrap_or(0.)
    }

    // optional float attenuation_quadratic = 8;

    pub fn clear_attenuation_quadratic(&mut self) {
        self.attenuation_quadratic = ::std::option::Option::None;
    }

    pub fn has_attenuation_quadratic(&self) -> bool {
        self.attenuation_quadratic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attenuation_quadratic(&mut self, v: f32) {
        self.attenuation_quadratic = ::std::option::Option::Some(v);
    }

    pub fn get_attenuation_quadratic(&self) -> f32 {
        self.attenuation_quadratic.unwrap_or(0.)
    }

    // optional .gazebo.msgs.Vector3d direction = 9;

    pub fn clear_direction(&mut self) {
        self.direction.clear();
    }

    pub fn has_direction(&self) -> bool {
        self.direction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: super::vector3d::Vector3d) {
        self.direction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_direction(&mut self) -> &mut super::vector3d::Vector3d {
        if self.direction.is_none() {
            self.direction.set_default();
        };
        self.direction.as_mut().unwrap()
    }

    // Take field
    pub fn take_direction(&mut self) -> super::vector3d::Vector3d {
        self.direction.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_direction(&self) -> &super::vector3d::Vector3d {
        self.direction.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // optional float range = 10;

    pub fn clear_range(&mut self) {
        self.range = ::std::option::Option::None;
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: f32) {
        self.range = ::std::option::Option::Some(v);
    }

    pub fn get_range(&self) -> f32 {
        self.range.unwrap_or(0.)
    }

    // optional bool cast_shadows = 11;

    pub fn clear_cast_shadows(&mut self) {
        self.cast_shadows = ::std::option::Option::None;
    }

    pub fn has_cast_shadows(&self) -> bool {
        self.cast_shadows.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cast_shadows(&mut self, v: bool) {
        self.cast_shadows = ::std::option::Option::Some(v);
    }

    pub fn get_cast_shadows(&self) -> bool {
        self.cast_shadows.unwrap_or(false)
    }

    // optional float spot_inner_angle = 12;

    pub fn clear_spot_inner_angle(&mut self) {
        self.spot_inner_angle = ::std::option::Option::None;
    }

    pub fn has_spot_inner_angle(&self) -> bool {
        self.spot_inner_angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spot_inner_angle(&mut self, v: f32) {
        self.spot_inner_angle = ::std::option::Option::Some(v);
    }

    pub fn get_spot_inner_angle(&self) -> f32 {
        self.spot_inner_angle.unwrap_or(0.)
    }

    // optional float spot_outer_angle = 13;

    pub fn clear_spot_outer_angle(&mut self) {
        self.spot_outer_angle = ::std::option::Option::None;
    }

    pub fn has_spot_outer_angle(&self) -> bool {
        self.spot_outer_angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spot_outer_angle(&mut self, v: f32) {
        self.spot_outer_angle = ::std::option::Option::Some(v);
    }

    pub fn get_spot_outer_angle(&self) -> f32 {
        self.spot_outer_angle.unwrap_or(0.)
    }

    // optional float spot_falloff = 14;

    pub fn clear_spot_falloff(&mut self) {
        self.spot_falloff = ::std::option::Option::None;
    }

    pub fn has_spot_falloff(&self) -> bool {
        self.spot_falloff.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spot_falloff(&mut self, v: f32) {
        self.spot_falloff = ::std::option::Option::Some(v);
    }

    pub fn get_spot_falloff(&self) -> f32 {
        self.spot_falloff.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Light {
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
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diffuse));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.specular));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.attenuation_constant = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.attenuation_linear = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.attenuation_quadratic = ::std::option::Option::Some(tmp);
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.direction));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.range = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.cast_shadows = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.spot_inner_angle = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.spot_outer_angle = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.spot_falloff = ::std::option::Option::Some(tmp);
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.diffuse {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.specular {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.attenuation_constant.is_some() {
            my_size += 5;
        };
        if self.attenuation_linear.is_some() {
            my_size += 5;
        };
        if self.attenuation_quadratic.is_some() {
            my_size += 5;
        };
        for value in &self.direction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.range.is_some() {
            my_size += 5;
        };
        if self.cast_shadows.is_some() {
            my_size += 2;
        };
        if self.spot_inner_angle.is_some() {
            my_size += 5;
        };
        if self.spot_outer_angle.is_some() {
            my_size += 5;
        };
        if self.spot_falloff.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.diffuse.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.specular.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.attenuation_constant {
            try!(os.write_float(6, v));
        };
        if let Some(v) = self.attenuation_linear {
            try!(os.write_float(7, v));
        };
        if let Some(v) = self.attenuation_quadratic {
            try!(os.write_float(8, v));
        };
        if let Some(v) = self.direction.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.range {
            try!(os.write_float(10, v));
        };
        if let Some(v) = self.cast_shadows {
            try!(os.write_bool(11, v));
        };
        if let Some(v) = self.spot_inner_angle {
            try!(os.write_float(12, v));
        };
        if let Some(v) = self.spot_outer_angle {
            try!(os.write_float(13, v));
        };
        if let Some(v) = self.spot_falloff {
            try!(os.write_float(14, v));
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
        ::std::any::TypeId::of::<Light>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Light {
    fn new() -> Light {
        Light::new()
    }

    fn descriptor_static(_: ::std::option::Option<Light>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Light::has_name,
                    Light::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Light::has_field_type,
                    Light::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Light::has_pose,
                    Light::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "diffuse",
                    Light::has_diffuse,
                    Light::get_diffuse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "specular",
                    Light::has_specular,
                    Light::get_specular,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "attenuation_constant",
                    Light::has_attenuation_constant,
                    Light::get_attenuation_constant,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "attenuation_linear",
                    Light::has_attenuation_linear,
                    Light::get_attenuation_linear,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "attenuation_quadratic",
                    Light::has_attenuation_quadratic,
                    Light::get_attenuation_quadratic,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "direction",
                    Light::has_direction,
                    Light::get_direction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "range",
                    Light::has_range,
                    Light::get_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "cast_shadows",
                    Light::has_cast_shadows,
                    Light::get_cast_shadows,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "spot_inner_angle",
                    Light::has_spot_inner_angle,
                    Light::get_spot_inner_angle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "spot_outer_angle",
                    Light::has_spot_outer_angle,
                    Light::get_spot_outer_angle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "spot_falloff",
                    Light::has_spot_falloff,
                    Light::get_spot_falloff,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Light>(
                    "Light",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Light {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field_type();
        self.clear_pose();
        self.clear_diffuse();
        self.clear_specular();
        self.clear_attenuation_constant();
        self.clear_attenuation_linear();
        self.clear_attenuation_quadratic();
        self.clear_direction();
        self.clear_range();
        self.clear_cast_shadows();
        self.clear_spot_inner_angle();
        self.clear_spot_outer_angle();
        self.clear_spot_falloff();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Light {
    fn eq(&self, other: &Light) -> bool {
        self.name == other.name &&
        self.field_type == other.field_type &&
        self.pose == other.pose &&
        self.diffuse == other.diffuse &&
        self.specular == other.specular &&
        self.attenuation_constant == other.attenuation_constant &&
        self.attenuation_linear == other.attenuation_linear &&
        self.attenuation_quadratic == other.attenuation_quadratic &&
        self.direction == other.direction &&
        self.range == other.range &&
        self.cast_shadows == other.cast_shadows &&
        self.spot_inner_angle == other.spot_inner_angle &&
        self.spot_outer_angle == other.spot_outer_angle &&
        self.spot_falloff == other.spot_falloff &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Light {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Light_LightType {
    POINT = 1,
    SPOT = 2,
    DIRECTIONAL = 3,
}

impl ::protobuf::ProtobufEnum for Light_LightType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Light_LightType> {
        match value {
            1 => ::std::option::Option::Some(Light_LightType::POINT),
            2 => ::std::option::Option::Some(Light_LightType::SPOT),
            3 => ::std::option::Option::Some(Light_LightType::DIRECTIONAL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Light_LightType] = &[
            Light_LightType::POINT,
            Light_LightType::SPOT,
            Light_LightType::DIRECTIONAL,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Light_LightType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Light_LightType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Light_LightType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x70, 0x6f, 0x73, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0b, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0xd2, 0x03, 0x0a, 0x05, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x12, 0x0c, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x2a, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x67, 0x61, 0x7a, 0x65,
    0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x2e, 0x4c, 0x69,
    0x67, 0x68, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x23, 0x0a, 0x07, 0x64, 0x69, 0x66, 0x66,
    0x75, 0x73, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65,
    0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x24, 0x0a,
    0x08, 0x73, 0x70, 0x65, 0x63, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x43, 0x6f,
    0x6c, 0x6f, 0x72, 0x12, 0x1c, 0x0a, 0x14, 0x61, 0x74, 0x74, 0x65, 0x6e, 0x75, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x02, 0x12, 0x1a, 0x0a, 0x12, 0x61, 0x74, 0x74, 0x65, 0x6e, 0x75, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x6c, 0x69, 0x6e, 0x65, 0x61, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x12, 0x1d, 0x0a,
    0x15, 0x61, 0x74, 0x74, 0x65, 0x6e, 0x75, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x71, 0x75, 0x61,
    0x64, 0x72, 0x61, 0x74, 0x69, 0x63, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x12, 0x28, 0x0a, 0x09,
    0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x12, 0x0d, 0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x02, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x61, 0x73, 0x74, 0x5f, 0x73, 0x68,
    0x61, 0x64, 0x6f, 0x77, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x12, 0x18, 0x0a, 0x10, 0x73,
    0x70, 0x6f, 0x74, 0x5f, 0x69, 0x6e, 0x6e, 0x65, 0x72, 0x5f, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18,
    0x0c, 0x20, 0x01, 0x28, 0x02, 0x12, 0x18, 0x0a, 0x10, 0x73, 0x70, 0x6f, 0x74, 0x5f, 0x6f, 0x75,
    0x74, 0x65, 0x72, 0x5f, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x02, 0x12,
    0x14, 0x0a, 0x0c, 0x73, 0x70, 0x6f, 0x74, 0x5f, 0x66, 0x61, 0x6c, 0x6c, 0x6f, 0x66, 0x66, 0x18,
    0x0e, 0x20, 0x01, 0x28, 0x02, 0x22, 0x31, 0x0a, 0x09, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x50, 0x4f, 0x49, 0x4e, 0x54, 0x10, 0x01, 0x12, 0x08, 0x0a,
    0x04, 0x53, 0x50, 0x4f, 0x54, 0x10, 0x02, 0x12, 0x0f, 0x0a, 0x0b, 0x44, 0x49, 0x52, 0x45, 0x43,
    0x54, 0x49, 0x4f, 0x4e, 0x41, 0x4c, 0x10, 0x03, 0x4a, 0xba, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x23, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x08, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x09, 0x07, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0a, 0x07, 0x14, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0c, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0e, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0e, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x00, 0x04, 0x00, 0x12, 0x04, 0x0f, 0x02, 0x14, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x0f, 0x07, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x11, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x11, 0x12, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x12, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x12, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x12, 0x12, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x13, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x13, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x13, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x15, 0x02,
    0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x15, 0x0b, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x17, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x17,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x10, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x2b, 0x2c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x18, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x18, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x18, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x18, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x19, 0x02, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x19, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x19, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x19, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x1a, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x1a, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1a, 0x11, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1a, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x1b, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x1b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x1b, 0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1b,
    0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x1c, 0x02, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1c, 0x11, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x1c, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12,
    0x03, 0x1d, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x1d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x1d, 0x0b, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1d, 0x14, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x1d, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x1e, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x09, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05,
    0x12, 0x03, 0x1e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x1e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1e, 0x2b,
    0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x1f, 0x02, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x1f, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x1f, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x1f, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03,
    0x20, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x20, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x20, 0x11, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x20, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x0c, 0x12, 0x03, 0x21, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c,
    0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12,
    0x03, 0x21, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x21,
    0x11, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x21, 0x2b, 0x2d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x22, 0x02, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x22, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0d, 0x01, 0x12, 0x03, 0x22, 0x11, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03,
    0x12, 0x03, 0x22, 0x2b, 0x2d,
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
