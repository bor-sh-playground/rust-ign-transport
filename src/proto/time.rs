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
pub struct Time {
    // message fields
    sec: ::std::option::Option<i32>,
    nsec: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Time {}

impl Time {
    pub fn new() -> Time {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Time {
        static mut instance: ::protobuf::lazy::Lazy<Time> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Time,
        };
        unsafe {
            instance.get(|| {
                Time {
                    sec: ::std::option::Option::None,
                    nsec: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 sec = 1;

    pub fn clear_sec(&mut self) {
        self.sec = ::std::option::Option::None;
    }

    pub fn has_sec(&self) -> bool {
        self.sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sec(&mut self, v: i32) {
        self.sec = ::std::option::Option::Some(v);
    }

    pub fn get_sec(&self) -> i32 {
        self.sec.unwrap_or(0)
    }

    // required int32 nsec = 2;

    pub fn clear_nsec(&mut self) {
        self.nsec = ::std::option::Option::None;
    }

    pub fn has_nsec(&self) -> bool {
        self.nsec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nsec(&mut self, v: i32) {
        self.nsec = ::std::option::Option::Some(v);
    }

    pub fn get_nsec(&self) -> i32 {
        self.nsec.unwrap_or(0)
    }
}

impl ::protobuf::Message for Time {
    fn is_initialized(&self) -> bool {
        if self.sec.is_none() {
            return false;
        };
        if self.nsec.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.sec = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.nsec = ::std::option::Option::Some(tmp);
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
        for value in &self.sec {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.nsec {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sec {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.nsec {
            try!(os.write_int32(2, v));
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
        ::std::any::TypeId::of::<Time>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Time {
    fn new() -> Time {
        Time::new()
    }

    fn descriptor_static(_: ::std::option::Option<Time>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sec",
                    Time::has_sec,
                    Time::get_sec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "nsec",
                    Time::has_nsec,
                    Time::get_nsec,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Time>(
                    "Time",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Time {
    fn clear(&mut self) {
        self.clear_sec();
        self.clear_nsec();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.sec == other.sec &&
        self.nsec == other.nsec &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Time {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0x21, 0x0a, 0x04, 0x54, 0x69, 0x6d,
    0x65, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x65, 0x63, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0c,
    0x0a, 0x04, 0x6e, 0x73, 0x65, 0x63, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x4a, 0xb4, 0x01, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x0b, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08,
    0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x09, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x11, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x17, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0a, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0a, 0x18, 0x19,
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
