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
pub struct JointWrench {
    // message fields
    body_1_name: ::protobuf::SingularField<::std::string::String>,
    body_1_id: ::std::option::Option<u32>,
    body_2_name: ::protobuf::SingularField<::std::string::String>,
    body_2_id: ::std::option::Option<u32>,
    body_1_wrench: ::protobuf::SingularPtrField<super::wrench::Wrench>,
    body_2_wrench: ::protobuf::SingularPtrField<super::wrench::Wrench>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JointWrench {}

impl JointWrench {
    pub fn new() -> JointWrench {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JointWrench {
        static mut instance: ::protobuf::lazy::Lazy<JointWrench> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JointWrench,
        };
        unsafe {
            instance.get(|| {
                JointWrench {
                    body_1_name: ::protobuf::SingularField::none(),
                    body_1_id: ::std::option::Option::None,
                    body_2_name: ::protobuf::SingularField::none(),
                    body_2_id: ::std::option::Option::None,
                    body_1_wrench: ::protobuf::SingularPtrField::none(),
                    body_2_wrench: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string body_1_name = 1;

    pub fn clear_body_1_name(&mut self) {
        self.body_1_name.clear();
    }

    pub fn has_body_1_name(&self) -> bool {
        self.body_1_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body_1_name(&mut self, v: ::std::string::String) {
        self.body_1_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body_1_name(&mut self) -> &mut ::std::string::String {
        if self.body_1_name.is_none() {
            self.body_1_name.set_default();
        };
        self.body_1_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_body_1_name(&mut self) -> ::std::string::String {
        self.body_1_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_body_1_name(&self) -> &str {
        match self.body_1_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint32 body_1_id = 2;

    pub fn clear_body_1_id(&mut self) {
        self.body_1_id = ::std::option::Option::None;
    }

    pub fn has_body_1_id(&self) -> bool {
        self.body_1_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body_1_id(&mut self, v: u32) {
        self.body_1_id = ::std::option::Option::Some(v);
    }

    pub fn get_body_1_id(&self) -> u32 {
        self.body_1_id.unwrap_or(0)
    }

    // required string body_2_name = 3;

    pub fn clear_body_2_name(&mut self) {
        self.body_2_name.clear();
    }

    pub fn has_body_2_name(&self) -> bool {
        self.body_2_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body_2_name(&mut self, v: ::std::string::String) {
        self.body_2_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body_2_name(&mut self) -> &mut ::std::string::String {
        if self.body_2_name.is_none() {
            self.body_2_name.set_default();
        };
        self.body_2_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_body_2_name(&mut self) -> ::std::string::String {
        self.body_2_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_body_2_name(&self) -> &str {
        match self.body_2_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint32 body_2_id = 4;

    pub fn clear_body_2_id(&mut self) {
        self.body_2_id = ::std::option::Option::None;
    }

    pub fn has_body_2_id(&self) -> bool {
        self.body_2_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body_2_id(&mut self, v: u32) {
        self.body_2_id = ::std::option::Option::Some(v);
    }

    pub fn get_body_2_id(&self) -> u32 {
        self.body_2_id.unwrap_or(0)
    }

    // required .gazebo.msgs.Wrench body_1_wrench = 5;

    pub fn clear_body_1_wrench(&mut self) {
        self.body_1_wrench.clear();
    }

    pub fn has_body_1_wrench(&self) -> bool {
        self.body_1_wrench.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body_1_wrench(&mut self, v: super::wrench::Wrench) {
        self.body_1_wrench = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body_1_wrench(&mut self) -> &mut super::wrench::Wrench {
        if self.body_1_wrench.is_none() {
            self.body_1_wrench.set_default();
        };
        self.body_1_wrench.as_mut().unwrap()
    }

    // Take field
    pub fn take_body_1_wrench(&mut self) -> super::wrench::Wrench {
        self.body_1_wrench.take().unwrap_or_else(|| super::wrench::Wrench::new())
    }

    pub fn get_body_1_wrench(&self) -> &super::wrench::Wrench {
        self.body_1_wrench.as_ref().unwrap_or_else(|| super::wrench::Wrench::default_instance())
    }

    // required .gazebo.msgs.Wrench body_2_wrench = 6;

    pub fn clear_body_2_wrench(&mut self) {
        self.body_2_wrench.clear();
    }

    pub fn has_body_2_wrench(&self) -> bool {
        self.body_2_wrench.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body_2_wrench(&mut self, v: super::wrench::Wrench) {
        self.body_2_wrench = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body_2_wrench(&mut self) -> &mut super::wrench::Wrench {
        if self.body_2_wrench.is_none() {
            self.body_2_wrench.set_default();
        };
        self.body_2_wrench.as_mut().unwrap()
    }

    // Take field
    pub fn take_body_2_wrench(&mut self) -> super::wrench::Wrench {
        self.body_2_wrench.take().unwrap_or_else(|| super::wrench::Wrench::new())
    }

    pub fn get_body_2_wrench(&self) -> &super::wrench::Wrench {
        self.body_2_wrench.as_ref().unwrap_or_else(|| super::wrench::Wrench::default_instance())
    }
}

impl ::protobuf::Message for JointWrench {
    fn is_initialized(&self) -> bool {
        if self.body_1_name.is_none() {
            return false;
        };
        if self.body_1_id.is_none() {
            return false;
        };
        if self.body_2_name.is_none() {
            return false;
        };
        if self.body_2_id.is_none() {
            return false;
        };
        if self.body_1_wrench.is_none() {
            return false;
        };
        if self.body_2_wrench.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.body_1_name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.body_1_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.body_2_name));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.body_2_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.body_1_wrench));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.body_2_wrench));
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
        for value in &self.body_1_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.body_1_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.body_2_name {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.body_2_id {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.body_1_wrench {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.body_2_wrench {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.body_1_name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.body_1_id {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.body_2_name.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.body_2_id {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.body_1_wrench.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.body_2_wrench.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<JointWrench>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for JointWrench {
    fn new() -> JointWrench {
        JointWrench::new()
    }

    fn descriptor_static(_: ::std::option::Option<JointWrench>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "body_1_name",
                    JointWrench::has_body_1_name,
                    JointWrench::get_body_1_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "body_1_id",
                    JointWrench::has_body_1_id,
                    JointWrench::get_body_1_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "body_2_name",
                    JointWrench::has_body_2_name,
                    JointWrench::get_body_2_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "body_2_id",
                    JointWrench::has_body_2_id,
                    JointWrench::get_body_2_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "body_1_wrench",
                    JointWrench::has_body_1_wrench,
                    JointWrench::get_body_1_wrench,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "body_2_wrench",
                    JointWrench::has_body_2_wrench,
                    JointWrench::get_body_2_wrench,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JointWrench>(
                    "JointWrench",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JointWrench {
    fn clear(&mut self) {
        self.clear_body_1_name();
        self.clear_body_1_id();
        self.clear_body_2_name();
        self.clear_body_2_id();
        self.clear_body_1_wrench();
        self.clear_body_2_wrench();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for JointWrench {
    fn eq(&self, other: &JointWrench) -> bool {
        self.body_1_name == other.body_1_name &&
        self.body_1_id == other.body_1_id &&
        self.body_2_name == other.body_2_name &&
        self.body_2_id == other.body_2_id &&
        self.body_1_wrench == other.body_1_wrench &&
        self.body_2_wrench == other.body_2_wrench &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for JointWrench {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x77, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x1a, 0x0c, 0x77, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0xb5, 0x01, 0x0a, 0x0b, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x57, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x12,
    0x13, 0x0a, 0x0b, 0x62, 0x6f, 0x64, 0x79, 0x5f, 0x31, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x62, 0x6f, 0x64, 0x79, 0x5f, 0x31, 0x5f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x62, 0x6f, 0x64, 0x79, 0x5f,
    0x32, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09,
    0x62, 0x6f, 0x64, 0x79, 0x5f, 0x32, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0d, 0x12,
    0x2a, 0x0a, 0x0d, 0x62, 0x6f, 0x64, 0x79, 0x5f, 0x31, 0x5f, 0x77, 0x72, 0x65, 0x6e, 0x63, 0x68,
    0x18, 0x05, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e,
    0x6d, 0x73, 0x67, 0x73, 0x2e, 0x57, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x12, 0x2a, 0x0a, 0x0d, 0x62,
    0x6f, 0x64, 0x79, 0x5f, 0x32, 0x5f, 0x77, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x18, 0x06, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x57, 0x72, 0x65, 0x6e, 0x63, 0x68, 0x4a, 0xd3, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x11, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08,
    0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0a, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0a, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x0c, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0c,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x0d, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x0d, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x10, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x10, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x12, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x22, 0x23,
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
