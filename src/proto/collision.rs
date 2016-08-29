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
pub struct Collision {
    // message fields
    id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    laser_retro: ::std::option::Option<f64>,
    max_contacts: ::std::option::Option<f64>,
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    geometry: ::protobuf::SingularPtrField<super::geometry::Geometry>,
    surface: ::protobuf::SingularPtrField<super::surface::Surface>,
    visual: ::protobuf::RepeatedField<super::visual::Visual>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Collision {}

impl Collision {
    pub fn new() -> Collision {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Collision {
        static mut instance: ::protobuf::lazy::Lazy<Collision> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Collision,
        };
        unsafe {
            instance.get(|| {
                Collision {
                    id: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    laser_retro: ::std::option::Option::None,
                    max_contacts: ::std::option::Option::None,
                    pose: ::protobuf::SingularPtrField::none(),
                    geometry: ::protobuf::SingularPtrField::none(),
                    surface: ::protobuf::SingularPtrField::none(),
                    visual: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 id = 1;

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

    // optional double laser_retro = 3;

    pub fn clear_laser_retro(&mut self) {
        self.laser_retro = ::std::option::Option::None;
    }

    pub fn has_laser_retro(&self) -> bool {
        self.laser_retro.is_some()
    }

    // Param is passed by value, moved
    pub fn set_laser_retro(&mut self, v: f64) {
        self.laser_retro = ::std::option::Option::Some(v);
    }

    pub fn get_laser_retro(&self) -> f64 {
        self.laser_retro.unwrap_or(0.)
    }

    // optional double max_contacts = 4;

    pub fn clear_max_contacts(&mut self) {
        self.max_contacts = ::std::option::Option::None;
    }

    pub fn has_max_contacts(&self) -> bool {
        self.max_contacts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_contacts(&mut self, v: f64) {
        self.max_contacts = ::std::option::Option::Some(v);
    }

    pub fn get_max_contacts(&self) -> f64 {
        self.max_contacts.unwrap_or(0.)
    }

    // optional .gazebo.msgs.Pose pose = 5;

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

    // optional .gazebo.msgs.Geometry geometry = 6;

    pub fn clear_geometry(&mut self) {
        self.geometry.clear();
    }

    pub fn has_geometry(&self) -> bool {
        self.geometry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_geometry(&mut self, v: super::geometry::Geometry) {
        self.geometry = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_geometry(&mut self) -> &mut super::geometry::Geometry {
        if self.geometry.is_none() {
            self.geometry.set_default();
        };
        self.geometry.as_mut().unwrap()
    }

    // Take field
    pub fn take_geometry(&mut self) -> super::geometry::Geometry {
        self.geometry.take().unwrap_or_else(|| super::geometry::Geometry::new())
    }

    pub fn get_geometry(&self) -> &super::geometry::Geometry {
        self.geometry.as_ref().unwrap_or_else(|| super::geometry::Geometry::default_instance())
    }

    // optional .gazebo.msgs.Surface surface = 7;

    pub fn clear_surface(&mut self) {
        self.surface.clear();
    }

    pub fn has_surface(&self) -> bool {
        self.surface.is_some()
    }

    // Param is passed by value, moved
    pub fn set_surface(&mut self, v: super::surface::Surface) {
        self.surface = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_surface(&mut self) -> &mut super::surface::Surface {
        if self.surface.is_none() {
            self.surface.set_default();
        };
        self.surface.as_mut().unwrap()
    }

    // Take field
    pub fn take_surface(&mut self) -> super::surface::Surface {
        self.surface.take().unwrap_or_else(|| super::surface::Surface::new())
    }

    pub fn get_surface(&self) -> &super::surface::Surface {
        self.surface.as_ref().unwrap_or_else(|| super::surface::Surface::default_instance())
    }

    // repeated .gazebo.msgs.Visual visual = 8;

    pub fn clear_visual(&mut self) {
        self.visual.clear();
    }

    // Param is passed by value, moved
    pub fn set_visual(&mut self, v: ::protobuf::RepeatedField<super::visual::Visual>) {
        self.visual = v;
    }

    // Mutable pointer to the field.
    pub fn mut_visual(&mut self) -> &mut ::protobuf::RepeatedField<super::visual::Visual> {
        &mut self.visual
    }

    // Take field
    pub fn take_visual(&mut self) -> ::protobuf::RepeatedField<super::visual::Visual> {
        ::std::mem::replace(&mut self.visual, ::protobuf::RepeatedField::new())
    }

    pub fn get_visual(&self) -> &[super::visual::Visual] {
        &self.visual
    }
}

impl ::protobuf::Message for Collision {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.laser_retro = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.max_contacts = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.geometry));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.surface));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.visual));
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.laser_retro.is_some() {
            my_size += 9;
        };
        if self.max_contacts.is_some() {
            my_size += 9;
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.geometry {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.surface {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.visual {
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
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.laser_retro {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.max_contacts {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.geometry.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.surface.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.visual {
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
        ::std::any::TypeId::of::<Collision>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Collision {
    fn new() -> Collision {
        Collision::new()
    }

    fn descriptor_static(_: ::std::option::Option<Collision>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    Collision::has_id,
                    Collision::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Collision::has_name,
                    Collision::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "laser_retro",
                    Collision::has_laser_retro,
                    Collision::get_laser_retro,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "max_contacts",
                    Collision::has_max_contacts,
                    Collision::get_max_contacts,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Collision::has_pose,
                    Collision::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "geometry",
                    Collision::has_geometry,
                    Collision::get_geometry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "surface",
                    Collision::has_surface,
                    Collision::get_surface,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "visual",
                    Collision::get_visual,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Collision>(
                    "Collision",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Collision {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_laser_retro();
        self.clear_max_contacts();
        self.clear_pose();
        self.clear_geometry();
        self.clear_surface();
        self.clear_visual();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Collision {
    fn eq(&self, other: &Collision) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.laser_retro == other.laser_retro &&
        self.max_contacts == other.max_contacts &&
        self.pose == other.pose &&
        self.geometry == other.geometry &&
        self.surface == other.surface &&
        self.visual == other.visual &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Collision {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a,
    0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x67, 0x65, 0x6f, 0x6d,
    0x65, 0x74, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0d, 0x73, 0x75, 0x72, 0x66,
    0x61, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x76, 0x69, 0x73, 0x75, 0x61,
    0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xe6, 0x01, 0x0a, 0x09, 0x43, 0x6f, 0x6c, 0x6c,
    0x69, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x13, 0x0a, 0x0b, 0x6c, 0x61, 0x73, 0x65, 0x72, 0x5f, 0x72, 0x65, 0x74, 0x72, 0x6f, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x01, 0x12, 0x14, 0x0a, 0x0c, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x63, 0x74, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f,
    0x73, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62,
    0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x08, 0x67,
    0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x47, 0x65, 0x6f, 0x6d,
    0x65, 0x74, 0x72, 0x79, 0x12, 0x25, 0x0a, 0x07, 0x73, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x53, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x12, 0x23, 0x0a, 0x06, 0x76,
    0x69, 0x73, 0x75, 0x61, 0x6c, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x69, 0x73, 0x75, 0x61, 0x6c,
    0x4a, 0xfe, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x17, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x13,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x09, 0x07, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0a, 0x07,
    0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0e, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0f, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0f, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x02, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x11, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x11, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x12, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x11, 0x22, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x12, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x06, 0x12, 0x03, 0x12, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x12, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x12,
    0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x13, 0x02, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x13, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x13, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x13, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12,
    0x03, 0x14, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x14,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x14, 0x0b, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x14, 0x13, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x14, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x16, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06,
    0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x16, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x16, 0x22,
    0x23,
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
