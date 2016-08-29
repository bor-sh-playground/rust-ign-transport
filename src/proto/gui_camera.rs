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
pub struct GUICamera {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    view_controller: ::protobuf::SingularField<::std::string::String>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    track: ::protobuf::SingularPtrField<super::track_visual::TrackVisual>,
    projection_type: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GUICamera {}

impl GUICamera {
    pub fn new() -> GUICamera {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GUICamera {
        static mut instance: ::protobuf::lazy::Lazy<GUICamera> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GUICamera,
        };
        unsafe {
            instance.get(|| {
                GUICamera {
                    name: ::protobuf::SingularField::none(),
                    view_controller: ::protobuf::SingularField::none(),
                    pose: ::protobuf::SingularPtrField::none(),
                    track: ::protobuf::SingularPtrField::none(),
                    projection_type: ::protobuf::SingularField::none(),
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

    // optional string view_controller = 2;

    pub fn clear_view_controller(&mut self) {
        self.view_controller.clear();
    }

    pub fn has_view_controller(&self) -> bool {
        self.view_controller.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view_controller(&mut self, v: ::std::string::String) {
        self.view_controller = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_view_controller(&mut self) -> &mut ::std::string::String {
        if self.view_controller.is_none() {
            self.view_controller.set_default();
        };
        self.view_controller.as_mut().unwrap()
    }

    // Take field
    pub fn take_view_controller(&mut self) -> ::std::string::String {
        self.view_controller.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_view_controller(&self) -> &str {
        match self.view_controller.as_ref() {
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

    // optional .gazebo.msgs.TrackVisual track = 4;

    pub fn clear_track(&mut self) {
        self.track.clear();
    }

    pub fn has_track(&self) -> bool {
        self.track.is_some()
    }

    // Param is passed by value, moved
    pub fn set_track(&mut self, v: super::track_visual::TrackVisual) {
        self.track = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_track(&mut self) -> &mut super::track_visual::TrackVisual {
        if self.track.is_none() {
            self.track.set_default();
        };
        self.track.as_mut().unwrap()
    }

    // Take field
    pub fn take_track(&mut self) -> super::track_visual::TrackVisual {
        self.track.take().unwrap_or_else(|| super::track_visual::TrackVisual::new())
    }

    pub fn get_track(&self) -> &super::track_visual::TrackVisual {
        self.track.as_ref().unwrap_or_else(|| super::track_visual::TrackVisual::default_instance())
    }

    // optional string projection_type = 5;

    pub fn clear_projection_type(&mut self) {
        self.projection_type.clear();
    }

    pub fn has_projection_type(&self) -> bool {
        self.projection_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_projection_type(&mut self, v: ::std::string::String) {
        self.projection_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_projection_type(&mut self) -> &mut ::std::string::String {
        if self.projection_type.is_none() {
            self.projection_type.set_default();
        };
        self.projection_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_projection_type(&mut self) -> ::std::string::String {
        self.projection_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_projection_type(&self) -> &str {
        match self.projection_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GUICamera {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.view_controller));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.track));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.projection_type));
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
        for value in &self.view_controller {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.track {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.projection_type {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.view_controller.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.track.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.projection_type.as_ref() {
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
        ::std::any::TypeId::of::<GUICamera>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GUICamera {
    fn new() -> GUICamera {
        GUICamera::new()
    }

    fn descriptor_static(_: ::std::option::Option<GUICamera>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    GUICamera::has_name,
                    GUICamera::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "view_controller",
                    GUICamera::has_view_controller,
                    GUICamera::get_view_controller,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    GUICamera::has_pose,
                    GUICamera::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "track",
                    GUICamera::has_track,
                    GUICamera::get_track,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "projection_type",
                    GUICamera::has_projection_type,
                    GUICamera::get_projection_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GUICamera>(
                    "GUICamera",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GUICamera {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_view_controller();
        self.clear_pose();
        self.clear_track();
        self.clear_projection_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GUICamera {
    fn eq(&self, other: &GUICamera) -> bool {
        self.name == other.name &&
        self.view_controller == other.view_controller &&
        self.pose == other.pose &&
        self.track == other.track &&
        self.projection_type == other.projection_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GUICamera {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x67, 0x75, 0x69, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a,
    0x0a, 0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x12, 0x74, 0x72, 0x61,
    0x63, 0x6b, 0x5f, 0x76, 0x69, 0x73, 0x75, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x95, 0x01, 0x0a, 0x09, 0x47, 0x55, 0x49, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x12, 0x0c, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x17, 0x0a, 0x0f, 0x76,
    0x69, 0x65, 0x77, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x2e, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x56, 0x69, 0x73, 0x75, 0x61, 0x6c, 0x12, 0x17,
    0x0a, 0x0f, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x4a, 0xd9, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x14, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x08, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09,
    0x07, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x14, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x2a, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0e, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0e, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x02,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x10, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x10,
    0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x17, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10, 0x2a, 0x2b, 0x0a, 0x4b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x13, 0x02, 0x2c, 0x1a, 0x3e, 0x2f, 0x20, 0x5c,
    0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x54, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x72,
    0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x22, 0x70, 0x65, 0x72, 0x73, 0x70,
    0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x22, 0x20, 0x6f, 0x72, 0x20, 0x22, 0x6f, 0x72, 0x74, 0x68,
    0x6f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x22, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x13, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x13, 0x2a, 0x2b,
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
