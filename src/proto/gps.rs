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
pub struct GPS {
    // message fields
    time: ::protobuf::SingularPtrField<super::time::Time>,
    link_name: ::protobuf::SingularField<::std::string::String>,
    latitude_deg: ::std::option::Option<f64>,
    longitude_deg: ::std::option::Option<f64>,
    altitude: ::std::option::Option<f64>,
    velocity_east: ::std::option::Option<f64>,
    velocity_north: ::std::option::Option<f64>,
    velocity_up: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GPS {}

impl GPS {
    pub fn new() -> GPS {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GPS {
        static mut instance: ::protobuf::lazy::Lazy<GPS> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GPS,
        };
        unsafe {
            instance.get(|| {
                GPS {
                    time: ::protobuf::SingularPtrField::none(),
                    link_name: ::protobuf::SingularField::none(),
                    latitude_deg: ::std::option::Option::None,
                    longitude_deg: ::std::option::Option::None,
                    altitude: ::std::option::Option::None,
                    velocity_east: ::std::option::Option::None,
                    velocity_north: ::std::option::Option::None,
                    velocity_up: ::std::option::Option::None,
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

    // required string link_name = 2;

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

    // required double latitude_deg = 3;

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

    // required double longitude_deg = 4;

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

    // required double altitude = 5;

    pub fn clear_altitude(&mut self) {
        self.altitude = ::std::option::Option::None;
    }

    pub fn has_altitude(&self) -> bool {
        self.altitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_altitude(&mut self, v: f64) {
        self.altitude = ::std::option::Option::Some(v);
    }

    pub fn get_altitude(&self) -> f64 {
        self.altitude.unwrap_or(0.)
    }

    // optional double velocity_east = 6;

    pub fn clear_velocity_east(&mut self) {
        self.velocity_east = ::std::option::Option::None;
    }

    pub fn has_velocity_east(&self) -> bool {
        self.velocity_east.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity_east(&mut self, v: f64) {
        self.velocity_east = ::std::option::Option::Some(v);
    }

    pub fn get_velocity_east(&self) -> f64 {
        self.velocity_east.unwrap_or(0.)
    }

    // optional double velocity_north = 7;

    pub fn clear_velocity_north(&mut self) {
        self.velocity_north = ::std::option::Option::None;
    }

    pub fn has_velocity_north(&self) -> bool {
        self.velocity_north.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity_north(&mut self, v: f64) {
        self.velocity_north = ::std::option::Option::Some(v);
    }

    pub fn get_velocity_north(&self) -> f64 {
        self.velocity_north.unwrap_or(0.)
    }

    // optional double velocity_up = 8;

    pub fn clear_velocity_up(&mut self) {
        self.velocity_up = ::std::option::Option::None;
    }

    pub fn has_velocity_up(&self) -> bool {
        self.velocity_up.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity_up(&mut self, v: f64) {
        self.velocity_up = ::std::option::Option::Some(v);
    }

    pub fn get_velocity_up(&self) -> f64 {
        self.velocity_up.unwrap_or(0.)
    }
}

impl ::protobuf::Message for GPS {
    fn is_initialized(&self) -> bool {
        if self.time.is_none() {
            return false;
        };
        if self.link_name.is_none() {
            return false;
        };
        if self.latitude_deg.is_none() {
            return false;
        };
        if self.longitude_deg.is_none() {
            return false;
        };
        if self.altitude.is_none() {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.link_name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.latitude_deg = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.longitude_deg = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.altitude = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.velocity_east = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.velocity_north = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.velocity_up = ::std::option::Option::Some(tmp);
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
        for value in &self.link_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.latitude_deg.is_some() {
            my_size += 9;
        };
        if self.longitude_deg.is_some() {
            my_size += 9;
        };
        if self.altitude.is_some() {
            my_size += 9;
        };
        if self.velocity_east.is_some() {
            my_size += 9;
        };
        if self.velocity_north.is_some() {
            my_size += 9;
        };
        if self.velocity_up.is_some() {
            my_size += 9;
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
        if let Some(v) = self.link_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.latitude_deg {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.longitude_deg {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.altitude {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.velocity_east {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.velocity_north {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.velocity_up {
            try!(os.write_double(8, v));
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
        ::std::any::TypeId::of::<GPS>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GPS {
    fn new() -> GPS {
        GPS::new()
    }

    fn descriptor_static(_: ::std::option::Option<GPS>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "time",
                    GPS::has_time,
                    GPS::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "link_name",
                    GPS::has_link_name,
                    GPS::get_link_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude_deg",
                    GPS::has_latitude_deg,
                    GPS::get_latitude_deg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude_deg",
                    GPS::has_longitude_deg,
                    GPS::get_longitude_deg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "altitude",
                    GPS::has_altitude,
                    GPS::get_altitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "velocity_east",
                    GPS::has_velocity_east,
                    GPS::get_velocity_east,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "velocity_north",
                    GPS::has_velocity_north,
                    GPS::get_velocity_north,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "velocity_up",
                    GPS::has_velocity_up,
                    GPS::get_velocity_up,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GPS>(
                    "GPS",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GPS {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_link_name();
        self.clear_latitude_deg();
        self.clear_longitude_deg();
        self.clear_altitude();
        self.clear_velocity_east();
        self.clear_velocity_north();
        self.clear_velocity_up();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GPS {
    fn eq(&self, other: &GPS) -> bool {
        self.time == other.time &&
        self.link_name == other.link_name &&
        self.latitude_deg == other.latitude_deg &&
        self.longitude_deg == other.longitude_deg &&
        self.altitude == other.altitude &&
        self.velocity_east == other.velocity_east &&
        self.velocity_north == other.velocity_north &&
        self.velocity_up == other.velocity_up &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GPS {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x67, 0x70, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0xbc, 0x01, 0x0a, 0x03, 0x47, 0x50, 0x53, 0x12, 0x1f, 0x0a, 0x04,
    0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x11, 0x0a,
    0x09, 0x6c, 0x69, 0x6e, 0x6b, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x14, 0x0a, 0x0c, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x5f, 0x64, 0x65, 0x67,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x12, 0x15, 0x0a, 0x0d, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x5f, 0x64, 0x65, 0x67, 0x18, 0x04, 0x20, 0x02, 0x28, 0x01, 0x12, 0x10, 0x0a,
    0x08, 0x61, 0x6c, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28, 0x01, 0x12,
    0x15, 0x0a, 0x0d, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x5f, 0x65, 0x61, 0x73, 0x74,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x01, 0x12, 0x16, 0x0a, 0x0e, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69,
    0x74, 0x79, 0x5f, 0x6e, 0x6f, 0x72, 0x74, 0x68, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01, 0x12, 0x13,
    0x0a, 0x0b, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x5f, 0x75, 0x70, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x01, 0x4a, 0xdd, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x13, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x07, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x13, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x0b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b,
    0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x2a, 0x2b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0c, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d,
    0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x0e, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x12,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x2a, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0f, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x0f, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x10, 0x02,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x06, 0x12, 0x03, 0x11, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x11,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x11, 0x12, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x11, 0x2a, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x12, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x12, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x12, 0x2a, 0x2b,
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
