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
pub struct Material {
    // message fields
    script: ::protobuf::SingularPtrField<Material_Script>,
    shader_type: ::std::option::Option<Material_ShaderType>,
    normal_map: ::protobuf::SingularField<::std::string::String>,
    ambient: ::protobuf::SingularPtrField<super::color::Color>,
    diffuse: ::protobuf::SingularPtrField<super::color::Color>,
    specular: ::protobuf::SingularPtrField<super::color::Color>,
    emissive: ::protobuf::SingularPtrField<super::color::Color>,
    lighting: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Material {}

impl Material {
    pub fn new() -> Material {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Material {
        static mut instance: ::protobuf::lazy::Lazy<Material> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Material,
        };
        unsafe {
            instance.get(|| {
                Material {
                    script: ::protobuf::SingularPtrField::none(),
                    shader_type: ::std::option::Option::None,
                    normal_map: ::protobuf::SingularField::none(),
                    ambient: ::protobuf::SingularPtrField::none(),
                    diffuse: ::protobuf::SingularPtrField::none(),
                    specular: ::protobuf::SingularPtrField::none(),
                    emissive: ::protobuf::SingularPtrField::none(),
                    lighting: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gazebo.msgs.Material.Script script = 1;

    pub fn clear_script(&mut self) {
        self.script.clear();
    }

    pub fn has_script(&self) -> bool {
        self.script.is_some()
    }

    // Param is passed by value, moved
    pub fn set_script(&mut self, v: Material_Script) {
        self.script = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_script(&mut self) -> &mut Material_Script {
        if self.script.is_none() {
            self.script.set_default();
        };
        self.script.as_mut().unwrap()
    }

    // Take field
    pub fn take_script(&mut self) -> Material_Script {
        self.script.take().unwrap_or_else(|| Material_Script::new())
    }

    pub fn get_script(&self) -> &Material_Script {
        self.script.as_ref().unwrap_or_else(|| Material_Script::default_instance())
    }

    // optional .gazebo.msgs.Material.ShaderType shader_type = 2;

    pub fn clear_shader_type(&mut self) {
        self.shader_type = ::std::option::Option::None;
    }

    pub fn has_shader_type(&self) -> bool {
        self.shader_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shader_type(&mut self, v: Material_ShaderType) {
        self.shader_type = ::std::option::Option::Some(v);
    }

    pub fn get_shader_type(&self) -> Material_ShaderType {
        self.shader_type.unwrap_or(Material_ShaderType::VERTEX)
    }

    // optional string normal_map = 3;

    pub fn clear_normal_map(&mut self) {
        self.normal_map.clear();
    }

    pub fn has_normal_map(&self) -> bool {
        self.normal_map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normal_map(&mut self, v: ::std::string::String) {
        self.normal_map = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_normal_map(&mut self) -> &mut ::std::string::String {
        if self.normal_map.is_none() {
            self.normal_map.set_default();
        };
        self.normal_map.as_mut().unwrap()
    }

    // Take field
    pub fn take_normal_map(&mut self) -> ::std::string::String {
        self.normal_map.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_normal_map(&self) -> &str {
        match self.normal_map.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .gazebo.msgs.Color ambient = 4;

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

    // optional .gazebo.msgs.Color diffuse = 5;

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

    // optional .gazebo.msgs.Color specular = 6;

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

    // optional .gazebo.msgs.Color emissive = 7;

    pub fn clear_emissive(&mut self) {
        self.emissive.clear();
    }

    pub fn has_emissive(&self) -> bool {
        self.emissive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_emissive(&mut self, v: super::color::Color) {
        self.emissive = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_emissive(&mut self) -> &mut super::color::Color {
        if self.emissive.is_none() {
            self.emissive.set_default();
        };
        self.emissive.as_mut().unwrap()
    }

    // Take field
    pub fn take_emissive(&mut self) -> super::color::Color {
        self.emissive.take().unwrap_or_else(|| super::color::Color::new())
    }

    pub fn get_emissive(&self) -> &super::color::Color {
        self.emissive.as_ref().unwrap_or_else(|| super::color::Color::default_instance())
    }

    // optional bool lighting = 8;

    pub fn clear_lighting(&mut self) {
        self.lighting = ::std::option::Option::None;
    }

    pub fn has_lighting(&self) -> bool {
        self.lighting.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lighting(&mut self, v: bool) {
        self.lighting = ::std::option::Option::Some(v);
    }

    pub fn get_lighting(&self) -> bool {
        self.lighting.unwrap_or(false)
    }
}

impl ::protobuf::Message for Material {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.script));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.shader_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.normal_map));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ambient));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diffuse));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.specular));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.emissive));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.lighting = ::std::option::Option::Some(tmp);
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
        for value in &self.script {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.shader_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.normal_map {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.ambient {
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
        for value in &self.emissive {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.lighting.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.script.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.shader_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.normal_map.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.ambient.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.diffuse.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.specular.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.emissive.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.lighting {
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
        ::std::any::TypeId::of::<Material>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Material {
    fn new() -> Material {
        Material::new()
    }

    fn descriptor_static(_: ::std::option::Option<Material>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "script",
                    Material::has_script,
                    Material::get_script,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "shader_type",
                    Material::has_shader_type,
                    Material::get_shader_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "normal_map",
                    Material::has_normal_map,
                    Material::get_normal_map,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ambient",
                    Material::has_ambient,
                    Material::get_ambient,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "diffuse",
                    Material::has_diffuse,
                    Material::get_diffuse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "specular",
                    Material::has_specular,
                    Material::get_specular,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "emissive",
                    Material::has_emissive,
                    Material::get_emissive,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "lighting",
                    Material::has_lighting,
                    Material::get_lighting,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Material>(
                    "Material",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Material {
    fn clear(&mut self) {
        self.clear_script();
        self.clear_shader_type();
        self.clear_normal_map();
        self.clear_ambient();
        self.clear_diffuse();
        self.clear_specular();
        self.clear_emissive();
        self.clear_lighting();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Material {
    fn eq(&self, other: &Material) -> bool {
        self.script == other.script &&
        self.shader_type == other.shader_type &&
        self.normal_map == other.normal_map &&
        self.ambient == other.ambient &&
        self.diffuse == other.diffuse &&
        self.specular == other.specular &&
        self.emissive == other.emissive &&
        self.lighting == other.lighting &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Material {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Material_Script {
    // message fields
    uri: ::protobuf::RepeatedField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Material_Script {}

impl Material_Script {
    pub fn new() -> Material_Script {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Material_Script {
        static mut instance: ::protobuf::lazy::Lazy<Material_Script> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Material_Script,
        };
        unsafe {
            instance.get(|| {
                Material_Script {
                    uri: ::protobuf::RepeatedField::new(),
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string uri = 1;

    pub fn clear_uri(&mut self) {
        self.uri.clear();
    }

    // Param is passed by value, moved
    pub fn set_uri(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.uri = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uri(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.uri
    }

    // Take field
    pub fn take_uri(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.uri, ::protobuf::RepeatedField::new())
    }

    pub fn get_uri(&self) -> &[::std::string::String] {
        &self.uri
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
}

impl ::protobuf::Message for Material_Script {
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
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.uri));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
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
        for value in &self.uri {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.uri {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<Material_Script>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Material_Script {
    fn new() -> Material_Script {
        Material_Script::new()
    }

    fn descriptor_static(_: ::std::option::Option<Material_Script>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "uri",
                    Material_Script::get_uri,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Material_Script::has_name,
                    Material_Script::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Material_Script>(
                    "Material_Script",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Material_Script {
    fn clear(&mut self) {
        self.clear_uri();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Material_Script {
    fn eq(&self, other: &Material_Script) -> bool {
        self.uri == other.uri &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Material_Script {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Material_ShaderType {
    VERTEX = 1,
    PIXEL = 2,
    NORMAL_MAP_OBJECT_SPACE = 3,
    NORMAL_MAP_TANGENT_SPACE = 4,
}

impl ::protobuf::ProtobufEnum for Material_ShaderType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Material_ShaderType> {
        match value {
            1 => ::std::option::Option::Some(Material_ShaderType::VERTEX),
            2 => ::std::option::Option::Some(Material_ShaderType::PIXEL),
            3 => ::std::option::Option::Some(Material_ShaderType::NORMAL_MAP_OBJECT_SPACE),
            4 => ::std::option::Option::Some(Material_ShaderType::NORMAL_MAP_TANGENT_SPACE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Material_ShaderType] = &[
            Material_ShaderType::VERTEX,
            Material_ShaderType::PIXEL,
            Material_ShaderType::NORMAL_MAP_OBJECT_SPACE,
            Material_ShaderType::NORMAL_MAP_TANGENT_SPACE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Material_ShaderType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Material_ShaderType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Material_ShaderType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x6d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0b, 0x63,
    0x6f, 0x6c, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb0, 0x03, 0x0a, 0x08, 0x4d,
    0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x12, 0x2c, 0x0a, 0x06, 0x73, 0x63, 0x72, 0x69, 0x70,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f,
    0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x2e, 0x53,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x12, 0x35, 0x0a, 0x0b, 0x73, 0x68, 0x61, 0x64, 0x65, 0x72, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x20, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61,
    0x6c, 0x2e, 0x53, 0x68, 0x61, 0x64, 0x65, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x0a,
    0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x5f, 0x6d, 0x61, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x23, 0x0a, 0x07, 0x61, 0x6d, 0x62, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x23, 0x0a, 0x07, 0x64, 0x69, 0x66, 0x66, 0x75, 0x73, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e,
    0x6d, 0x73, 0x67, 0x73, 0x2e, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x24, 0x0a, 0x08, 0x73, 0x70,
    0x65, 0x63, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x43, 0x6f, 0x6c, 0x6f, 0x72,
    0x12, 0x24, 0x0a, 0x08, 0x65, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x76, 0x65, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x69,
    0x6e, 0x67, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x1a, 0x23, 0x0a, 0x06, 0x53, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x12, 0x0b, 0x0a, 0x03, 0x75, 0x72, 0x69, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x12,
    0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x5e, 0x0a,
    0x0a, 0x53, 0x68, 0x61, 0x64, 0x65, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x56,
    0x45, 0x52, 0x54, 0x45, 0x58, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x50, 0x49, 0x58, 0x45, 0x4c,
    0x10, 0x02, 0x12, 0x1b, 0x0a, 0x17, 0x4e, 0x4f, 0x52, 0x4d, 0x41, 0x4c, 0x5f, 0x4d, 0x41, 0x50,
    0x5f, 0x4f, 0x42, 0x4a, 0x45, 0x43, 0x54, 0x5f, 0x53, 0x50, 0x41, 0x43, 0x45, 0x10, 0x03, 0x12,
    0x1c, 0x0a, 0x18, 0x4e, 0x4f, 0x52, 0x4d, 0x41, 0x4c, 0x5f, 0x4d, 0x41, 0x50, 0x5f, 0x54, 0x41,
    0x4e, 0x47, 0x45, 0x4e, 0x54, 0x5f, 0x53, 0x50, 0x41, 0x43, 0x45, 0x10, 0x04, 0x4a, 0xef, 0x07,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x21, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00,
    0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x14, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x0b,
    0x02, 0x11, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x07,
    0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x20,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x1e, 0x1f,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x20, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x09, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0e, 0x1e, 0x1f, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x04, 0x20, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x1b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x1e, 0x1f, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x10, 0x04, 0x21, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x04, 0x1c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x10, 0x1f, 0x20, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x13, 0x02, 0x17, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x13, 0x0a, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x15, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x14, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x16, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x16, 0x14, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x16, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x19, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x19, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x12, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x1a, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1a, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1b, 0x02, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x1b, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x1c, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1c, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1c, 0x11, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1c, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x1d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1d,
    0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1d, 0x24, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1e, 0x02, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x1e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x1e, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x1e, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x1f,
    0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x1f, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1f, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1f, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x20, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04,
    0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03,
    0x20, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x20, 0x10,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x20, 0x24, 0x25,
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
