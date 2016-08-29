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
pub struct Sky {
    // message fields
    time: ::std::option::Option<f64>,
    sunrise: ::std::option::Option<f64>,
    sunset: ::std::option::Option<f64>,
    wind_speed: ::std::option::Option<f64>,
    wind_direction: ::std::option::Option<f64>,
    cloud_ambient: ::protobuf::SingularPtrField<super::color::Color>,
    humidity: ::std::option::Option<f64>,
    mean_cloud_size: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Sky {}

impl Sky {
    pub fn new() -> Sky {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Sky {
        static mut instance: ::protobuf::lazy::Lazy<Sky> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Sky,
        };
        unsafe {
            instance.get(|| {
                Sky {
                    time: ::std::option::Option::None,
                    sunrise: ::std::option::Option::None,
                    sunset: ::std::option::Option::None,
                    wind_speed: ::std::option::Option::None,
                    wind_direction: ::std::option::Option::None,
                    cloud_ambient: ::protobuf::SingularPtrField::none(),
                    humidity: ::std::option::Option::None,
                    mean_cloud_size: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double time = 1;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: f64) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time(&self) -> f64 {
        self.time.unwrap_or(0.)
    }

    // optional double sunrise = 2;

    pub fn clear_sunrise(&mut self) {
        self.sunrise = ::std::option::Option::None;
    }

    pub fn has_sunrise(&self) -> bool {
        self.sunrise.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sunrise(&mut self, v: f64) {
        self.sunrise = ::std::option::Option::Some(v);
    }

    pub fn get_sunrise(&self) -> f64 {
        self.sunrise.unwrap_or(0.)
    }

    // optional double sunset = 3;

    pub fn clear_sunset(&mut self) {
        self.sunset = ::std::option::Option::None;
    }

    pub fn has_sunset(&self) -> bool {
        self.sunset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sunset(&mut self, v: f64) {
        self.sunset = ::std::option::Option::Some(v);
    }

    pub fn get_sunset(&self) -> f64 {
        self.sunset.unwrap_or(0.)
    }

    // optional double wind_speed = 4;

    pub fn clear_wind_speed(&mut self) {
        self.wind_speed = ::std::option::Option::None;
    }

    pub fn has_wind_speed(&self) -> bool {
        self.wind_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wind_speed(&mut self, v: f64) {
        self.wind_speed = ::std::option::Option::Some(v);
    }

    pub fn get_wind_speed(&self) -> f64 {
        self.wind_speed.unwrap_or(0.)
    }

    // optional double wind_direction = 5;

    pub fn clear_wind_direction(&mut self) {
        self.wind_direction = ::std::option::Option::None;
    }

    pub fn has_wind_direction(&self) -> bool {
        self.wind_direction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wind_direction(&mut self, v: f64) {
        self.wind_direction = ::std::option::Option::Some(v);
    }

    pub fn get_wind_direction(&self) -> f64 {
        self.wind_direction.unwrap_or(0.)
    }

    // optional .gazebo.msgs.Color cloud_ambient = 6;

    pub fn clear_cloud_ambient(&mut self) {
        self.cloud_ambient.clear();
    }

    pub fn has_cloud_ambient(&self) -> bool {
        self.cloud_ambient.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cloud_ambient(&mut self, v: super::color::Color) {
        self.cloud_ambient = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cloud_ambient(&mut self) -> &mut super::color::Color {
        if self.cloud_ambient.is_none() {
            self.cloud_ambient.set_default();
        };
        self.cloud_ambient.as_mut().unwrap()
    }

    // Take field
    pub fn take_cloud_ambient(&mut self) -> super::color::Color {
        self.cloud_ambient.take().unwrap_or_else(|| super::color::Color::new())
    }

    pub fn get_cloud_ambient(&self) -> &super::color::Color {
        self.cloud_ambient.as_ref().unwrap_or_else(|| super::color::Color::default_instance())
    }

    // optional double humidity = 7;

    pub fn clear_humidity(&mut self) {
        self.humidity = ::std::option::Option::None;
    }

    pub fn has_humidity(&self) -> bool {
        self.humidity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_humidity(&mut self, v: f64) {
        self.humidity = ::std::option::Option::Some(v);
    }

    pub fn get_humidity(&self) -> f64 {
        self.humidity.unwrap_or(0.)
    }

    // optional double mean_cloud_size = 8;

    pub fn clear_mean_cloud_size(&mut self) {
        self.mean_cloud_size = ::std::option::Option::None;
    }

    pub fn has_mean_cloud_size(&self) -> bool {
        self.mean_cloud_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mean_cloud_size(&mut self, v: f64) {
        self.mean_cloud_size = ::std::option::Option::Some(v);
    }

    pub fn get_mean_cloud_size(&self) -> f64 {
        self.mean_cloud_size.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Sky {
    fn is_initialized(&self) -> bool {
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
                    self.time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.sunrise = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.sunset = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.wind_speed = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.wind_direction = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cloud_ambient));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.humidity = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.mean_cloud_size = ::std::option::Option::Some(tmp);
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
        if self.time.is_some() {
            my_size += 9;
        };
        if self.sunrise.is_some() {
            my_size += 9;
        };
        if self.sunset.is_some() {
            my_size += 9;
        };
        if self.wind_speed.is_some() {
            my_size += 9;
        };
        if self.wind_direction.is_some() {
            my_size += 9;
        };
        for value in &self.cloud_ambient {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.humidity.is_some() {
            my_size += 9;
        };
        if self.mean_cloud_size.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.sunrise {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.sunset {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.wind_speed {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.wind_direction {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.cloud_ambient.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.humidity {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.mean_cloud_size {
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
        ::std::any::TypeId::of::<Sky>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Sky {
    fn new() -> Sky {
        Sky::new()
    }

    fn descriptor_static(_: ::std::option::Option<Sky>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "time",
                    Sky::has_time,
                    Sky::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "sunrise",
                    Sky::has_sunrise,
                    Sky::get_sunrise,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "sunset",
                    Sky::has_sunset,
                    Sky::get_sunset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "wind_speed",
                    Sky::has_wind_speed,
                    Sky::get_wind_speed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "wind_direction",
                    Sky::has_wind_direction,
                    Sky::get_wind_direction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cloud_ambient",
                    Sky::has_cloud_ambient,
                    Sky::get_cloud_ambient,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "humidity",
                    Sky::has_humidity,
                    Sky::get_humidity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "mean_cloud_size",
                    Sky::has_mean_cloud_size,
                    Sky::get_mean_cloud_size,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Sky>(
                    "Sky",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Sky {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_sunrise();
        self.clear_sunset();
        self.clear_wind_speed();
        self.clear_wind_direction();
        self.clear_cloud_ambient();
        self.clear_humidity();
        self.clear_mean_cloud_size();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Sky {
    fn eq(&self, other: &Sky) -> bool {
        self.time == other.time &&
        self.sunrise == other.sunrise &&
        self.sunset == other.sunset &&
        self.wind_speed == other.wind_speed &&
        self.wind_direction == other.wind_direction &&
        self.cloud_ambient == other.cloud_ambient &&
        self.humidity == other.humidity &&
        self.mean_cloud_size == other.mean_cloud_size &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Sky {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x73, 0x6b, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0b, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb6, 0x01, 0x0a, 0x03, 0x53, 0x6b, 0x79, 0x12, 0x0c, 0x0a,
    0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0f, 0x0a, 0x07, 0x73,
    0x75, 0x6e, 0x72, 0x69, 0x73, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0e, 0x0a, 0x06,
    0x73, 0x75, 0x6e, 0x73, 0x65, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x12, 0x12, 0x0a, 0x0a,
    0x77, 0x69, 0x6e, 0x64, 0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01,
    0x12, 0x16, 0x0a, 0x0e, 0x77, 0x69, 0x6e, 0x64, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x12, 0x29, 0x0a, 0x0d, 0x63, 0x6c, 0x6f, 0x75,
    0x64, 0x5f, 0x61, 0x6d, 0x62, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x43, 0x6f,
    0x6c, 0x6f, 0x72, 0x12, 0x10, 0x0a, 0x08, 0x68, 0x75, 0x6d, 0x69, 0x64, 0x69, 0x74, 0x79, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x01, 0x12, 0x17, 0x0a, 0x0f, 0x6d, 0x65, 0x61, 0x6e, 0x5f, 0x63, 0x6c,
    0x6f, 0x75, 0x64, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x01, 0x4a, 0xdd,
    0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x14, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x14, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x09, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0b, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c,
    0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0f,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0f, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x10, 0x12,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x10, 0x23, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x11, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x06, 0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x11, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x11, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x12, 0x02,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x12, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x12, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x07, 0x12, 0x03, 0x13, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x13,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x13, 0x12, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x13, 0x24, 0x25,
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
