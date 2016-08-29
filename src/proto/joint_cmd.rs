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
pub struct JointCmd {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    axis: ::std::option::Option<i32>,
    force: ::std::option::Option<f64>,
    position: ::protobuf::SingularPtrField<super::pid::PID>,
    velocity: ::protobuf::SingularPtrField<super::pid::PID>,
    reset: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JointCmd {}

impl JointCmd {
    pub fn new() -> JointCmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JointCmd {
        static mut instance: ::protobuf::lazy::Lazy<JointCmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JointCmd,
        };
        unsafe {
            instance.get(|| {
                JointCmd {
                    name: ::protobuf::SingularField::none(),
                    axis: ::std::option::Option::None,
                    force: ::std::option::Option::None,
                    position: ::protobuf::SingularPtrField::none(),
                    velocity: ::protobuf::SingularPtrField::none(),
                    reset: ::std::option::Option::None,
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

    // optional int32 axis = 2;

    pub fn clear_axis(&mut self) {
        self.axis = ::std::option::Option::None;
    }

    pub fn has_axis(&self) -> bool {
        self.axis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_axis(&mut self, v: i32) {
        self.axis = ::std::option::Option::Some(v);
    }

    pub fn get_axis(&self) -> i32 {
        self.axis.unwrap_or(0i32)
    }

    // optional double force = 3;

    pub fn clear_force(&mut self) {
        self.force = ::std::option::Option::None;
    }

    pub fn has_force(&self) -> bool {
        self.force.is_some()
    }

    // Param is passed by value, moved
    pub fn set_force(&mut self, v: f64) {
        self.force = ::std::option::Option::Some(v);
    }

    pub fn get_force(&self) -> f64 {
        self.force.unwrap_or(0.)
    }

    // optional .gazebo.msgs.PID position = 4;

    pub fn clear_position(&mut self) {
        self.position.clear();
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: super::pid::PID) {
        self.position = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_position(&mut self) -> &mut super::pid::PID {
        if self.position.is_none() {
            self.position.set_default();
        };
        self.position.as_mut().unwrap()
    }

    // Take field
    pub fn take_position(&mut self) -> super::pid::PID {
        self.position.take().unwrap_or_else(|| super::pid::PID::new())
    }

    pub fn get_position(&self) -> &super::pid::PID {
        self.position.as_ref().unwrap_or_else(|| super::pid::PID::default_instance())
    }

    // optional .gazebo.msgs.PID velocity = 5;

    pub fn clear_velocity(&mut self) {
        self.velocity.clear();
    }

    pub fn has_velocity(&self) -> bool {
        self.velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity(&mut self, v: super::pid::PID) {
        self.velocity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_velocity(&mut self) -> &mut super::pid::PID {
        if self.velocity.is_none() {
            self.velocity.set_default();
        };
        self.velocity.as_mut().unwrap()
    }

    // Take field
    pub fn take_velocity(&mut self) -> super::pid::PID {
        self.velocity.take().unwrap_or_else(|| super::pid::PID::new())
    }

    pub fn get_velocity(&self) -> &super::pid::PID {
        self.velocity.as_ref().unwrap_or_else(|| super::pid::PID::default_instance())
    }

    // optional bool reset = 6;

    pub fn clear_reset(&mut self) {
        self.reset = ::std::option::Option::None;
    }

    pub fn has_reset(&self) -> bool {
        self.reset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reset(&mut self, v: bool) {
        self.reset = ::std::option::Option::Some(v);
    }

    pub fn get_reset(&self) -> bool {
        self.reset.unwrap_or(false)
    }
}

impl ::protobuf::Message for JointCmd {
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
                    let tmp = try!(is.read_int32());
                    self.axis = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.force = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.position));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.velocity));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.reset = ::std::option::Option::Some(tmp);
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
        for value in &self.axis {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.force.is_some() {
            my_size += 9;
        };
        for value in &self.position {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.velocity {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.reset.is_some() {
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
        if let Some(v) = self.axis {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.force {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.position.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.velocity.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.reset {
            try!(os.write_bool(6, v));
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
        ::std::any::TypeId::of::<JointCmd>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for JointCmd {
    fn new() -> JointCmd {
        JointCmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<JointCmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    JointCmd::has_name,
                    JointCmd::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "axis",
                    JointCmd::has_axis,
                    JointCmd::get_axis,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "force",
                    JointCmd::has_force,
                    JointCmd::get_force,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "position",
                    JointCmd::has_position,
                    JointCmd::get_position,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "velocity",
                    JointCmd::has_velocity,
                    JointCmd::get_velocity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "reset",
                    JointCmd::has_reset,
                    JointCmd::get_reset,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JointCmd>(
                    "JointCmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JointCmd {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_axis();
        self.clear_force();
        self.clear_position();
        self.clear_velocity();
        self.clear_reset();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for JointCmd {
    fn eq(&self, other: &JointCmd) -> bool {
        self.name == other.name &&
        self.axis == other.axis &&
        self.force == other.force &&
        self.position == other.position &&
        self.velocity == other.velocity &&
        self.reset == other.reset &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for JointCmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x09,
    0x70, 0x69, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x8f, 0x01, 0x0a, 0x08, 0x4a, 0x6f,
    0x69, 0x6e, 0x74, 0x43, 0x6d, 0x64, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x04, 0x61, 0x78, 0x69, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x3a, 0x01, 0x30, 0x12, 0x0d, 0x0a, 0x05, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x01, 0x12, 0x22, 0x0a, 0x08, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e,
    0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x49, 0x44, 0x12, 0x22, 0x0a, 0x08, 0x76, 0x65, 0x6c, 0x6f,
    0x63, 0x69, 0x74, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x49, 0x44, 0x12, 0x0d, 0x0a, 0x05,
    0x72, 0x65, 0x73, 0x65, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x4a, 0xef, 0x03, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x12, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x09, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x0c, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x11, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x1f, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x07, 0x12, 0x03, 0x0c, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x0d, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x1d, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x0f, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x0f, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x0f, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x10, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x10, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x10, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x11, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x11, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x11, 0x10, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x11, 0x1d, 0x1e,
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
