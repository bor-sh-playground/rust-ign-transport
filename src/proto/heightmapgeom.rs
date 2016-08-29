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
pub struct HeightmapGeom {
    // message fields
    image: ::protobuf::SingularPtrField<super::image::Image>,
    size: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    origin: ::protobuf::SingularPtrField<super::vector3d::Vector3d>,
    heights: ::std::vec::Vec<f32>,
    width: ::std::option::Option<i32>,
    height: ::std::option::Option<i32>,
    texture: ::protobuf::RepeatedField<HeightmapGeom_Texture>,
    blend: ::protobuf::RepeatedField<HeightmapGeom_Blend>,
    use_terrain_paging: ::std::option::Option<bool>,
    filename: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeightmapGeom {}

impl HeightmapGeom {
    pub fn new() -> HeightmapGeom {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeightmapGeom {
        static mut instance: ::protobuf::lazy::Lazy<HeightmapGeom> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeightmapGeom,
        };
        unsafe {
            instance.get(|| {
                HeightmapGeom {
                    image: ::protobuf::SingularPtrField::none(),
                    size: ::protobuf::SingularPtrField::none(),
                    origin: ::protobuf::SingularPtrField::none(),
                    heights: ::std::vec::Vec::new(),
                    width: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    texture: ::protobuf::RepeatedField::new(),
                    blend: ::protobuf::RepeatedField::new(),
                    use_terrain_paging: ::std::option::Option::None,
                    filename: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gazebo.msgs.Image image = 1;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: super::image::Image) {
        self.image = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image(&mut self) -> &mut super::image::Image {
        if self.image.is_none() {
            self.image.set_default();
        };
        self.image.as_mut().unwrap()
    }

    // Take field
    pub fn take_image(&mut self) -> super::image::Image {
        self.image.take().unwrap_or_else(|| super::image::Image::new())
    }

    pub fn get_image(&self) -> &super::image::Image {
        self.image.as_ref().unwrap_or_else(|| super::image::Image::default_instance())
    }

    // required .gazebo.msgs.Vector3d size = 2;

    pub fn clear_size(&mut self) {
        self.size.clear();
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: super::vector3d::Vector3d) {
        self.size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_size(&mut self) -> &mut super::vector3d::Vector3d {
        if self.size.is_none() {
            self.size.set_default();
        };
        self.size.as_mut().unwrap()
    }

    // Take field
    pub fn take_size(&mut self) -> super::vector3d::Vector3d {
        self.size.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_size(&self) -> &super::vector3d::Vector3d {
        self.size.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // optional .gazebo.msgs.Vector3d origin = 3;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: super::vector3d::Vector3d) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut super::vector3d::Vector3d {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> super::vector3d::Vector3d {
        self.origin.take().unwrap_or_else(|| super::vector3d::Vector3d::new())
    }

    pub fn get_origin(&self) -> &super::vector3d::Vector3d {
        self.origin.as_ref().unwrap_or_else(|| super::vector3d::Vector3d::default_instance())
    }

    // repeated float heights = 4;

    pub fn clear_heights(&mut self) {
        self.heights.clear();
    }

    // Param is passed by value, moved
    pub fn set_heights(&mut self, v: ::std::vec::Vec<f32>) {
        self.heights = v;
    }

    // Mutable pointer to the field.
    pub fn mut_heights(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.heights
    }

    // Take field
    pub fn take_heights(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.heights, ::std::vec::Vec::new())
    }

    pub fn get_heights(&self) -> &[f32] {
        &self.heights
    }

    // optional int32 width = 5;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: i32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> i32 {
        self.width.unwrap_or(0)
    }

    // optional int32 height = 6;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> i32 {
        self.height.unwrap_or(0)
    }

    // repeated .gazebo.msgs.HeightmapGeom.Texture texture = 7;

    pub fn clear_texture(&mut self) {
        self.texture.clear();
    }

    // Param is passed by value, moved
    pub fn set_texture(&mut self, v: ::protobuf::RepeatedField<HeightmapGeom_Texture>) {
        self.texture = v;
    }

    // Mutable pointer to the field.
    pub fn mut_texture(&mut self) -> &mut ::protobuf::RepeatedField<HeightmapGeom_Texture> {
        &mut self.texture
    }

    // Take field
    pub fn take_texture(&mut self) -> ::protobuf::RepeatedField<HeightmapGeom_Texture> {
        ::std::mem::replace(&mut self.texture, ::protobuf::RepeatedField::new())
    }

    pub fn get_texture(&self) -> &[HeightmapGeom_Texture] {
        &self.texture
    }

    // repeated .gazebo.msgs.HeightmapGeom.Blend blend = 8;

    pub fn clear_blend(&mut self) {
        self.blend.clear();
    }

    // Param is passed by value, moved
    pub fn set_blend(&mut self, v: ::protobuf::RepeatedField<HeightmapGeom_Blend>) {
        self.blend = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blend(&mut self) -> &mut ::protobuf::RepeatedField<HeightmapGeom_Blend> {
        &mut self.blend
    }

    // Take field
    pub fn take_blend(&mut self) -> ::protobuf::RepeatedField<HeightmapGeom_Blend> {
        ::std::mem::replace(&mut self.blend, ::protobuf::RepeatedField::new())
    }

    pub fn get_blend(&self) -> &[HeightmapGeom_Blend] {
        &self.blend
    }

    // optional bool use_terrain_paging = 9;

    pub fn clear_use_terrain_paging(&mut self) {
        self.use_terrain_paging = ::std::option::Option::None;
    }

    pub fn has_use_terrain_paging(&self) -> bool {
        self.use_terrain_paging.is_some()
    }

    // Param is passed by value, moved
    pub fn set_use_terrain_paging(&mut self, v: bool) {
        self.use_terrain_paging = ::std::option::Option::Some(v);
    }

    pub fn get_use_terrain_paging(&self) -> bool {
        self.use_terrain_paging.unwrap_or(false)
    }

    // optional string filename = 10;

    pub fn clear_filename(&mut self) {
        self.filename.clear();
    }

    pub fn has_filename(&self) -> bool {
        self.filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        if self.filename.is_none() {
            self.filename.set_default();
        };
        self.filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        self.filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_filename(&self) -> &str {
        match self.filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for HeightmapGeom {
    fn is_initialized(&self) -> bool {
        if self.size.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.image));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.size));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.heights));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.width = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.height = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.texture));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blend));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.use_terrain_paging = ::std::option::Option::Some(tmp);
                },
                10 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename));
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
        for value in &self.image {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.size {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.origin {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += 5 * self.heights.len() as u32;
        for value in &self.width {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.height {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.texture {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.blend {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.use_terrain_paging.is_some() {
            my_size += 2;
        };
        for value in &self.filename {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.image.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.size.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.heights {
            try!(os.write_float(4, *v));
        };
        if let Some(v) = self.width {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.height {
            try!(os.write_int32(6, v));
        };
        for v in &self.texture {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.blend {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.use_terrain_paging {
            try!(os.write_bool(9, v));
        };
        if let Some(v) = self.filename.as_ref() {
            try!(os.write_string(10, &v));
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
        ::std::any::TypeId::of::<HeightmapGeom>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HeightmapGeom {
    fn new() -> HeightmapGeom {
        HeightmapGeom::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeightmapGeom>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "image",
                    HeightmapGeom::has_image,
                    HeightmapGeom::get_image,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "size",
                    HeightmapGeom::has_size,
                    HeightmapGeom::get_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin",
                    HeightmapGeom::has_origin,
                    HeightmapGeom::get_origin,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "heights",
                    HeightmapGeom::get_heights,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "width",
                    HeightmapGeom::has_width,
                    HeightmapGeom::get_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "height",
                    HeightmapGeom::has_height,
                    HeightmapGeom::get_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "texture",
                    HeightmapGeom::get_texture,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "blend",
                    HeightmapGeom::get_blend,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "use_terrain_paging",
                    HeightmapGeom::has_use_terrain_paging,
                    HeightmapGeom::get_use_terrain_paging,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "filename",
                    HeightmapGeom::has_filename,
                    HeightmapGeom::get_filename,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeightmapGeom>(
                    "HeightmapGeom",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeightmapGeom {
    fn clear(&mut self) {
        self.clear_image();
        self.clear_size();
        self.clear_origin();
        self.clear_heights();
        self.clear_width();
        self.clear_height();
        self.clear_texture();
        self.clear_blend();
        self.clear_use_terrain_paging();
        self.clear_filename();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HeightmapGeom {
    fn eq(&self, other: &HeightmapGeom) -> bool {
        self.image == other.image &&
        self.size == other.size &&
        self.origin == other.origin &&
        self.heights == other.heights &&
        self.width == other.width &&
        self.height == other.height &&
        self.texture == other.texture &&
        self.blend == other.blend &&
        self.use_terrain_paging == other.use_terrain_paging &&
        self.filename == other.filename &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HeightmapGeom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct HeightmapGeom_Texture {
    // message fields
    diffuse: ::protobuf::SingularField<::std::string::String>,
    normal: ::protobuf::SingularField<::std::string::String>,
    size: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeightmapGeom_Texture {}

impl HeightmapGeom_Texture {
    pub fn new() -> HeightmapGeom_Texture {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeightmapGeom_Texture {
        static mut instance: ::protobuf::lazy::Lazy<HeightmapGeom_Texture> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeightmapGeom_Texture,
        };
        unsafe {
            instance.get(|| {
                HeightmapGeom_Texture {
                    diffuse: ::protobuf::SingularField::none(),
                    normal: ::protobuf::SingularField::none(),
                    size: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string diffuse = 1;

    pub fn clear_diffuse(&mut self) {
        self.diffuse.clear();
    }

    pub fn has_diffuse(&self) -> bool {
        self.diffuse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diffuse(&mut self, v: ::std::string::String) {
        self.diffuse = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diffuse(&mut self) -> &mut ::std::string::String {
        if self.diffuse.is_none() {
            self.diffuse.set_default();
        };
        self.diffuse.as_mut().unwrap()
    }

    // Take field
    pub fn take_diffuse(&mut self) -> ::std::string::String {
        self.diffuse.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_diffuse(&self) -> &str {
        match self.diffuse.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string normal = 2;

    pub fn clear_normal(&mut self) {
        self.normal.clear();
    }

    pub fn has_normal(&self) -> bool {
        self.normal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normal(&mut self, v: ::std::string::String) {
        self.normal = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_normal(&mut self) -> &mut ::std::string::String {
        if self.normal.is_none() {
            self.normal.set_default();
        };
        self.normal.as_mut().unwrap()
    }

    // Take field
    pub fn take_normal(&mut self) -> ::std::string::String {
        self.normal.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_normal(&self) -> &str {
        match self.normal.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required double size = 3;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: f64) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> f64 {
        self.size.unwrap_or(0.)
    }
}

impl ::protobuf::Message for HeightmapGeom_Texture {
    fn is_initialized(&self) -> bool {
        if self.diffuse.is_none() {
            return false;
        };
        if self.normal.is_none() {
            return false;
        };
        if self.size.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.diffuse));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.normal));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.size = ::std::option::Option::Some(tmp);
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
        for value in &self.diffuse {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.normal {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.size.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.diffuse.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.normal.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.size {
            try!(os.write_double(3, v));
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
        ::std::any::TypeId::of::<HeightmapGeom_Texture>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HeightmapGeom_Texture {
    fn new() -> HeightmapGeom_Texture {
        HeightmapGeom_Texture::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeightmapGeom_Texture>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "diffuse",
                    HeightmapGeom_Texture::has_diffuse,
                    HeightmapGeom_Texture::get_diffuse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "normal",
                    HeightmapGeom_Texture::has_normal,
                    HeightmapGeom_Texture::get_normal,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "size",
                    HeightmapGeom_Texture::has_size,
                    HeightmapGeom_Texture::get_size,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeightmapGeom_Texture>(
                    "HeightmapGeom_Texture",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeightmapGeom_Texture {
    fn clear(&mut self) {
        self.clear_diffuse();
        self.clear_normal();
        self.clear_size();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HeightmapGeom_Texture {
    fn eq(&self, other: &HeightmapGeom_Texture) -> bool {
        self.diffuse == other.diffuse &&
        self.normal == other.normal &&
        self.size == other.size &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HeightmapGeom_Texture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct HeightmapGeom_Blend {
    // message fields
    min_height: ::std::option::Option<f64>,
    fade_dist: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeightmapGeom_Blend {}

impl HeightmapGeom_Blend {
    pub fn new() -> HeightmapGeom_Blend {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeightmapGeom_Blend {
        static mut instance: ::protobuf::lazy::Lazy<HeightmapGeom_Blend> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeightmapGeom_Blend,
        };
        unsafe {
            instance.get(|| {
                HeightmapGeom_Blend {
                    min_height: ::std::option::Option::None,
                    fade_dist: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double min_height = 1;

    pub fn clear_min_height(&mut self) {
        self.min_height = ::std::option::Option::None;
    }

    pub fn has_min_height(&self) -> bool {
        self.min_height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_height(&mut self, v: f64) {
        self.min_height = ::std::option::Option::Some(v);
    }

    pub fn get_min_height(&self) -> f64 {
        self.min_height.unwrap_or(0.)
    }

    // required double fade_dist = 2;

    pub fn clear_fade_dist(&mut self) {
        self.fade_dist = ::std::option::Option::None;
    }

    pub fn has_fade_dist(&self) -> bool {
        self.fade_dist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_dist(&mut self, v: f64) {
        self.fade_dist = ::std::option::Option::Some(v);
    }

    pub fn get_fade_dist(&self) -> f64 {
        self.fade_dist.unwrap_or(0.)
    }
}

impl ::protobuf::Message for HeightmapGeom_Blend {
    fn is_initialized(&self) -> bool {
        if self.min_height.is_none() {
            return false;
        };
        if self.fade_dist.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.min_height = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.fade_dist = ::std::option::Option::Some(tmp);
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
        if self.min_height.is_some() {
            my_size += 9;
        };
        if self.fade_dist.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.min_height {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.fade_dist {
            try!(os.write_double(2, v));
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
        ::std::any::TypeId::of::<HeightmapGeom_Blend>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HeightmapGeom_Blend {
    fn new() -> HeightmapGeom_Blend {
        HeightmapGeom_Blend::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeightmapGeom_Blend>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "min_height",
                    HeightmapGeom_Blend::has_min_height,
                    HeightmapGeom_Blend::get_min_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "fade_dist",
                    HeightmapGeom_Blend::has_fade_dist,
                    HeightmapGeom_Blend::get_fade_dist,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeightmapGeom_Blend>(
                    "HeightmapGeom_Blend",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeightmapGeom_Blend {
    fn clear(&mut self) {
        self.clear_min_height();
        self.clear_fade_dist();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HeightmapGeom_Blend {
    fn eq(&self, other: &HeightmapGeom_Blend) -> bool {
        self.min_height == other.min_height &&
        self.fade_dist == other.fade_dist &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HeightmapGeom_Blend {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x6d, 0x61, 0x70, 0x67, 0x65, 0x6f, 0x6d, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x1a, 0x0b, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x0e, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0xac, 0x03, 0x0a, 0x0d, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x6d, 0x61, 0x70, 0x47, 0x65, 0x6f,
    0x6d, 0x12, 0x21, 0x0a, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x49,
    0x6d, 0x61, 0x67, 0x65, 0x12, 0x23, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64, 0x12, 0x25, 0x0a, 0x06, 0x6f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67, 0x61, 0x7a, 0x65,
    0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x64,
    0x12, 0x0f, 0x0a, 0x07, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x02, 0x12, 0x0d, 0x0a, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05,
    0x12, 0x0e, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05,
    0x12, 0x33, 0x0a, 0x07, 0x74, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x18, 0x07, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x22, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x6d, 0x61, 0x70, 0x47, 0x65, 0x6f, 0x6d, 0x2e, 0x54, 0x65,
    0x78, 0x74, 0x75, 0x72, 0x65, 0x12, 0x2f, 0x0a, 0x05, 0x62, 0x6c, 0x65, 0x6e, 0x64, 0x18, 0x08,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x2e, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x6d, 0x61, 0x70, 0x47, 0x65, 0x6f, 0x6d,
    0x2e, 0x42, 0x6c, 0x65, 0x6e, 0x64, 0x12, 0x1a, 0x0a, 0x12, 0x75, 0x73, 0x65, 0x5f, 0x74, 0x65,
    0x72, 0x72, 0x61, 0x69, 0x6e, 0x5f, 0x70, 0x61, 0x67, 0x69, 0x6e, 0x67, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x09, 0x1a, 0x38, 0x0a, 0x07, 0x54, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x12,
    0x0f, 0x0a, 0x07, 0x64, 0x69, 0x66, 0x66, 0x75, 0x73, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x0e, 0x0a, 0x06, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x0c, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x1a, 0x2e,
    0x0a, 0x05, 0x42, 0x6c, 0x65, 0x6e, 0x64, 0x12, 0x12, 0x0a, 0x0a, 0x6d, 0x69, 0x6e, 0x5f, 0x68,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x66,
    0x61, 0x64, 0x65, 0x5f, 0x64, 0x69, 0x73, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x4a, 0xeb,
    0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x25, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x14, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x07, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x09, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08,
    0x15, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x27, 0x22, 0x11,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x20, 0x64, 0x61, 0x74, 0x61,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x25, 0x26, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x0c, 0x02, 0x27, 0x22, 0x10, 0x20, 0x53, 0x69, 0x7a, 0x65, 0x20, 0x69, 0x6e,
    0x20, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x0c, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c,
    0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x25, 0x26,
    0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x27, 0x22, 0x22, 0x20,
    0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20,
    0x63, 0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x0e, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0e,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x11, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x25, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x0f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x0f, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x10, 0x02, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00,
    0x12, 0x04, 0x12, 0x02, 0x17, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12,
    0x03, 0x12, 0x0a, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x14, 0x04, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x14, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x14, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x14, 0x14, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x14, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x15,
    0x04, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15,
    0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15,
    0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15,
    0x14, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15,
    0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x16, 0x04,
    0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x16, 0x04,
    0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x16, 0x0d,
    0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x16, 0x14,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x01, 0x12, 0x04, 0x19, 0x02, 0x1d, 0x03, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x01, 0x01, 0x12, 0x03, 0x19, 0x0a, 0x0f, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x27, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x14, 0x1e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1c, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x14, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x25, 0x26, 0x0a, 0x1f, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x06, 0x12, 0x03, 0x1f, 0x02, 0x27, 0x22, 0x12, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x06, 0x12, 0x03, 0x1f, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x1f, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x1f, 0x25, 0x26, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x20, 0x02,
    0x27, 0x22, 0x1b, 0x20, 0x48, 0x6f, 0x77, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x6c, 0x65, 0x6e, 0x64,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x73, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x20, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x20, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x20, 0x25, 0x26, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03,
    0x21, 0x02, 0x27, 0x22, 0x24, 0x20, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x65, 0x72,
    0x72, 0x61, 0x69, 0x6e, 0x20, 0x70, 0x61, 0x67, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x20, 0x72,
    0x65, 0x6e, 0x64, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05,
    0x12, 0x03, 0x21, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x21, 0x10, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x21, 0x25,
    0x26, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x24, 0x02, 0x28, 0x1a, 0x14,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x6e,
    0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x24,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x24, 0x12, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x24, 0x25, 0x27,
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
