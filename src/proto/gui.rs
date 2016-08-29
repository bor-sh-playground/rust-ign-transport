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
pub struct GUI {
    // message fields
    fullscreen: ::std::option::Option<bool>,
    camera: ::protobuf::SingularPtrField<super::gui_camera::GUICamera>,
    plugin: ::protobuf::RepeatedField<super::plugin::Plugin>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GUI {}

impl GUI {
    pub fn new() -> GUI {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GUI {
        static mut instance: ::protobuf::lazy::Lazy<GUI> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GUI,
        };
        unsafe {
            instance.get(|| {
                GUI {
                    fullscreen: ::std::option::Option::None,
                    camera: ::protobuf::SingularPtrField::none(),
                    plugin: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool fullscreen = 1;

    pub fn clear_fullscreen(&mut self) {
        self.fullscreen = ::std::option::Option::None;
    }

    pub fn has_fullscreen(&self) -> bool {
        self.fullscreen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fullscreen(&mut self, v: bool) {
        self.fullscreen = ::std::option::Option::Some(v);
    }

    pub fn get_fullscreen(&self) -> bool {
        self.fullscreen.unwrap_or(false)
    }

    // optional .gazebo.msgs.GUICamera camera = 2;

    pub fn clear_camera(&mut self) {
        self.camera.clear();
    }

    pub fn has_camera(&self) -> bool {
        self.camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera(&mut self, v: super::gui_camera::GUICamera) {
        self.camera = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_camera(&mut self) -> &mut super::gui_camera::GUICamera {
        if self.camera.is_none() {
            self.camera.set_default();
        };
        self.camera.as_mut().unwrap()
    }

    // Take field
    pub fn take_camera(&mut self) -> super::gui_camera::GUICamera {
        self.camera.take().unwrap_or_else(|| super::gui_camera::GUICamera::new())
    }

    pub fn get_camera(&self) -> &super::gui_camera::GUICamera {
        self.camera.as_ref().unwrap_or_else(|| super::gui_camera::GUICamera::default_instance())
    }

    // repeated .gazebo.msgs.Plugin plugin = 3;

    pub fn clear_plugin(&mut self) {
        self.plugin.clear();
    }

    // Param is passed by value, moved
    pub fn set_plugin(&mut self, v: ::protobuf::RepeatedField<super::plugin::Plugin>) {
        self.plugin = v;
    }

    // Mutable pointer to the field.
    pub fn mut_plugin(&mut self) -> &mut ::protobuf::RepeatedField<super::plugin::Plugin> {
        &mut self.plugin
    }

    // Take field
    pub fn take_plugin(&mut self) -> ::protobuf::RepeatedField<super::plugin::Plugin> {
        ::std::mem::replace(&mut self.plugin, ::protobuf::RepeatedField::new())
    }

    pub fn get_plugin(&self) -> &[super::plugin::Plugin] {
        &self.plugin
    }
}

impl ::protobuf::Message for GUI {
    fn is_initialized(&self) -> bool {
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
                    let tmp = try!(is.read_bool());
                    self.fullscreen = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.camera));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.plugin));
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
        if self.fullscreen.is_some() {
            my_size += 2;
        };
        for value in &self.camera {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.plugin {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fullscreen {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.camera.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.plugin {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<GUI>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GUI {
    fn new() -> GUI {
        GUI::new()
    }

    fn descriptor_static(_: ::std::option::Option<GUI>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "fullscreen",
                    GUI::has_fullscreen,
                    GUI::get_fullscreen,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "camera",
                    GUI::has_camera,
                    GUI::get_camera,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "plugin",
                    GUI::get_plugin,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GUI>(
                    "GUI",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GUI {
    fn clear(&mut self) {
        self.clear_fullscreen();
        self.clear_camera();
        self.clear_plugin();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GUI {
    fn eq(&self, other: &GUI) -> bool {
        self.fullscreen == other.fullscreen &&
        self.camera == other.camera &&
        self.plugin == other.plugin &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GUI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x67, 0x75, 0x69, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x10, 0x67, 0x75, 0x69, 0x5f, 0x63, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x70, 0x6c, 0x75, 0x67,
    0x69, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x66, 0x0a, 0x03, 0x47, 0x55, 0x49, 0x12,
    0x12, 0x0a, 0x0a, 0x66, 0x75, 0x6c, 0x6c, 0x73, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x26, 0x0a, 0x06, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x2e, 0x47, 0x55, 0x49, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x12, 0x23, 0x0a, 0x06, 0x70,
    0x6c, 0x75, 0x67, 0x69, 0x6e, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e,
    0x4a, 0x8f, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x19,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x07, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x09, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x0c, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x0b,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x15, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x0d, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d,
    0x1e, 0x1f,
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
