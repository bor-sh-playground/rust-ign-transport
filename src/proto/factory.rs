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
pub struct Factory {
    // message fields
    sdf: ::protobuf::SingularField<::std::string::String>,
    sdf_filename: ::protobuf::SingularField<::std::string::String>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    edit_name: ::protobuf::SingularField<::std::string::String>,
    clone_model_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Factory {}

impl Factory {
    pub fn new() -> Factory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Factory {
        static mut instance: ::protobuf::lazy::Lazy<Factory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Factory,
        };
        unsafe {
            instance.get(|| {
                Factory {
                    sdf: ::protobuf::SingularField::none(),
                    sdf_filename: ::protobuf::SingularField::none(),
                    pose: ::protobuf::SingularPtrField::none(),
                    edit_name: ::protobuf::SingularField::none(),
                    clone_model_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string sdf = 1;

    pub fn clear_sdf(&mut self) {
        self.sdf.clear();
    }

    pub fn has_sdf(&self) -> bool {
        self.sdf.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sdf(&mut self, v: ::std::string::String) {
        self.sdf = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sdf(&mut self) -> &mut ::std::string::String {
        if self.sdf.is_none() {
            self.sdf.set_default();
        };
        self.sdf.as_mut().unwrap()
    }

    // Take field
    pub fn take_sdf(&mut self) -> ::std::string::String {
        self.sdf.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sdf(&self) -> &str {
        match self.sdf.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string sdf_filename = 2;

    pub fn clear_sdf_filename(&mut self) {
        self.sdf_filename.clear();
    }

    pub fn has_sdf_filename(&self) -> bool {
        self.sdf_filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sdf_filename(&mut self, v: ::std::string::String) {
        self.sdf_filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sdf_filename(&mut self) -> &mut ::std::string::String {
        if self.sdf_filename.is_none() {
            self.sdf_filename.set_default();
        };
        self.sdf_filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_sdf_filename(&mut self) -> ::std::string::String {
        self.sdf_filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sdf_filename(&self) -> &str {
        match self.sdf_filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
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

    // optional string edit_name = 4;

    pub fn clear_edit_name(&mut self) {
        self.edit_name.clear();
    }

    pub fn has_edit_name(&self) -> bool {
        self.edit_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_edit_name(&mut self, v: ::std::string::String) {
        self.edit_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_edit_name(&mut self) -> &mut ::std::string::String {
        if self.edit_name.is_none() {
            self.edit_name.set_default();
        };
        self.edit_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_edit_name(&mut self) -> ::std::string::String {
        self.edit_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_edit_name(&self) -> &str {
        match self.edit_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string clone_model_name = 5;

    pub fn clear_clone_model_name(&mut self) {
        self.clone_model_name.clear();
    }

    pub fn has_clone_model_name(&self) -> bool {
        self.clone_model_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clone_model_name(&mut self, v: ::std::string::String) {
        self.clone_model_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clone_model_name(&mut self) -> &mut ::std::string::String {
        if self.clone_model_name.is_none() {
            self.clone_model_name.set_default();
        };
        self.clone_model_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_clone_model_name(&mut self) -> ::std::string::String {
        self.clone_model_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clone_model_name(&self) -> &str {
        match self.clone_model_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Factory {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sdf));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sdf_filename));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.edit_name));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clone_model_name));
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
        for value in &self.sdf {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.sdf_filename {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.edit_name {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.clone_model_name {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sdf.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.sdf_filename.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.edit_name.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.clone_model_name.as_ref() {
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
        ::std::any::TypeId::of::<Factory>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Factory {
    fn new() -> Factory {
        Factory::new()
    }

    fn descriptor_static(_: ::std::option::Option<Factory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sdf",
                    Factory::has_sdf,
                    Factory::get_sdf,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sdf_filename",
                    Factory::has_sdf_filename,
                    Factory::get_sdf_filename,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Factory::has_pose,
                    Factory::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "edit_name",
                    Factory::has_edit_name,
                    Factory::get_edit_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "clone_model_name",
                    Factory::has_clone_model_name,
                    Factory::get_clone_model_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Factory>(
                    "Factory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Factory {
    fn clear(&mut self) {
        self.clear_sdf();
        self.clear_sdf_filename();
        self.clear_pose();
        self.clear_edit_name();
        self.clear_clone_model_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Factory {
    fn eq(&self, other: &Factory) -> bool {
        self.sdf == other.sdf &&
        self.sdf_filename == other.sdf_filename &&
        self.pose == other.pose &&
        self.edit_name == other.edit_name &&
        self.clone_model_name == other.clone_model_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Factory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x70, 0x6f,
    0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x7a, 0x0a, 0x07, 0x46, 0x61, 0x63, 0x74,
    0x6f, 0x72, 0x79, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x64, 0x66, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x14, 0x0a, 0x0c, 0x73, 0x64, 0x66, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x65, 0x64, 0x69, 0x74, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12, 0x18, 0x0a, 0x10, 0x63, 0x6c,
    0x6f, 0x6e, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x09, 0x4a, 0xe7, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x25, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x13, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x15, 0x00, 0x25, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0f, 0x0a, 0x38, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x02, 0x30, 0x1a, 0x2b, 0x2f, 0x20, 0x5c, 0x62, 0x72,
    0x69, 0x65, 0x66, 0x20, 0x53, 0x44, 0x46, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x12, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x2e, 0x2f, 0x0a, 0x2d,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x02, 0x30, 0x1a, 0x20, 0x2f, 0x20, 0x5c,
    0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x46, 0x75, 0x6c, 0x6c, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20,
    0x74, 0x6f, 0x20, 0x53, 0x44, 0x46, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x1b, 0x2e, 0x2f, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1e,
    0x02, 0x30, 0x1a, 0x30, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x50, 0x6f, 0x73,
    0x65, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73, 0x70, 0x61, 0x77, 0x6e,
    0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1e, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x10, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x2e, 0x2f, 0x0a, 0x40, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x21, 0x02, 0x30, 0x1a, 0x33, 0x2f, 0x20, 0x5c, 0x62, 0x72,
    0x69, 0x65, 0x66, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x21, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x21, 0x2e, 0x2f, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03,
    0x24, 0x02, 0x30, 0x1a, 0x21, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4e, 0x61,
    0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x63,
    0x6c, 0x6f, 0x6e, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x24,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x24, 0x12, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x24, 0x2e, 0x2f,
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
