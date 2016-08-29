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
pub struct PropagationParticle {
    // message fields
    x: ::std::option::Option<f64>,
    y: ::std::option::Option<f64>,
    signal_level: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PropagationParticle {}

impl PropagationParticle {
    pub fn new() -> PropagationParticle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PropagationParticle {
        static mut instance: ::protobuf::lazy::Lazy<PropagationParticle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PropagationParticle,
        };
        unsafe {
            instance.get(|| {
                PropagationParticle {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    signal_level: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f64) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f64 {
        self.x.unwrap_or(0.)
    }

    // required double y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f64) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f64 {
        self.y.unwrap_or(0.)
    }

    // required double signal_level = 3;

    pub fn clear_signal_level(&mut self) {
        self.signal_level = ::std::option::Option::None;
    }

    pub fn has_signal_level(&self) -> bool {
        self.signal_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signal_level(&mut self, v: f64) {
        self.signal_level = ::std::option::Option::Some(v);
    }

    pub fn get_signal_level(&self) -> f64 {
        self.signal_level.unwrap_or(0.)
    }
}

impl ::protobuf::Message for PropagationParticle {
    fn is_initialized(&self) -> bool {
        if self.x.is_none() {
            return false;
        };
        if self.y.is_none() {
            return false;
        };
        if self.signal_level.is_none() {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.signal_level = ::std::option::Option::Some(tmp);
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
        if self.x.is_some() {
            my_size += 9;
        };
        if self.y.is_some() {
            my_size += 9;
        };
        if self.signal_level.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.y {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.signal_level {
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
        ::std::any::TypeId::of::<PropagationParticle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PropagationParticle {
    fn new() -> PropagationParticle {
        PropagationParticle::new()
    }

    fn descriptor_static(_: ::std::option::Option<PropagationParticle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "x",
                    PropagationParticle::has_x,
                    PropagationParticle::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "y",
                    PropagationParticle::has_y,
                    PropagationParticle::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "signal_level",
                    PropagationParticle::has_signal_level,
                    PropagationParticle::get_signal_level,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PropagationParticle>(
                    "PropagationParticle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PropagationParticle {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_signal_level();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PropagationParticle {
    fn eq(&self, other: &PropagationParticle) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.signal_level == other.signal_level &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PropagationParticle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x70, 0x72, 0x6f, 0x70, 0x61, 0x67, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0x41, 0x0a, 0x13, 0x50, 0x72, 0x6f,
    0x70, 0x61, 0x67, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65,
    0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x12, 0x09, 0x0a, 0x01, 0x79,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c,
    0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x4a, 0xf9, 0x01, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x0c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08,
    0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x09, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x16, 0x17, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0a, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0a, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x21, 0x22,
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
