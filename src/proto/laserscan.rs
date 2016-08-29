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
pub struct LaserScan {
    // message fields
    frame: ::protobuf::SingularField<::std::string::String>,
    world_pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    angle_min: ::std::option::Option<f64>,
    angle_max: ::std::option::Option<f64>,
    angle_step: ::std::option::Option<f64>,
    range_min: ::std::option::Option<f64>,
    range_max: ::std::option::Option<f64>,
    count: ::std::option::Option<u32>,
    vertical_angle_min: ::std::option::Option<f64>,
    vertical_angle_max: ::std::option::Option<f64>,
    vertical_angle_step: ::std::option::Option<f64>,
    vertical_count: ::std::option::Option<u32>,
    ranges: ::std::vec::Vec<f64>,
    intensities: ::std::vec::Vec<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LaserScan {}

impl LaserScan {
    pub fn new() -> LaserScan {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LaserScan {
        static mut instance: ::protobuf::lazy::Lazy<LaserScan> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LaserScan,
        };
        unsafe {
            instance.get(|| {
                LaserScan {
                    frame: ::protobuf::SingularField::none(),
                    world_pose: ::protobuf::SingularPtrField::none(),
                    angle_min: ::std::option::Option::None,
                    angle_max: ::std::option::Option::None,
                    angle_step: ::std::option::Option::None,
                    range_min: ::std::option::Option::None,
                    range_max: ::std::option::Option::None,
                    count: ::std::option::Option::None,
                    vertical_angle_min: ::std::option::Option::None,
                    vertical_angle_max: ::std::option::Option::None,
                    vertical_angle_step: ::std::option::Option::None,
                    vertical_count: ::std::option::Option::None,
                    ranges: ::std::vec::Vec::new(),
                    intensities: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string frame = 1;

    pub fn clear_frame(&mut self) {
        self.frame.clear();
    }

    pub fn has_frame(&self) -> bool {
        self.frame.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame(&mut self, v: ::std::string::String) {
        self.frame = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_frame(&mut self) -> &mut ::std::string::String {
        if self.frame.is_none() {
            self.frame.set_default();
        };
        self.frame.as_mut().unwrap()
    }

    // Take field
    pub fn take_frame(&mut self) -> ::std::string::String {
        self.frame.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_frame(&self) -> &str {
        match self.frame.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .gazebo.msgs.Pose world_pose = 2;

    pub fn clear_world_pose(&mut self) {
        self.world_pose.clear();
    }

    pub fn has_world_pose(&self) -> bool {
        self.world_pose.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_pose(&mut self, v: super::pose::Pose) {
        self.world_pose = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_pose(&mut self) -> &mut super::pose::Pose {
        if self.world_pose.is_none() {
            self.world_pose.set_default();
        };
        self.world_pose.as_mut().unwrap()
    }

    // Take field
    pub fn take_world_pose(&mut self) -> super::pose::Pose {
        self.world_pose.take().unwrap_or_else(|| super::pose::Pose::new())
    }

    pub fn get_world_pose(&self) -> &super::pose::Pose {
        self.world_pose.as_ref().unwrap_or_else(|| super::pose::Pose::default_instance())
    }

    // required double angle_min = 3;

    pub fn clear_angle_min(&mut self) {
        self.angle_min = ::std::option::Option::None;
    }

    pub fn has_angle_min(&self) -> bool {
        self.angle_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle_min(&mut self, v: f64) {
        self.angle_min = ::std::option::Option::Some(v);
    }

    pub fn get_angle_min(&self) -> f64 {
        self.angle_min.unwrap_or(0.)
    }

    // required double angle_max = 4;

    pub fn clear_angle_max(&mut self) {
        self.angle_max = ::std::option::Option::None;
    }

    pub fn has_angle_max(&self) -> bool {
        self.angle_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle_max(&mut self, v: f64) {
        self.angle_max = ::std::option::Option::Some(v);
    }

    pub fn get_angle_max(&self) -> f64 {
        self.angle_max.unwrap_or(0.)
    }

    // required double angle_step = 5;

    pub fn clear_angle_step(&mut self) {
        self.angle_step = ::std::option::Option::None;
    }

    pub fn has_angle_step(&self) -> bool {
        self.angle_step.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle_step(&mut self, v: f64) {
        self.angle_step = ::std::option::Option::Some(v);
    }

    pub fn get_angle_step(&self) -> f64 {
        self.angle_step.unwrap_or(0.)
    }

    // required double range_min = 6;

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

    // required double range_max = 7;

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

    // required uint32 count = 8;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    // optional double vertical_angle_min = 9;

    pub fn clear_vertical_angle_min(&mut self) {
        self.vertical_angle_min = ::std::option::Option::None;
    }

    pub fn has_vertical_angle_min(&self) -> bool {
        self.vertical_angle_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_angle_min(&mut self, v: f64) {
        self.vertical_angle_min = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_angle_min(&self) -> f64 {
        self.vertical_angle_min.unwrap_or(0.)
    }

    // optional double vertical_angle_max = 10;

    pub fn clear_vertical_angle_max(&mut self) {
        self.vertical_angle_max = ::std::option::Option::None;
    }

    pub fn has_vertical_angle_max(&self) -> bool {
        self.vertical_angle_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_angle_max(&mut self, v: f64) {
        self.vertical_angle_max = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_angle_max(&self) -> f64 {
        self.vertical_angle_max.unwrap_or(0.)
    }

    // optional double vertical_angle_step = 11;

    pub fn clear_vertical_angle_step(&mut self) {
        self.vertical_angle_step = ::std::option::Option::None;
    }

    pub fn has_vertical_angle_step(&self) -> bool {
        self.vertical_angle_step.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_angle_step(&mut self, v: f64) {
        self.vertical_angle_step = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_angle_step(&self) -> f64 {
        self.vertical_angle_step.unwrap_or(0.)
    }

    // optional uint32 vertical_count = 12;

    pub fn clear_vertical_count(&mut self) {
        self.vertical_count = ::std::option::Option::None;
    }

    pub fn has_vertical_count(&self) -> bool {
        self.vertical_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_count(&mut self, v: u32) {
        self.vertical_count = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_count(&self) -> u32 {
        self.vertical_count.unwrap_or(0)
    }

    // repeated double ranges = 13;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::std::vec::Vec<f64>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.ranges, ::std::vec::Vec::new())
    }

    pub fn get_ranges(&self) -> &[f64] {
        &self.ranges
    }

    // repeated double intensities = 14;

    pub fn clear_intensities(&mut self) {
        self.intensities.clear();
    }

    // Param is passed by value, moved
    pub fn set_intensities(&mut self, v: ::std::vec::Vec<f64>) {
        self.intensities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_intensities(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.intensities
    }

    // Take field
    pub fn take_intensities(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.intensities, ::std::vec::Vec::new())
    }

    pub fn get_intensities(&self) -> &[f64] {
        &self.intensities
    }
}

impl ::protobuf::Message for LaserScan {
    fn is_initialized(&self) -> bool {
        if self.frame.is_none() {
            return false;
        };
        if self.world_pose.is_none() {
            return false;
        };
        if self.angle_min.is_none() {
            return false;
        };
        if self.angle_max.is_none() {
            return false;
        };
        if self.angle_step.is_none() {
            return false;
        };
        if self.range_min.is_none() {
            return false;
        };
        if self.range_max.is_none() {
            return false;
        };
        if self.count.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.frame));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.world_pose));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.angle_min = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.angle_max = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.angle_step = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.range_min = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.range_max = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.count = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vertical_angle_min = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vertical_angle_max = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vertical_angle_step = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.vertical_count = ::std::option::Option::Some(tmp);
                },
                13 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.ranges));
                },
                14 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.intensities));
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
        for value in &self.frame {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.world_pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.angle_min.is_some() {
            my_size += 9;
        };
        if self.angle_max.is_some() {
            my_size += 9;
        };
        if self.angle_step.is_some() {
            my_size += 9;
        };
        if self.range_min.is_some() {
            my_size += 9;
        };
        if self.range_max.is_some() {
            my_size += 9;
        };
        for value in &self.count {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.vertical_angle_min.is_some() {
            my_size += 9;
        };
        if self.vertical_angle_max.is_some() {
            my_size += 9;
        };
        if self.vertical_angle_step.is_some() {
            my_size += 9;
        };
        for value in &self.vertical_count {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 9 * self.ranges.len() as u32;
        my_size += 9 * self.intensities.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.frame.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.world_pose.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.angle_min {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.angle_max {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.angle_step {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.range_min {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.range_max {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.count {
            try!(os.write_uint32(8, v));
        };
        if let Some(v) = self.vertical_angle_min {
            try!(os.write_double(9, v));
        };
        if let Some(v) = self.vertical_angle_max {
            try!(os.write_double(10, v));
        };
        if let Some(v) = self.vertical_angle_step {
            try!(os.write_double(11, v));
        };
        if let Some(v) = self.vertical_count {
            try!(os.write_uint32(12, v));
        };
        for v in &self.ranges {
            try!(os.write_double(13, *v));
        };
        for v in &self.intensities {
            try!(os.write_double(14, *v));
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
        ::std::any::TypeId::of::<LaserScan>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LaserScan {
    fn new() -> LaserScan {
        LaserScan::new()
    }

    fn descriptor_static(_: ::std::option::Option<LaserScan>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "frame",
                    LaserScan::has_frame,
                    LaserScan::get_frame,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "world_pose",
                    LaserScan::has_world_pose,
                    LaserScan::get_world_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "angle_min",
                    LaserScan::has_angle_min,
                    LaserScan::get_angle_min,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "angle_max",
                    LaserScan::has_angle_max,
                    LaserScan::get_angle_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "angle_step",
                    LaserScan::has_angle_step,
                    LaserScan::get_angle_step,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "range_min",
                    LaserScan::has_range_min,
                    LaserScan::get_range_min,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "range_max",
                    LaserScan::has_range_max,
                    LaserScan::get_range_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "count",
                    LaserScan::has_count,
                    LaserScan::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vertical_angle_min",
                    LaserScan::has_vertical_angle_min,
                    LaserScan::get_vertical_angle_min,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vertical_angle_max",
                    LaserScan::has_vertical_angle_max,
                    LaserScan::get_vertical_angle_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vertical_angle_step",
                    LaserScan::has_vertical_angle_step,
                    LaserScan::get_vertical_angle_step,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "vertical_count",
                    LaserScan::has_vertical_count,
                    LaserScan::get_vertical_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "ranges",
                    LaserScan::get_ranges,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "intensities",
                    LaserScan::get_intensities,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LaserScan>(
                    "LaserScan",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LaserScan {
    fn clear(&mut self) {
        self.clear_frame();
        self.clear_world_pose();
        self.clear_angle_min();
        self.clear_angle_max();
        self.clear_angle_step();
        self.clear_range_min();
        self.clear_range_max();
        self.clear_count();
        self.clear_vertical_angle_min();
        self.clear_vertical_angle_max();
        self.clear_vertical_angle_step();
        self.clear_vertical_count();
        self.clear_ranges();
        self.clear_intensities();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LaserScan {
    fn eq(&self, other: &LaserScan) -> bool {
        self.frame == other.frame &&
        self.world_pose == other.world_pose &&
        self.angle_min == other.angle_min &&
        self.angle_max == other.angle_max &&
        self.angle_step == other.angle_step &&
        self.range_min == other.range_min &&
        self.range_max == other.range_max &&
        self.count == other.count &&
        self.vertical_angle_min == other.vertical_angle_min &&
        self.vertical_angle_max == other.vertical_angle_max &&
        self.vertical_angle_step == other.vertical_angle_step &&
        self.vertical_count == other.vertical_count &&
        self.ranges == other.ranges &&
        self.intensities == other.intensities &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LaserScan {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x6c, 0x61, 0x73, 0x65, 0x72, 0x73, 0x63, 0x61, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a,
    0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc2, 0x02, 0x0a, 0x09, 0x4c,
    0x61, 0x73, 0x65, 0x72, 0x53, 0x63, 0x61, 0x6e, 0x12, 0x0d, 0x0a, 0x05, 0x66, 0x72, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x25, 0x0a, 0x0a, 0x77, 0x6f, 0x72, 0x6c, 0x64,
    0x5f, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12, 0x11,
    0x0a, 0x09, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x6d, 0x69, 0x6e, 0x18, 0x03, 0x20, 0x02, 0x28,
    0x01, 0x12, 0x11, 0x0a, 0x09, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x04,
    0x20, 0x02, 0x28, 0x01, 0x12, 0x12, 0x0a, 0x0a, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x73, 0x74,
    0x65, 0x70, 0x18, 0x05, 0x20, 0x02, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x5f, 0x6d, 0x69, 0x6e, 0x18, 0x06, 0x20, 0x02, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x07, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0d,
    0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x08, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x1a, 0x0a,
    0x12, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f,
    0x6d, 0x69, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1a, 0x0a, 0x12, 0x76, 0x65, 0x72,
    0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x6d, 0x61, 0x78, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1b, 0x0a, 0x13, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61,
    0x6c, 0x5f, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x73, 0x74, 0x65, 0x70, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x01, 0x12, 0x16, 0x0a, 0x0e, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x73, 0x18, 0x0d, 0x20, 0x03, 0x28, 0x01, 0x12, 0x13, 0x0a, 0x0b, 0x69, 0x6e,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x74, 0x69, 0x65, 0x73, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x01, 0x4a,
    0xfb, 0x07, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0b, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0c, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x28,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x0d, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x0e, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0e, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f,
    0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x28, 0x29,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x10, 0x02, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x10, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x10, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x11,
    0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x11, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x11, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x12, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04,
    0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03,
    0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x12, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x12, 0x28, 0x29, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x13, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x13, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12,
    0x03, 0x13, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x14, 0x02,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x14, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x14, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0a, 0x12, 0x03, 0x15, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12,
    0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x15,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x15, 0x12, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x15, 0x28, 0x2a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x16, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0b, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01,
    0x12, 0x03, 0x16, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03,
    0x16, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x18, 0x02, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x18, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x18, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0c, 0x03, 0x12, 0x03, 0x18, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d,
    0x12, 0x03, 0x19, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x03,
    0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x19, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x19, 0x12, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x19, 0x28, 0x2a,
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
