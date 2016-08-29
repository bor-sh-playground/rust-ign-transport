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
pub struct PID {
    // message fields
    target: ::std::option::Option<f64>,
    p_gain: ::std::option::Option<f64>,
    i_gain: ::std::option::Option<f64>,
    d_gain: ::std::option::Option<f64>,
    i_max: ::std::option::Option<f64>,
    i_min: ::std::option::Option<f64>,
    limit: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PID {}

impl PID {
    pub fn new() -> PID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PID {
        static mut instance: ::protobuf::lazy::Lazy<PID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PID,
        };
        unsafe {
            instance.get(|| {
                PID {
                    target: ::std::option::Option::None,
                    p_gain: ::std::option::Option::None,
                    i_gain: ::std::option::Option::None,
                    d_gain: ::std::option::Option::None,
                    i_max: ::std::option::Option::None,
                    i_min: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double target = 1;

    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: f64) {
        self.target = ::std::option::Option::Some(v);
    }

    pub fn get_target(&self) -> f64 {
        self.target.unwrap_or(0f64)
    }

    // optional double p_gain = 2;

    pub fn clear_p_gain(&mut self) {
        self.p_gain = ::std::option::Option::None;
    }

    pub fn has_p_gain(&self) -> bool {
        self.p_gain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p_gain(&mut self, v: f64) {
        self.p_gain = ::std::option::Option::Some(v);
    }

    pub fn get_p_gain(&self) -> f64 {
        self.p_gain.unwrap_or(0f64)
    }

    // optional double i_gain = 3;

    pub fn clear_i_gain(&mut self) {
        self.i_gain = ::std::option::Option::None;
    }

    pub fn has_i_gain(&self) -> bool {
        self.i_gain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_i_gain(&mut self, v: f64) {
        self.i_gain = ::std::option::Option::Some(v);
    }

    pub fn get_i_gain(&self) -> f64 {
        self.i_gain.unwrap_or(0f64)
    }

    // optional double d_gain = 4;

    pub fn clear_d_gain(&mut self) {
        self.d_gain = ::std::option::Option::None;
    }

    pub fn has_d_gain(&self) -> bool {
        self.d_gain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_d_gain(&mut self, v: f64) {
        self.d_gain = ::std::option::Option::Some(v);
    }

    pub fn get_d_gain(&self) -> f64 {
        self.d_gain.unwrap_or(0f64)
    }

    // optional double i_max = 5;

    pub fn clear_i_max(&mut self) {
        self.i_max = ::std::option::Option::None;
    }

    pub fn has_i_max(&self) -> bool {
        self.i_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_i_max(&mut self, v: f64) {
        self.i_max = ::std::option::Option::Some(v);
    }

    pub fn get_i_max(&self) -> f64 {
        self.i_max.unwrap_or(0f64)
    }

    // optional double i_min = 6;

    pub fn clear_i_min(&mut self) {
        self.i_min = ::std::option::Option::None;
    }

    pub fn has_i_min(&self) -> bool {
        self.i_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_i_min(&mut self, v: f64) {
        self.i_min = ::std::option::Option::Some(v);
    }

    pub fn get_i_min(&self) -> f64 {
        self.i_min.unwrap_or(0f64)
    }

    // optional double limit = 7;

    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None;
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: f64) {
        self.limit = ::std::option::Option::Some(v);
    }

    pub fn get_limit(&self) -> f64 {
        self.limit.unwrap_or(0f64)
    }
}

impl ::protobuf::Message for PID {
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
                    self.target = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.p_gain = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.i_gain = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.d_gain = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.i_max = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.i_min = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.limit = ::std::option::Option::Some(tmp);
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
        if self.target.is_some() {
            my_size += 9;
        };
        if self.p_gain.is_some() {
            my_size += 9;
        };
        if self.i_gain.is_some() {
            my_size += 9;
        };
        if self.d_gain.is_some() {
            my_size += 9;
        };
        if self.i_max.is_some() {
            my_size += 9;
        };
        if self.i_min.is_some() {
            my_size += 9;
        };
        if self.limit.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.target {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.p_gain {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.i_gain {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.d_gain {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.i_max {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.i_min {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.limit {
            try!(os.write_double(7, v));
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
        ::std::any::TypeId::of::<PID>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PID {
    fn new() -> PID {
        PID::new()
    }

    fn descriptor_static(_: ::std::option::Option<PID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "target",
                    PID::has_target,
                    PID::get_target,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "p_gain",
                    PID::has_p_gain,
                    PID::get_p_gain,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "i_gain",
                    PID::has_i_gain,
                    PID::get_i_gain,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "d_gain",
                    PID::has_d_gain,
                    PID::get_d_gain,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "i_max",
                    PID::has_i_max,
                    PID::get_i_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "i_min",
                    PID::has_i_min,
                    PID::get_i_min,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "limit",
                    PID::has_limit,
                    PID::get_limit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PID>(
                    "PID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PID {
    fn clear(&mut self) {
        self.clear_target();
        self.clear_p_gain();
        self.clear_i_gain();
        self.clear_d_gain();
        self.clear_i_max();
        self.clear_i_min();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PID {
    fn eq(&self, other: &PID) -> bool {
        self.target == other.target &&
        self.p_gain == other.p_gain &&
        self.i_gain == other.i_gain &&
        self.d_gain == other.d_gain &&
        self.i_max == other.i_max &&
        self.i_min == other.i_min &&
        self.limit == other.limit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x70, 0x69, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0x87, 0x01, 0x0a, 0x03, 0x50, 0x49, 0x44,
    0x12, 0x11, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01,
    0x3a, 0x01, 0x30, 0x12, 0x11, 0x0a, 0x06, 0x70, 0x5f, 0x67, 0x61, 0x69, 0x6e, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x01, 0x3a, 0x01, 0x30, 0x12, 0x11, 0x0a, 0x06, 0x69, 0x5f, 0x67, 0x61, 0x69, 0x6e,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x01, 0x30, 0x12, 0x11, 0x0a, 0x06, 0x64, 0x5f, 0x67,
    0x61, 0x69, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x01, 0x30, 0x12, 0x10, 0x0a, 0x05,
    0x69, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x01, 0x30, 0x12, 0x10,
    0x0a, 0x05, 0x69, 0x5f, 0x6d, 0x69, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x01, 0x30,
    0x12, 0x10, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01, 0x3a,
    0x01, 0x30, 0x4a, 0xd1, 0x05, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x10, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07,
    0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x09, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x09, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x09,
    0x1c, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x07, 0x12, 0x03, 0x09, 0x25, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0a, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03,
    0x0a, 0x1c, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x07, 0x12, 0x03, 0x0a, 0x25,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x0b, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12,
    0x03, 0x0b, 0x1c, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x07, 0x12, 0x03, 0x0b,
    0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x0c, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x08,
    0x12, 0x03, 0x0c, 0x1c, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x07, 0x12, 0x03,
    0x0c, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x08, 0x12, 0x03, 0x0d, 0x1c, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x07, 0x12,
    0x03, 0x0d, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x02,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0e, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x08, 0x12, 0x03, 0x0e, 0x1c, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x07,
    0x12, 0x03, 0x0e, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0f,
    0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0f, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x08, 0x12, 0x03, 0x0f, 0x1c, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x07, 0x12, 0x03, 0x0f, 0x25, 0x28,
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
