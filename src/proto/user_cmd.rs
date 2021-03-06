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
pub struct UserCmd {
    // message fields
    id: ::std::option::Option<u32>,
    description: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<UserCmd_Type>,
    model: ::protobuf::RepeatedField<super::model::Model>,
    light: ::protobuf::RepeatedField<super::light::Light>,
    entity_name: ::protobuf::SingularField<::std::string::String>,
    world_control: ::protobuf::SingularPtrField<super::world_control::WorldControl>,
    wrench: ::protobuf::SingularPtrField<super::wrench::Wrench>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserCmd {}

impl UserCmd {
    pub fn new() -> UserCmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserCmd {
        static mut instance: ::protobuf::lazy::Lazy<UserCmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserCmd,
        };
        unsafe {
            instance.get(|| {
                UserCmd {
                    id: ::std::option::Option::None,
                    description: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    model: ::protobuf::RepeatedField::new(),
                    light: ::protobuf::RepeatedField::new(),
                    entity_name: ::protobuf::SingularField::none(),
                    world_control: ::protobuf::SingularPtrField::none(),
                    wrench: ::protobuf::SingularPtrField::none(),
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

    // required string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .gazebo.msgs.UserCmd.Type type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: UserCmd_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> UserCmd_Type {
        self.field_type.unwrap_or(UserCmd_Type::MOVING)
    }

    // repeated .gazebo.msgs.Model model = 4;

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

    // repeated .gazebo.msgs.Light light = 5;

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

    // optional string entity_name = 6;

    pub fn clear_entity_name(&mut self) {
        self.entity_name.clear();
    }

    pub fn has_entity_name(&self) -> bool {
        self.entity_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_name(&mut self, v: ::std::string::String) {
        self.entity_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity_name(&mut self) -> &mut ::std::string::String {
        if self.entity_name.is_none() {
            self.entity_name.set_default();
        };
        self.entity_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity_name(&mut self) -> ::std::string::String {
        self.entity_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_entity_name(&self) -> &str {
        match self.entity_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .gazebo.msgs.WorldControl world_control = 7;

    pub fn clear_world_control(&mut self) {
        self.world_control.clear();
    }

    pub fn has_world_control(&self) -> bool {
        self.world_control.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_control(&mut self, v: super::world_control::WorldControl) {
        self.world_control = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_control(&mut self) -> &mut super::world_control::WorldControl {
        if self.world_control.is_none() {
            self.world_control.set_default();
        };
        self.world_control.as_mut().unwrap()
    }

    // Take field
    pub fn take_world_control(&mut self) -> super::world_control::WorldControl {
        self.world_control.take().unwrap_or_else(|| super::world_control::WorldControl::new())
    }

    pub fn get_world_control(&self) -> &super::world_control::WorldControl {
        self.world_control.as_ref().unwrap_or_else(|| super::world_control::WorldControl::default_instance())
    }

    // optional .gazebo.msgs.Wrench wrench = 8;

    pub fn clear_wrench(&mut self) {
        self.wrench.clear();
    }

    pub fn has_wrench(&self) -> bool {
        self.wrench.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wrench(&mut self, v: super::wrench::Wrench) {
        self.wrench = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wrench(&mut self) -> &mut super::wrench::Wrench {
        if self.wrench.is_none() {
            self.wrench.set_default();
        };
        self.wrench.as_mut().unwrap()
    }

    // Take field
    pub fn take_wrench(&mut self) -> super::wrench::Wrench {
        self.wrench.take().unwrap_or_else(|| super::wrench::Wrench::new())
    }

    pub fn get_wrench(&self) -> &super::wrench::Wrench {
        self.wrench.as_ref().unwrap_or_else(|| super::wrench::Wrench::default_instance())
    }
}

impl ::protobuf::Message for UserCmd {
    fn is_initialized(&self) -> bool {
        if self.description.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.model));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.light));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.entity_name));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.world_control));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.wrench));
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
        for value in &self.description {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in &self.model {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.light {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.entity_name {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        for value in &self.world_control {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.wrench {
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
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(3, v.value()));
        };
        for v in &self.model {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.light {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.entity_name.as_ref() {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.world_control.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.wrench.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<UserCmd>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UserCmd {
    fn new() -> UserCmd {
        UserCmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserCmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    UserCmd::has_id,
                    UserCmd::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    UserCmd::has_description,
                    UserCmd::get_description,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    UserCmd::has_field_type,
                    UserCmd::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "model",
                    UserCmd::get_model,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "light",
                    UserCmd::get_light,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "entity_name",
                    UserCmd::has_entity_name,
                    UserCmd::get_entity_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "world_control",
                    UserCmd::has_world_control,
                    UserCmd::get_world_control,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "wrench",
                    UserCmd::has_wrench,
                    UserCmd::get_wrench,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserCmd>(
                    "UserCmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserCmd {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_description();
        self.clear_field_type();
        self.clear_model();
        self.clear_light();
        self.clear_entity_name();
        self.clear_world_control();
        self.clear_wrench();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UserCmd {
    fn eq(&self, other: &UserCmd) -> bool {
        self.id == other.id &&
        self.description == other.description &&
        self.field_type == other.field_type &&
        self.model == other.model &&
        self.light == other.light &&
        self.entity_name == other.entity_name &&
        self.world_control == other.world_control &&
        self.wrench == other.wrench &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UserCmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UserCmd_Type {
    MOVING = 1,
    WORLD_CONTROL = 2,
    WRENCH = 3,
    SCALING = 4,
}

impl ::protobuf::ProtobufEnum for UserCmd_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UserCmd_Type> {
        match value {
            1 => ::std::option::Option::Some(UserCmd_Type::MOVING),
            2 => ::std::option::Option::Some(UserCmd_Type::WORLD_CONTROL),
            3 => ::std::option::Option::Some(UserCmd_Type::WRENCH),
            4 => ::std::option::Option::Some(UserCmd_Type::SCALING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UserCmd_Type] = &[
            UserCmd_Type::MOVING,
            UserCmd_Type::WORLD_CONTROL,
            UserCmd_Type::WRENCH,
            UserCmd_Type::SCALING,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<UserCmd_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UserCmd_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UserCmd_Type {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x63, 0x6d, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0b, 0x6c,
    0x69, 0x67, 0x68, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0b, 0x6d, 0x6f, 0x64, 0x65,
    0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x77, 0x72,
    0x65, 0x6e, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc5, 0x02, 0x0a, 0x07, 0x55,
    0x73, 0x65, 0x72, 0x43, 0x6d, 0x64, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x27, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x43, 0x6d, 0x64, 0x2e, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x21, 0x0a, 0x05, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4d, 0x6f,
    0x64, 0x65, 0x6c, 0x12, 0x21, 0x0a, 0x05, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x18, 0x05, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x12, 0x13, 0x0a, 0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x12, 0x30, 0x0a, 0x0d, 0x77,
    0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x12, 0x23, 0x0a,
    0x06, 0x77, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x57, 0x72, 0x65, 0x6e,
    0x63, 0x68, 0x22, 0x3e, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x4d, 0x4f,
    0x56, 0x49, 0x4e, 0x47, 0x10, 0x01, 0x12, 0x11, 0x0a, 0x0d, 0x57, 0x4f, 0x52, 0x4c, 0x44, 0x5f,
    0x43, 0x4f, 0x4e, 0x54, 0x52, 0x4f, 0x4c, 0x10, 0x02, 0x12, 0x0a, 0x0a, 0x06, 0x57, 0x52, 0x45,
    0x4e, 0x43, 0x48, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x43, 0x41, 0x4c, 0x49, 0x4e, 0x47,
    0x10, 0x04, 0x4a, 0xaa, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x34, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02,
    0x07, 0x14, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x03, 0x07, 0x14, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x04, 0x07, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x05, 0x07, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x34, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x0f, 0x0a, 0x2e, 0x0a, 0x04, 0x04,
    0x00, 0x04, 0x00, 0x12, 0x04, 0x0e, 0x02, 0x1b, 0x03, 0x1a, 0x20, 0x2f, 0x20, 0x5c, 0x62, 0x72,
    0x69, 0x65, 0x66, 0x20, 0x54, 0x79, 0x70, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x75, 0x73, 0x65,
    0x72, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x07, 0x0b, 0x0a, 0x2a, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x11, 0x04, 0x0f, 0x1a, 0x1b, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69,
    0x65, 0x66, 0x20, 0x4d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x74,
    0x69, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x11, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x11, 0x0d, 0x0e, 0x0a, 0x2f, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x14, 0x04, 0x16, 0x1a, 0x20, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x43,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77,
    0x6f, 0x72, 0x6c, 0x64, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x14, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x14, 0x14, 0x15, 0x0a, 0x29, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x17, 0x04, 0x0f, 0x1a, 0x1a, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20,
    0x41, 0x70, 0x70, 0x6c, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x2e,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x17, 0x0d,
    0x0e, 0x0a, 0x2b, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x04, 0x10,
    0x1a, 0x1c, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x53, 0x63, 0x61, 0x6c, 0x69,
    0x6e, 0x67, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1a, 0x04, 0x0b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x1a, 0x0e, 0x0f, 0x0a, 0x32,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x02, 0x19, 0x1a, 0x25, 0x2f, 0x20, 0x5c,
    0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x55, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x69, 0x64, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x17, 0x18, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x21, 0x02, 0x22, 0x1a, 0x26, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65,
    0x66, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x21, 0x20, 0x21, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x24, 0x02, 0x19, 0x1a, 0x1a, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x54,
    0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x24, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x24, 0x17, 0x18, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x27, 0x02, 0x1b, 0x1a, 0x24, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20,
    0x46, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x20, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x79,
    0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x27, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x27, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x27, 0x19, 0x1a, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x1b,
    0x1a, 0x24, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x6c,
    0x69, 0x67, 0x68, 0x74, 0x20, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x79, 0x20, 0x63, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x2a,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2a, 0x11, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2a, 0x19, 0x1a, 0x0a, 0x39,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x02, 0x22, 0x1a, 0x2c, 0x2f, 0x20, 0x5c,
    0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x65, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x20, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79,
    0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x2d, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2d, 0x20,
    0x21, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x30, 0x02, 0x2a, 0x1a, 0x25,
    0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x57, 0x6f, 0x72,
    0x6c, 0x64, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61,
    0x6e, 0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x30, 0x0b,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x30, 0x18, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x30, 0x28, 0x29, 0x0a, 0x38, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x33, 0x02, 0x1d, 0x1a, 0x2b, 0x2f, 0x20, 0x5c, 0x62,
    0x72, 0x69, 0x65, 0x66, 0x20, 0x57, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x61, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x77, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04,
    0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03,
    0x33, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x33, 0x12,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x33, 0x1b, 0x1c,
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
