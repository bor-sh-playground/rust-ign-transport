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
pub struct ModelConfiguration {
    // message fields
    time: ::protobuf::SingularPtrField<super::time::Time>,
    joint_names: ::protobuf::RepeatedField<::std::string::String>,
    joint_positions: ::std::vec::Vec<f64>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    link_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ModelConfiguration {}

impl ModelConfiguration {
    pub fn new() -> ModelConfiguration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ModelConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<ModelConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ModelConfiguration,
        };
        unsafe {
            instance.get(|| {
                ModelConfiguration {
                    time: ::protobuf::SingularPtrField::none(),
                    joint_names: ::protobuf::RepeatedField::new(),
                    joint_positions: ::std::vec::Vec::new(),
                    pose: ::protobuf::SingularPtrField::none(),
                    link_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Time time = 1;

    pub fn clear_time(&mut self) {
        self.time.clear();
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: super::time::Time) {
        self.time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_time(&mut self) -> &mut super::time::Time {
        if self.time.is_none() {
            self.time.set_default();
        };
        self.time.as_mut().unwrap()
    }

    // Take field
    pub fn take_time(&mut self) -> super::time::Time {
        self.time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_time(&self) -> &super::time::Time {
        self.time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // repeated string joint_names = 2;

    pub fn clear_joint_names(&mut self) {
        self.joint_names.clear();
    }

    // Param is passed by value, moved
    pub fn set_joint_names(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.joint_names = v;
    }

    // Mutable pointer to the field.
    pub fn mut_joint_names(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.joint_names
    }

    // Take field
    pub fn take_joint_names(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.joint_names, ::protobuf::RepeatedField::new())
    }

    pub fn get_joint_names(&self) -> &[::std::string::String] {
        &self.joint_names
    }

    // repeated double joint_positions = 3;

    pub fn clear_joint_positions(&mut self) {
        self.joint_positions.clear();
    }

    // Param is passed by value, moved
    pub fn set_joint_positions(&mut self, v: ::std::vec::Vec<f64>) {
        self.joint_positions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_joint_positions(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.joint_positions
    }

    // Take field
    pub fn take_joint_positions(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.joint_positions, ::std::vec::Vec::new())
    }

    pub fn get_joint_positions(&self) -> &[f64] {
        &self.joint_positions
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

    // optional string link_name = 5;

    pub fn clear_link_name(&mut self) {
        self.link_name.clear();
    }

    pub fn has_link_name(&self) -> bool {
        self.link_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_link_name(&mut self, v: ::std::string::String) {
        self.link_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_link_name(&mut self) -> &mut ::std::string::String {
        if self.link_name.is_none() {
            self.link_name.set_default();
        };
        self.link_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_link_name(&mut self) -> ::std::string::String {
        self.link_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_link_name(&self) -> &str {
        match self.link_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ModelConfiguration {
    fn is_initialized(&self) -> bool {
        if self.time.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.time));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.joint_names));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.joint_positions));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.link_name));
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
        for value in &self.time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.joint_names {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += 9 * self.joint_positions.len() as u32;
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.link_name {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.joint_names {
            try!(os.write_string(2, &v));
        };
        for v in &self.joint_positions {
            try!(os.write_double(3, *v));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.link_name.as_ref() {
            try!(os.write_string(5, &v));
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
        ::std::any::TypeId::of::<ModelConfiguration>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ModelConfiguration {
    fn new() -> ModelConfiguration {
        ModelConfiguration::new()
    }

    fn descriptor_static(_: ::std::option::Option<ModelConfiguration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "time",
                    ModelConfiguration::has_time,
                    ModelConfiguration::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "joint_names",
                    ModelConfiguration::get_joint_names,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "joint_positions",
                    ModelConfiguration::get_joint_positions,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    ModelConfiguration::has_pose,
                    ModelConfiguration::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "link_name",
                    ModelConfiguration::has_link_name,
                    ModelConfiguration::get_link_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ModelConfiguration>(
                    "ModelConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ModelConfiguration {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_joint_names();
        self.clear_joint_positions();
        self.clear_pose();
        self.clear_link_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ModelConfiguration {
    fn eq(&self, other: &ModelConfiguration) -> bool {
        self.time == other.time &&
        self.joint_names == other.joint_names &&
        self.joint_positions == other.joint_positions &&
        self.pose == other.pose &&
        self.link_name == other.link_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ModelConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x97, 0x01, 0x0a, 0x12, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x13, 0x0a, 0x0b, 0x6a, 0x6f, 0x69, 0x6e,
    0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x12, 0x17, 0x0a,
    0x0f, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x18, 0x03, 0x20, 0x03, 0x28, 0x01, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x6c, 0x69, 0x6e, 0x6b, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x4a, 0x8f, 0x04, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x11, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x08, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x11,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x1a, 0x0a, 0x34, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x30, 0x22, 0x27, 0x20, 0x54, 0x69, 0x6d,
    0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x20,
    0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63,
    0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x10, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d,
    0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x2e, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x02, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0e, 0x2e, 0x2f, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0f,
    0x02, 0x30, 0x22, 0x14, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20, 0x6d, 0x6f, 0x64,
    0x65, 0x6c, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x0f, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0f,
    0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0f, 0x2e, 0x2f,
    0x0a, 0x42, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x30, 0x22, 0x35, 0x20,
    0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6d, 0x6f,
    0x64, 0x65, 0x6c, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x20, 0x62, 0x79, 0x20, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x6c,
    0x69, 0x6e, 0x6b, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x10,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x10, 0x12, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x10, 0x2e, 0x2f,
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
