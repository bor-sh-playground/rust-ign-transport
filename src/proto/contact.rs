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
pub struct Contact {
    // message fields
    collision1: ::protobuf::SingularField<::std::string::String>,
    collision2: ::protobuf::SingularField<::std::string::String>,
    position: ::protobuf::RepeatedField<super::vector3d::Vector3d>,
    normal: ::protobuf::RepeatedField<super::vector3d::Vector3d>,
    depth: ::std::vec::Vec<f64>,
    wrench: ::protobuf::RepeatedField<super::joint_wrench::JointWrench>,
    time: ::protobuf::SingularPtrField<super::time::Time>,
    world: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Contact {}

impl Contact {
    pub fn new() -> Contact {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Contact {
        static mut instance: ::protobuf::lazy::Lazy<Contact> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Contact,
        };
        unsafe {
            instance.get(|| {
                Contact {
                    collision1: ::protobuf::SingularField::none(),
                    collision2: ::protobuf::SingularField::none(),
                    position: ::protobuf::RepeatedField::new(),
                    normal: ::protobuf::RepeatedField::new(),
                    depth: ::std::vec::Vec::new(),
                    wrench: ::protobuf::RepeatedField::new(),
                    time: ::protobuf::SingularPtrField::none(),
                    world: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string collision1 = 1;

    pub fn clear_collision1(&mut self) {
        self.collision1.clear();
    }

    pub fn has_collision1(&self) -> bool {
        self.collision1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collision1(&mut self, v: ::std::string::String) {
        self.collision1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collision1(&mut self) -> &mut ::std::string::String {
        if self.collision1.is_none() {
            self.collision1.set_default();
        };
        self.collision1.as_mut().unwrap()
    }

    // Take field
    pub fn take_collision1(&mut self) -> ::std::string::String {
        self.collision1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_collision1(&self) -> &str {
        match self.collision1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string collision2 = 2;

    pub fn clear_collision2(&mut self) {
        self.collision2.clear();
    }

    pub fn has_collision2(&self) -> bool {
        self.collision2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collision2(&mut self, v: ::std::string::String) {
        self.collision2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collision2(&mut self) -> &mut ::std::string::String {
        if self.collision2.is_none() {
            self.collision2.set_default();
        };
        self.collision2.as_mut().unwrap()
    }

    // Take field
    pub fn take_collision2(&mut self) -> ::std::string::String {
        self.collision2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_collision2(&self) -> &str {
        match self.collision2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .gazebo.msgs.Vector3d position = 3;

    pub fn clear_position(&mut self) {
        self.position.clear();
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: ::protobuf::RepeatedField<super::vector3d::Vector3d>) {
        self.position = v;
    }

    // Mutable pointer to the field.
    pub fn mut_position(&mut self) -> &mut ::protobuf::RepeatedField<super::vector3d::Vector3d> {
        &mut self.position
    }

    // Take field
    pub fn take_position(&mut self) -> ::protobuf::RepeatedField<super::vector3d::Vector3d> {
        ::std::mem::replace(&mut self.position, ::protobuf::RepeatedField::new())
    }

    pub fn get_position(&self) -> &[super::vector3d::Vector3d] {
        &self.position
    }

    // repeated .gazebo.msgs.Vector3d normal = 4;

    pub fn clear_normal(&mut self) {
        self.normal.clear();
    }

    // Param is passed by value, moved
    pub fn set_normal(&mut self, v: ::protobuf::RepeatedField<super::vector3d::Vector3d>) {
        self.normal = v;
    }

    // Mutable pointer to the field.
    pub fn mut_normal(&mut self) -> &mut ::protobuf::RepeatedField<super::vector3d::Vector3d> {
        &mut self.normal
    }

    // Take field
    pub fn take_normal(&mut self) -> ::protobuf::RepeatedField<super::vector3d::Vector3d> {
        ::std::mem::replace(&mut self.normal, ::protobuf::RepeatedField::new())
    }

    pub fn get_normal(&self) -> &[super::vector3d::Vector3d] {
        &self.normal
    }

    // repeated double depth = 5;

    pub fn clear_depth(&mut self) {
        self.depth.clear();
    }

    // Param is passed by value, moved
    pub fn set_depth(&mut self, v: ::std::vec::Vec<f64>) {
        self.depth = v;
    }

    // Mutable pointer to the field.
    pub fn mut_depth(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.depth
    }

    // Take field
    pub fn take_depth(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.depth, ::std::vec::Vec::new())
    }

    pub fn get_depth(&self) -> &[f64] {
        &self.depth
    }

    // repeated .gazebo.msgs.JointWrench wrench = 6;

    pub fn clear_wrench(&mut self) {
        self.wrench.clear();
    }

    // Param is passed by value, moved
    pub fn set_wrench(&mut self, v: ::protobuf::RepeatedField<super::joint_wrench::JointWrench>) {
        self.wrench = v;
    }

    // Mutable pointer to the field.
    pub fn mut_wrench(&mut self) -> &mut ::protobuf::RepeatedField<super::joint_wrench::JointWrench> {
        &mut self.wrench
    }

    // Take field
    pub fn take_wrench(&mut self) -> ::protobuf::RepeatedField<super::joint_wrench::JointWrench> {
        ::std::mem::replace(&mut self.wrench, ::protobuf::RepeatedField::new())
    }

    pub fn get_wrench(&self) -> &[super::joint_wrench::JointWrench] {
        &self.wrench
    }

    // required .gazebo.msgs.Time time = 7;

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

    // required string world = 8;

    pub fn clear_world(&mut self) {
        self.world.clear();
    }

    pub fn has_world(&self) -> bool {
        self.world.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world(&mut self, v: ::std::string::String) {
        self.world = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world(&mut self) -> &mut ::std::string::String {
        if self.world.is_none() {
            self.world.set_default();
        };
        self.world.as_mut().unwrap()
    }

    // Take field
    pub fn take_world(&mut self) -> ::std::string::String {
        self.world.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_world(&self) -> &str {
        match self.world.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Contact {
    fn is_initialized(&self) -> bool {
        if self.collision1.is_none() {
            return false;
        };
        if self.collision2.is_none() {
            return false;
        };
        if self.time.is_none() {
            return false;
        };
        if self.world.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.collision1));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.collision2));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.position));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.normal));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.depth));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.wrench));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.time));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.world));
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
        for value in &self.collision1 {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.collision2 {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.position {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.normal {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += 9 * self.depth.len() as u32;
        for value in &self.wrench {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.world {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.collision1.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.collision2.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in &self.position {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.normal {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.depth {
            try!(os.write_double(5, *v));
        };
        for v in &self.wrench {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.time.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.world.as_ref() {
            try!(os.write_string(8, &v));
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
        ::std::any::TypeId::of::<Contact>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Contact {
    fn new() -> Contact {
        Contact::new()
    }

    fn descriptor_static(_: ::std::option::Option<Contact>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "collision1",
                    Contact::has_collision1,
                    Contact::get_collision1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "collision2",
                    Contact::has_collision2,
                    Contact::get_collision2,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "position",
                    Contact::get_position,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "normal",
                    Contact::get_normal,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "depth",
                    Contact::get_depth,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "wrench",
                    Contact::get_wrench,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "time",
                    Contact::has_time,
                    Contact::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "world",
                    Contact::has_world,
                    Contact::get_world,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Contact>(
                    "Contact",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Contact {
    fn clear(&mut self) {
        self.clear_collision1();
        self.clear_collision2();
        self.clear_position();
        self.clear_normal();
        self.clear_depth();
        self.clear_wrench();
        self.clear_time();
        self.clear_world();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Contact {
    fn eq(&self, other: &Contact) -> bool {
        self.collision1 == other.collision1 &&
        self.collision2 == other.collision2 &&
        self.position == other.position &&
        self.normal == other.normal &&
        self.depth == other.depth &&
        self.wrench == other.wrench &&
        self.time == other.time &&
        self.world == other.world &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Contact {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0e, 0x76, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x74, 0x69,
    0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x12, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x5f,
    0x77, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xea, 0x01, 0x0a,
    0x07, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6f, 0x6c, 0x6c,
    0x69, 0x73, 0x69, 0x6f, 0x6e, 0x31, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x12, 0x0a, 0x0a,
    0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x32, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x27, 0x0a, 0x08, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x12, 0x25, 0x0a, 0x06, 0x6e, 0x6f, 0x72,
    0x6d, 0x61, 0x6c, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65,
    0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64,
    0x12, 0x0d, 0x0a, 0x05, 0x64, 0x65, 0x70, 0x74, 0x68, 0x18, 0x05, 0x20, 0x03, 0x28, 0x01, 0x12,
    0x28, 0x0a, 0x06, 0x77, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x18, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4a, 0x6f,
    0x69, 0x6e, 0x74, 0x57, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x12, 0x1f, 0x0a, 0x04, 0x74, 0x69, 0x6d,
    0x65, 0x18, 0x07, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f,
    0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x77, 0x6f,
    0x72, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x02, 0x28, 0x09, 0x4a, 0xf3, 0x04, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x17, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x08, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09, 0x07, 0x1b, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0d, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0e, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x02, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x10, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x10, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x11, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x11, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x11, 0x0b, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x14, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x11, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x12, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x12,
    0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x12, 0x20, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x13, 0x02, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x13, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x13, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x13, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x15,
    0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x15, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x15, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x15, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x16, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04,
    0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03,
    0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x16, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x16, 0x1f, 0x20,
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
