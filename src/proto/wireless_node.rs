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
pub struct WirelessNode {
    // message fields
    essid: ::protobuf::SingularField<::std::string::String>,
    frequency: ::std::option::Option<f64>,
    signal_level: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WirelessNode {}

impl WirelessNode {
    pub fn new() -> WirelessNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WirelessNode {
        static mut instance: ::protobuf::lazy::Lazy<WirelessNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WirelessNode,
        };
        unsafe {
            instance.get(|| {
                WirelessNode {
                    essid: ::protobuf::SingularField::none(),
                    frequency: ::std::option::Option::None,
                    signal_level: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string essid = 1;

    pub fn clear_essid(&mut self) {
        self.essid.clear();
    }

    pub fn has_essid(&self) -> bool {
        self.essid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_essid(&mut self, v: ::std::string::String) {
        self.essid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_essid(&mut self) -> &mut ::std::string::String {
        if self.essid.is_none() {
            self.essid.set_default();
        };
        self.essid.as_mut().unwrap()
    }

    // Take field
    pub fn take_essid(&mut self) -> ::std::string::String {
        self.essid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_essid(&self) -> &str {
        match self.essid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required double frequency = 2;

    pub fn clear_frequency(&mut self) {
        self.frequency = ::std::option::Option::None;
    }

    pub fn has_frequency(&self) -> bool {
        self.frequency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frequency(&mut self, v: f64) {
        self.frequency = ::std::option::Option::Some(v);
    }

    pub fn get_frequency(&self) -> f64 {
        self.frequency.unwrap_or(0.)
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

impl ::protobuf::Message for WirelessNode {
    fn is_initialized(&self) -> bool {
        if self.essid.is_none() {
            return false;
        };
        if self.frequency.is_none() {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.essid));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.frequency = ::std::option::Option::Some(tmp);
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
        for value in &self.essid {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.frequency.is_some() {
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
        if let Some(v) = self.essid.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.frequency {
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
        ::std::any::TypeId::of::<WirelessNode>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WirelessNode {
    fn new() -> WirelessNode {
        WirelessNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<WirelessNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "essid",
                    WirelessNode::has_essid,
                    WirelessNode::get_essid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "frequency",
                    WirelessNode::has_frequency,
                    WirelessNode::get_frequency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "signal_level",
                    WirelessNode::has_signal_level,
                    WirelessNode::get_signal_level,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WirelessNode>(
                    "WirelessNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WirelessNode {
    fn clear(&mut self) {
        self.clear_essid();
        self.clear_frequency();
        self.clear_signal_level();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WirelessNode {
    fn eq(&self, other: &WirelessNode) -> bool {
        self.essid == other.essid &&
        self.frequency == other.frequency &&
        self.signal_level == other.signal_level &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WirelessNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x77, 0x69, 0x72, 0x65, 0x6c, 0x65, 0x73, 0x73, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x22, 0x46, 0x0a, 0x0c, 0x57, 0x69, 0x72, 0x65, 0x6c, 0x65, 0x73, 0x73, 0x4e, 0x6f,
    0x64, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x65, 0x73, 0x73, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x01, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x5f, 0x6c,
    0x65, 0x76, 0x65, 0x6c, 0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x4a, 0xf9, 0x01, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x0c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x09, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0a, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x21,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x0b, 0x21, 0x22,
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
