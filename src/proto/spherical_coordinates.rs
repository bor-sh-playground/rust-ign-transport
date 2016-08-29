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
pub struct SphericalCoordinates {
    // message fields
    surface_model: ::std::option::Option<SphericalCoordinates_SurfaceModel>,
    latitude_deg: ::std::option::Option<f64>,
    longitude_deg: ::std::option::Option<f64>,
    elevation: ::std::option::Option<f64>,
    heading_deg: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SphericalCoordinates {}

impl SphericalCoordinates {
    pub fn new() -> SphericalCoordinates {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SphericalCoordinates {
        static mut instance: ::protobuf::lazy::Lazy<SphericalCoordinates> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SphericalCoordinates,
        };
        unsafe {
            instance.get(|| {
                SphericalCoordinates {
                    surface_model: ::std::option::Option::None,
                    latitude_deg: ::std::option::Option::None,
                    longitude_deg: ::std::option::Option::None,
                    elevation: ::std::option::Option::None,
                    heading_deg: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.SphericalCoordinates.SurfaceModel surface_model = 1;

    pub fn clear_surface_model(&mut self) {
        self.surface_model = ::std::option::Option::None;
    }

    pub fn has_surface_model(&self) -> bool {
        self.surface_model.is_some()
    }

    // Param is passed by value, moved
    pub fn set_surface_model(&mut self, v: SphericalCoordinates_SurfaceModel) {
        self.surface_model = ::std::option::Option::Some(v);
    }

    pub fn get_surface_model(&self) -> SphericalCoordinates_SurfaceModel {
        self.surface_model.unwrap_or(SphericalCoordinates_SurfaceModel::EARTH_WGS84)
    }

    // required double latitude_deg = 2;

    pub fn clear_latitude_deg(&mut self) {
        self.latitude_deg = ::std::option::Option::None;
    }

    pub fn has_latitude_deg(&self) -> bool {
        self.latitude_deg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latitude_deg(&mut self, v: f64) {
        self.latitude_deg = ::std::option::Option::Some(v);
    }

    pub fn get_latitude_deg(&self) -> f64 {
        self.latitude_deg.unwrap_or(0.)
    }

    // required double longitude_deg = 3;

    pub fn clear_longitude_deg(&mut self) {
        self.longitude_deg = ::std::option::Option::None;
    }

    pub fn has_longitude_deg(&self) -> bool {
        self.longitude_deg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_longitude_deg(&mut self, v: f64) {
        self.longitude_deg = ::std::option::Option::Some(v);
    }

    pub fn get_longitude_deg(&self) -> f64 {
        self.longitude_deg.unwrap_or(0.)
    }

    // required double elevation = 4;

    pub fn clear_elevation(&mut self) {
        self.elevation = ::std::option::Option::None;
    }

    pub fn has_elevation(&self) -> bool {
        self.elevation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_elevation(&mut self, v: f64) {
        self.elevation = ::std::option::Option::Some(v);
    }

    pub fn get_elevation(&self) -> f64 {
        self.elevation.unwrap_or(0.)
    }

    // required double heading_deg = 5;

    pub fn clear_heading_deg(&mut self) {
        self.heading_deg = ::std::option::Option::None;
    }

    pub fn has_heading_deg(&self) -> bool {
        self.heading_deg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_heading_deg(&mut self, v: f64) {
        self.heading_deg = ::std::option::Option::Some(v);
    }

    pub fn get_heading_deg(&self) -> f64 {
        self.heading_deg.unwrap_or(0.)
    }
}

impl ::protobuf::Message for SphericalCoordinates {
    fn is_initialized(&self) -> bool {
        if self.surface_model.is_none() {
            return false;
        };
        if self.latitude_deg.is_none() {
            return false;
        };
        if self.longitude_deg.is_none() {
            return false;
        };
        if self.elevation.is_none() {
            return false;
        };
        if self.heading_deg.is_none() {
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
                    let tmp = try!(is.read_enum());
                    self.surface_model = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.latitude_deg = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.longitude_deg = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.elevation = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.heading_deg = ::std::option::Option::Some(tmp);
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
        for value in &self.surface_model {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        if self.latitude_deg.is_some() {
            my_size += 9;
        };
        if self.longitude_deg.is_some() {
            my_size += 9;
        };
        if self.elevation.is_some() {
            my_size += 9;
        };
        if self.heading_deg.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.surface_model {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.latitude_deg {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.longitude_deg {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.elevation {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.heading_deg {
            try!(os.write_double(5, v));
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
        ::std::any::TypeId::of::<SphericalCoordinates>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SphericalCoordinates {
    fn new() -> SphericalCoordinates {
        SphericalCoordinates::new()
    }

    fn descriptor_static(_: ::std::option::Option<SphericalCoordinates>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "surface_model",
                    SphericalCoordinates::has_surface_model,
                    SphericalCoordinates::get_surface_model,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude_deg",
                    SphericalCoordinates::has_latitude_deg,
                    SphericalCoordinates::get_latitude_deg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude_deg",
                    SphericalCoordinates::has_longitude_deg,
                    SphericalCoordinates::get_longitude_deg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "elevation",
                    SphericalCoordinates::has_elevation,
                    SphericalCoordinates::get_elevation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "heading_deg",
                    SphericalCoordinates::has_heading_deg,
                    SphericalCoordinates::get_heading_deg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SphericalCoordinates>(
                    "SphericalCoordinates",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SphericalCoordinates {
    fn clear(&mut self) {
        self.clear_surface_model();
        self.clear_latitude_deg();
        self.clear_longitude_deg();
        self.clear_elevation();
        self.clear_heading_deg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SphericalCoordinates {
    fn eq(&self, other: &SphericalCoordinates) -> bool {
        self.surface_model == other.surface_model &&
        self.latitude_deg == other.latitude_deg &&
        self.longitude_deg == other.longitude_deg &&
        self.elevation == other.elevation &&
        self.heading_deg == other.heading_deg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SphericalCoordinates {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SphericalCoordinates_SurfaceModel {
    EARTH_WGS84 = 1,
}

impl ::protobuf::ProtobufEnum for SphericalCoordinates_SurfaceModel {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SphericalCoordinates_SurfaceModel> {
        match value {
            1 => ::std::option::Option::Some(SphericalCoordinates_SurfaceModel::EARTH_WGS84),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SphericalCoordinates_SurfaceModel] = &[
            SphericalCoordinates_SurfaceModel::EARTH_WGS84,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SphericalCoordinates_SurfaceModel>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SphericalCoordinates_SurfaceModel", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SphericalCoordinates_SurfaceModel {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1b, 0x73, 0x70, 0x68, 0x65, 0x72, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x63, 0x6f, 0x6f, 0x72,
    0x64, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0xd3, 0x01, 0x0a, 0x14, 0x53,
    0x70, 0x68, 0x65, 0x72, 0x69, 0x63, 0x61, 0x6c, 0x43, 0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61,
    0x74, 0x65, 0x73, 0x12, 0x45, 0x0a, 0x0d, 0x73, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x5f, 0x6d,
    0x6f, 0x64, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x2e, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x53, 0x70, 0x68, 0x65, 0x72, 0x69, 0x63,
    0x61, 0x6c, 0x43, 0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x53, 0x75,
    0x72, 0x66, 0x61, 0x63, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x12, 0x14, 0x0a, 0x0c, 0x6c, 0x61,
    0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x5f, 0x64, 0x65, 0x67, 0x18, 0x02, 0x20, 0x02, 0x28, 0x01,
    0x12, 0x15, 0x0a, 0x0d, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x5f, 0x64, 0x65,
    0x67, 0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x65, 0x6c, 0x65, 0x76, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x02, 0x28, 0x01, 0x12, 0x13, 0x0a, 0x0b, 0x68, 0x65,
    0x61, 0x64, 0x69, 0x6e, 0x67, 0x5f, 0x64, 0x65, 0x67, 0x18, 0x05, 0x20, 0x02, 0x28, 0x01, 0x22,
    0x1f, 0x0a, 0x0c, 0x53, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x12,
    0x0f, 0x0a, 0x0b, 0x45, 0x41, 0x52, 0x54, 0x48, 0x5f, 0x57, 0x47, 0x53, 0x38, 0x34, 0x10, 0x01,
    0x4a, 0xce, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x12, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x12,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x1c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x08, 0x02, 0x0b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x0a, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0d, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0d, 0x0b,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x18, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0e, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e,
    0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x02, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0f, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x10, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x10,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x12, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x11, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x11, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x11, 0x2a,
    0x2b,
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
