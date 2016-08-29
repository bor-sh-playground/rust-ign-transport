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
pub struct WorldStatistics {
    // message fields
    sim_time: ::protobuf::SingularPtrField<super::time::Time>,
    pause_time: ::protobuf::SingularPtrField<super::time::Time>,
    real_time: ::protobuf::SingularPtrField<super::time::Time>,
    paused: ::std::option::Option<bool>,
    iterations: ::std::option::Option<u64>,
    model_count: ::std::option::Option<i32>,
    log_playback_stats: ::protobuf::SingularPtrField<super::log_playback_stats::LogPlaybackStatistics>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WorldStatistics {}

impl WorldStatistics {
    pub fn new() -> WorldStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WorldStatistics {
        static mut instance: ::protobuf::lazy::Lazy<WorldStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WorldStatistics,
        };
        unsafe {
            instance.get(|| {
                WorldStatistics {
                    sim_time: ::protobuf::SingularPtrField::none(),
                    pause_time: ::protobuf::SingularPtrField::none(),
                    real_time: ::protobuf::SingularPtrField::none(),
                    paused: ::std::option::Option::None,
                    iterations: ::std::option::Option::None,
                    model_count: ::std::option::Option::None,
                    log_playback_stats: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Time sim_time = 2;

    pub fn clear_sim_time(&mut self) {
        self.sim_time.clear();
    }

    pub fn has_sim_time(&self) -> bool {
        self.sim_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sim_time(&mut self, v: super::time::Time) {
        self.sim_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sim_time(&mut self) -> &mut super::time::Time {
        if self.sim_time.is_none() {
            self.sim_time.set_default();
        };
        self.sim_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_sim_time(&mut self) -> super::time::Time {
        self.sim_time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_sim_time(&self) -> &super::time::Time {
        self.sim_time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // required .gazebo.msgs.Time pause_time = 3;

    pub fn clear_pause_time(&mut self) {
        self.pause_time.clear();
    }

    pub fn has_pause_time(&self) -> bool {
        self.pause_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pause_time(&mut self, v: super::time::Time) {
        self.pause_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pause_time(&mut self) -> &mut super::time::Time {
        if self.pause_time.is_none() {
            self.pause_time.set_default();
        };
        self.pause_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_pause_time(&mut self) -> super::time::Time {
        self.pause_time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_pause_time(&self) -> &super::time::Time {
        self.pause_time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // required .gazebo.msgs.Time real_time = 4;

    pub fn clear_real_time(&mut self) {
        self.real_time.clear();
    }

    pub fn has_real_time(&self) -> bool {
        self.real_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_real_time(&mut self, v: super::time::Time) {
        self.real_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_real_time(&mut self) -> &mut super::time::Time {
        if self.real_time.is_none() {
            self.real_time.set_default();
        };
        self.real_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_real_time(&mut self) -> super::time::Time {
        self.real_time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_real_time(&self) -> &super::time::Time {
        self.real_time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // required bool paused = 5;

    pub fn clear_paused(&mut self) {
        self.paused = ::std::option::Option::None;
    }

    pub fn has_paused(&self) -> bool {
        self.paused.is_some()
    }

    // Param is passed by value, moved
    pub fn set_paused(&mut self, v: bool) {
        self.paused = ::std::option::Option::Some(v);
    }

    pub fn get_paused(&self) -> bool {
        self.paused.unwrap_or(false)
    }

    // required uint64 iterations = 6;

    pub fn clear_iterations(&mut self) {
        self.iterations = ::std::option::Option::None;
    }

    pub fn has_iterations(&self) -> bool {
        self.iterations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iterations(&mut self, v: u64) {
        self.iterations = ::std::option::Option::Some(v);
    }

    pub fn get_iterations(&self) -> u64 {
        self.iterations.unwrap_or(0)
    }

    // optional int32 model_count = 7;

    pub fn clear_model_count(&mut self) {
        self.model_count = ::std::option::Option::None;
    }

    pub fn has_model_count(&self) -> bool {
        self.model_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_count(&mut self, v: i32) {
        self.model_count = ::std::option::Option::Some(v);
    }

    pub fn get_model_count(&self) -> i32 {
        self.model_count.unwrap_or(0)
    }

    // optional .gazebo.msgs.LogPlaybackStatistics log_playback_stats = 8;

    pub fn clear_log_playback_stats(&mut self) {
        self.log_playback_stats.clear();
    }

    pub fn has_log_playback_stats(&self) -> bool {
        self.log_playback_stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_log_playback_stats(&mut self, v: super::log_playback_stats::LogPlaybackStatistics) {
        self.log_playback_stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_playback_stats(&mut self) -> &mut super::log_playback_stats::LogPlaybackStatistics {
        if self.log_playback_stats.is_none() {
            self.log_playback_stats.set_default();
        };
        self.log_playback_stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_log_playback_stats(&mut self) -> super::log_playback_stats::LogPlaybackStatistics {
        self.log_playback_stats.take().unwrap_or_else(|| super::log_playback_stats::LogPlaybackStatistics::new())
    }

    pub fn get_log_playback_stats(&self) -> &super::log_playback_stats::LogPlaybackStatistics {
        self.log_playback_stats.as_ref().unwrap_or_else(|| super::log_playback_stats::LogPlaybackStatistics::default_instance())
    }
}

impl ::protobuf::Message for WorldStatistics {
    fn is_initialized(&self) -> bool {
        if self.sim_time.is_none() {
            return false;
        };
        if self.pause_time.is_none() {
            return false;
        };
        if self.real_time.is_none() {
            return false;
        };
        if self.paused.is_none() {
            return false;
        };
        if self.iterations.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sim_time));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pause_time));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.real_time));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.paused = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.iterations = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.model_count = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.log_playback_stats));
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
        for value in &self.sim_time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pause_time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.real_time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.paused.is_some() {
            my_size += 2;
        };
        for value in &self.iterations {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.model_count {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.log_playback_stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sim_time.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pause_time.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.real_time.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.paused {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.iterations {
            try!(os.write_uint64(6, v));
        };
        if let Some(v) = self.model_count {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.log_playback_stats.as_ref() {
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
        ::std::any::TypeId::of::<WorldStatistics>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WorldStatistics {
    fn new() -> WorldStatistics {
        WorldStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<WorldStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "sim_time",
                    WorldStatistics::has_sim_time,
                    WorldStatistics::get_sim_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pause_time",
                    WorldStatistics::has_pause_time,
                    WorldStatistics::get_pause_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "real_time",
                    WorldStatistics::has_real_time,
                    WorldStatistics::get_real_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "paused",
                    WorldStatistics::has_paused,
                    WorldStatistics::get_paused,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "iterations",
                    WorldStatistics::has_iterations,
                    WorldStatistics::get_iterations,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "model_count",
                    WorldStatistics::has_model_count,
                    WorldStatistics::get_model_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "log_playback_stats",
                    WorldStatistics::has_log_playback_stats,
                    WorldStatistics::get_log_playback_stats,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WorldStatistics>(
                    "WorldStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WorldStatistics {
    fn clear(&mut self) {
        self.clear_sim_time();
        self.clear_pause_time();
        self.clear_real_time();
        self.clear_paused();
        self.clear_iterations();
        self.clear_model_count();
        self.clear_log_playback_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WorldStatistics {
    fn eq(&self, other: &WorldStatistics) -> bool {
        self.sim_time == other.sim_time &&
        self.pause_time == other.pause_time &&
        self.real_time == other.real_time &&
        self.paused == other.paused &&
        self.iterations == other.iterations &&
        self.model_count == other.model_count &&
        self.log_playback_stats == other.log_playback_stats &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WorldStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x1a, 0x18, 0x6c, 0x6f, 0x67, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x73,
    0x74, 0x61, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x74, 0x69, 0x6d, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xfc, 0x01, 0x0a, 0x0f, 0x57, 0x6f, 0x72, 0x6c, 0x64,
    0x53, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x12, 0x23, 0x0a, 0x08, 0x73, 0x69,
    0x6d, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12,
    0x25, 0x0a, 0x0a, 0x70, 0x61, 0x75, 0x73, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x24, 0x0a, 0x09, 0x72, 0x65, 0x61, 0x6c, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65,
    0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x0e, 0x0a, 0x06,
    0x70, 0x61, 0x75, 0x73, 0x65, 0x64, 0x18, 0x05, 0x20, 0x02, 0x28, 0x08, 0x12, 0x12, 0x0a, 0x0a,
    0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x06, 0x20, 0x02, 0x28, 0x04,
    0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x05, 0x12, 0x3e, 0x0a, 0x12, 0x6c, 0x6f, 0x67, 0x5f, 0x70, 0x6c, 0x61,
    0x79, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x22, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x4c, 0x6f, 0x67, 0x50, 0x6c, 0x61, 0x79, 0x62, 0x61, 0x63, 0x6b, 0x53, 0x74, 0x61, 0x74, 0x69,
    0x73, 0x74, 0x69, 0x63, 0x73, 0x4a, 0xa3, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x12, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x06, 0x07, 0x21, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x07, 0x13,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0b, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x11, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x36, 0x37, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0c, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c,
    0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x38, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0d, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x0e, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x0f, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x36,
    0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x10, 0x02, 0x38, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x10, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03,
    0x11, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x11, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x11, 0x0b, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x11, 0x21, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x11, 0x36, 0x37,
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
