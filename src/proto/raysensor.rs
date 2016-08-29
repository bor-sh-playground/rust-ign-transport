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
pub struct RaySensor {
    // message fields
    display_scan: ::std::option::Option<bool>,
    horizontal_samples: ::std::option::Option<i32>,
    horizontal_resolution: ::std::option::Option<f64>,
    horizontal_min_angle: ::std::option::Option<f64>,
    horizontal_max_angle: ::std::option::Option<f64>,
    vertical_samples: ::std::option::Option<i32>,
    vertical_resolution: ::std::option::Option<f64>,
    vertical_min_angle: ::std::option::Option<f64>,
    vertical_max_angle: ::std::option::Option<f64>,
    range_min: ::std::option::Option<f64>,
    range_max: ::std::option::Option<f64>,
    range_resolution: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaySensor {}

impl RaySensor {
    pub fn new() -> RaySensor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaySensor {
        static mut instance: ::protobuf::lazy::Lazy<RaySensor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaySensor,
        };
        unsafe {
            instance.get(|| {
                RaySensor {
                    display_scan: ::std::option::Option::None,
                    horizontal_samples: ::std::option::Option::None,
                    horizontal_resolution: ::std::option::Option::None,
                    horizontal_min_angle: ::std::option::Option::None,
                    horizontal_max_angle: ::std::option::Option::None,
                    vertical_samples: ::std::option::Option::None,
                    vertical_resolution: ::std::option::Option::None,
                    vertical_min_angle: ::std::option::Option::None,
                    vertical_max_angle: ::std::option::Option::None,
                    range_min: ::std::option::Option::None,
                    range_max: ::std::option::Option::None,
                    range_resolution: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool display_scan = 1;

    pub fn clear_display_scan(&mut self) {
        self.display_scan = ::std::option::Option::None;
    }

    pub fn has_display_scan(&self) -> bool {
        self.display_scan.is_some()
    }

    // Param is passed by value, moved
    pub fn set_display_scan(&mut self, v: bool) {
        self.display_scan = ::std::option::Option::Some(v);
    }

    pub fn get_display_scan(&self) -> bool {
        self.display_scan.unwrap_or(false)
    }

    // optional int32 horizontal_samples = 2;

    pub fn clear_horizontal_samples(&mut self) {
        self.horizontal_samples = ::std::option::Option::None;
    }

    pub fn has_horizontal_samples(&self) -> bool {
        self.horizontal_samples.is_some()
    }

    // Param is passed by value, moved
    pub fn set_horizontal_samples(&mut self, v: i32) {
        self.horizontal_samples = ::std::option::Option::Some(v);
    }

    pub fn get_horizontal_samples(&self) -> i32 {
        self.horizontal_samples.unwrap_or(0)
    }

    // optional double horizontal_resolution = 3;

    pub fn clear_horizontal_resolution(&mut self) {
        self.horizontal_resolution = ::std::option::Option::None;
    }

    pub fn has_horizontal_resolution(&self) -> bool {
        self.horizontal_resolution.is_some()
    }

    // Param is passed by value, moved
    pub fn set_horizontal_resolution(&mut self, v: f64) {
        self.horizontal_resolution = ::std::option::Option::Some(v);
    }

    pub fn get_horizontal_resolution(&self) -> f64 {
        self.horizontal_resolution.unwrap_or(0.)
    }

    // optional double horizontal_min_angle = 4;

    pub fn clear_horizontal_min_angle(&mut self) {
        self.horizontal_min_angle = ::std::option::Option::None;
    }

    pub fn has_horizontal_min_angle(&self) -> bool {
        self.horizontal_min_angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_horizontal_min_angle(&mut self, v: f64) {
        self.horizontal_min_angle = ::std::option::Option::Some(v);
    }

    pub fn get_horizontal_min_angle(&self) -> f64 {
        self.horizontal_min_angle.unwrap_or(0.)
    }

    // optional double horizontal_max_angle = 5;

    pub fn clear_horizontal_max_angle(&mut self) {
        self.horizontal_max_angle = ::std::option::Option::None;
    }

    pub fn has_horizontal_max_angle(&self) -> bool {
        self.horizontal_max_angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_horizontal_max_angle(&mut self, v: f64) {
        self.horizontal_max_angle = ::std::option::Option::Some(v);
    }

    pub fn get_horizontal_max_angle(&self) -> f64 {
        self.horizontal_max_angle.unwrap_or(0.)
    }

    // optional int32 vertical_samples = 6;

    pub fn clear_vertical_samples(&mut self) {
        self.vertical_samples = ::std::option::Option::None;
    }

    pub fn has_vertical_samples(&self) -> bool {
        self.vertical_samples.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_samples(&mut self, v: i32) {
        self.vertical_samples = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_samples(&self) -> i32 {
        self.vertical_samples.unwrap_or(0)
    }

    // optional double vertical_resolution = 7;

    pub fn clear_vertical_resolution(&mut self) {
        self.vertical_resolution = ::std::option::Option::None;
    }

    pub fn has_vertical_resolution(&self) -> bool {
        self.vertical_resolution.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_resolution(&mut self, v: f64) {
        self.vertical_resolution = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_resolution(&self) -> f64 {
        self.vertical_resolution.unwrap_or(0.)
    }

    // optional double vertical_min_angle = 8;

    pub fn clear_vertical_min_angle(&mut self) {
        self.vertical_min_angle = ::std::option::Option::None;
    }

    pub fn has_vertical_min_angle(&self) -> bool {
        self.vertical_min_angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_min_angle(&mut self, v: f64) {
        self.vertical_min_angle = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_min_angle(&self) -> f64 {
        self.vertical_min_angle.unwrap_or(0.)
    }

    // optional double vertical_max_angle = 9;

    pub fn clear_vertical_max_angle(&mut self) {
        self.vertical_max_angle = ::std::option::Option::None;
    }

    pub fn has_vertical_max_angle(&self) -> bool {
        self.vertical_max_angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_max_angle(&mut self, v: f64) {
        self.vertical_max_angle = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_max_angle(&self) -> f64 {
        self.vertical_max_angle.unwrap_or(0.)
    }

    // optional double range_min = 10;

    pub fn clear_range_min(&mut self) {
        self.range_min = ::std::option::Option::None;
    }

    pub fn has_range_min(&self) -> bool {
        self.range_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_min(&mut self, v: f64) {
        self.range_min = ::std::option::Option::Some(v);
    }

    pub fn get_range_min(&self) -> f64 {
        self.range_min.unwrap_or(0.)
    }

    // optional double range_max = 11;

    pub fn clear_range_max(&mut self) {
        self.range_max = ::std::option::Option::None;
    }

    pub fn has_range_max(&self) -> bool {
        self.range_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_max(&mut self, v: f64) {
        self.range_max = ::std::option::Option::Some(v);
    }

    pub fn get_range_max(&self) -> f64 {
        self.range_max.unwrap_or(0.)
    }

    // optional double range_resolution = 12;

    pub fn clear_range_resolution(&mut self) {
        self.range_resolution = ::std::option::Option::None;
    }

    pub fn has_range_resolution(&self) -> bool {
        self.range_resolution.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_resolution(&mut self, v: f64) {
        self.range_resolution = ::std::option::Option::Some(v);
    }

    pub fn get_range_resolution(&self) -> f64 {
        self.range_resolution.unwrap_or(0.)
    }
}

impl ::protobuf::Message for RaySensor {
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
                    self.display_scan = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.horizontal_samples = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.horizontal_resolution = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.horizontal_min_angle = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.horizontal_max_angle = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.vertical_samples = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vertical_resolution = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vertical_min_angle = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vertical_max_angle = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.range_min = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.range_max = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.range_resolution = ::std::option::Option::Some(tmp);
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
        if self.display_scan.is_some() {
            my_size += 2;
        };
        for value in &self.horizontal_samples {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.horizontal_resolution.is_some() {
            my_size += 9;
        };
        if self.horizontal_min_angle.is_some() {
            my_size += 9;
        };
        if self.horizontal_max_angle.is_some() {
            my_size += 9;
        };
        for value in &self.vertical_samples {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.vertical_resolution.is_some() {
            my_size += 9;
        };
        if self.vertical_min_angle.is_some() {
            my_size += 9;
        };
        if self.vertical_max_angle.is_some() {
            my_size += 9;
        };
        if self.range_min.is_some() {
            my_size += 9;
        };
        if self.range_max.is_some() {
            my_size += 9;
        };
        if self.range_resolution.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.display_scan {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.horizontal_samples {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.horizontal_resolution {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.horizontal_min_angle {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.horizontal_max_angle {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.vertical_samples {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.vertical_resolution {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.vertical_min_angle {
            try!(os.write_double(8, v));
        };
        if let Some(v) = self.vertical_max_angle {
            try!(os.write_double(9, v));
        };
        if let Some(v) = self.range_min {
            try!(os.write_double(10, v));
        };
        if let Some(v) = self.range_max {
            try!(os.write_double(11, v));
        };
        if let Some(v) = self.range_resolution {
            try!(os.write_double(12, v));
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
        ::std::any::TypeId::of::<RaySensor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RaySensor {
    fn new() -> RaySensor {
        RaySensor::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaySensor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "display_scan",
                    RaySensor::has_display_scan,
                    RaySensor::get_display_scan,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "horizontal_samples",
                    RaySensor::has_horizontal_samples,
                    RaySensor::get_horizontal_samples,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "horizontal_resolution",
                    RaySensor::has_horizontal_resolution,
                    RaySensor::get_horizontal_resolution,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "horizontal_min_angle",
                    RaySensor::has_horizontal_min_angle,
                    RaySensor::get_horizontal_min_angle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "horizontal_max_angle",
                    RaySensor::has_horizontal_max_angle,
                    RaySensor::get_horizontal_max_angle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "vertical_samples",
                    RaySensor::has_vertical_samples,
                    RaySensor::get_vertical_samples,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vertical_resolution",
                    RaySensor::has_vertical_resolution,
                    RaySensor::get_vertical_resolution,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vertical_min_angle",
                    RaySensor::has_vertical_min_angle,
                    RaySensor::get_vertical_min_angle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vertical_max_angle",
                    RaySensor::has_vertical_max_angle,
                    RaySensor::get_vertical_max_angle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "range_min",
                    RaySensor::has_range_min,
                    RaySensor::get_range_min,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "range_max",
                    RaySensor::has_range_max,
                    RaySensor::get_range_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "range_resolution",
                    RaySensor::has_range_resolution,
                    RaySensor::get_range_resolution,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaySensor>(
                    "RaySensor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaySensor {
    fn clear(&mut self) {
        self.clear_display_scan();
        self.clear_horizontal_samples();
        self.clear_horizontal_resolution();
        self.clear_horizontal_min_angle();
        self.clear_horizontal_max_angle();
        self.clear_vertical_samples();
        self.clear_vertical_resolution();
        self.clear_vertical_min_angle();
        self.clear_vertical_max_angle();
        self.clear_range_min();
        self.clear_range_max();
        self.clear_range_resolution();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RaySensor {
    fn eq(&self, other: &RaySensor) -> bool {
        self.display_scan == other.display_scan &&
        self.horizontal_samples == other.horizontal_samples &&
        self.horizontal_resolution == other.horizontal_resolution &&
        self.horizontal_min_angle == other.horizontal_min_angle &&
        self.horizontal_max_angle == other.horizontal_max_angle &&
        self.vertical_samples == other.vertical_samples &&
        self.vertical_resolution == other.vertical_resolution &&
        self.vertical_min_angle == other.vertical_min_angle &&
        self.vertical_max_angle == other.vertical_max_angle &&
        self.range_min == other.range_min &&
        self.range_max == other.range_max &&
        self.range_resolution == other.range_resolution &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RaySensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x72, 0x61, 0x79, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0xc7,
    0x02, 0x0a, 0x09, 0x52, 0x61, 0x79, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x0c,
    0x64, 0x69, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x5f, 0x73, 0x63, 0x61, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x1a, 0x0a, 0x12, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x6f, 0x6e, 0x74, 0x61, 0x6c,
    0x5f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1d,
    0x0a, 0x15, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x6f, 0x6e, 0x74, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73,
    0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1c, 0x0a,
    0x14, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x6f, 0x6e, 0x74, 0x61, 0x6c, 0x5f, 0x6d, 0x69, 0x6e, 0x5f,
    0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1c, 0x0a, 0x14, 0x68,
    0x6f, 0x72, 0x69, 0x7a, 0x6f, 0x6e, 0x74, 0x61, 0x6c, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x61, 0x6e,
    0x67, 0x6c, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x12, 0x18, 0x0a, 0x10, 0x76, 0x65, 0x72,
    0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x05, 0x12, 0x1b, 0x0a, 0x13, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f,
    0x72, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01,
    0x12, 0x1a, 0x0a, 0x12, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x6d, 0x69, 0x6e,
    0x5f, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1a, 0x0a, 0x12,
    0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x61, 0x6e, 0x67,
    0x6c, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x5f, 0x6d, 0x69, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x01, 0x12, 0x18,
    0x0a, 0x10, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x01, 0x4a, 0xe6, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x17, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x07, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09,
    0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x0a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x11,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x2a, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x0b, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x0b, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x0d, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0d,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x2a, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x0f, 0x11, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x0f, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x10, 0x02, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x12, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x10, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07,
    0x12, 0x03, 0x11, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03,
    0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x11, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x11, 0x12, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x11, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x12, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08,
    0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x12, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x12,
    0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x14, 0x02, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x14, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x09, 0x03, 0x12, 0x03, 0x14, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12,
    0x03, 0x15, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x15,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x15, 0x12, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x15, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x16, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0b, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05,
    0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03,
    0x16, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x16, 0x2a,
    0x2c,
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
