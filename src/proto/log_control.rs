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
pub struct LogControl {
    // message fields
    start: ::std::option::Option<bool>,
    stop: ::std::option::Option<bool>,
    paused: ::std::option::Option<bool>,
    base_path: ::protobuf::SingularField<::std::string::String>,
    encoding: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogControl {}

impl LogControl {
    pub fn new() -> LogControl {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogControl {
        static mut instance: ::protobuf::lazy::Lazy<LogControl> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogControl,
        };
        unsafe {
            instance.get(|| {
                LogControl {
                    start: ::std::option::Option::None,
                    stop: ::std::option::Option::None,
                    paused: ::std::option::Option::None,
                    base_path: ::protobuf::SingularField::none(),
                    encoding: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: bool) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> bool {
        self.start.unwrap_or(false)
    }

    // optional bool stop = 2;

    pub fn clear_stop(&mut self) {
        self.stop = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        self.stop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: bool) {
        self.stop = ::std::option::Option::Some(v);
    }

    pub fn get_stop(&self) -> bool {
        self.stop.unwrap_or(false)
    }

    // optional bool paused = 3;

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

    // optional string base_path = 4;

    pub fn clear_base_path(&mut self) {
        self.base_path.clear();
    }

    pub fn has_base_path(&self) -> bool {
        self.base_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_path(&mut self, v: ::std::string::String) {
        self.base_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base_path(&mut self) -> &mut ::std::string::String {
        if self.base_path.is_none() {
            self.base_path.set_default();
        };
        self.base_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_base_path(&mut self) -> ::std::string::String {
        self.base_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_base_path(&self) -> &str {
        match self.base_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string encoding = 5;

    pub fn clear_encoding(&mut self) {
        self.encoding.clear();
    }

    pub fn has_encoding(&self) -> bool {
        self.encoding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encoding(&mut self, v: ::std::string::String) {
        self.encoding = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encoding(&mut self) -> &mut ::std::string::String {
        if self.encoding.is_none() {
            self.encoding.set_default();
        };
        self.encoding.as_mut().unwrap()
    }

    // Take field
    pub fn take_encoding(&mut self) -> ::std::string::String {
        self.encoding.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_encoding(&self) -> &str {
        match self.encoding.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for LogControl {
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
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stop = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.paused = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.base_path));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.encoding));
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
        if self.start.is_some() {
            my_size += 2;
        };
        if self.stop.is_some() {
            my_size += 2;
        };
        if self.paused.is_some() {
            my_size += 2;
        };
        for value in &self.base_path {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.encoding {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.stop {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.paused {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.base_path.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.encoding.as_ref() {
            try!(os.write_string(5, &v));
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
        ::std::any::TypeId::of::<LogControl>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogControl {
    fn new() -> LogControl {
        LogControl::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogControl>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "start",
                    LogControl::has_start,
                    LogControl::get_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stop",
                    LogControl::has_stop,
                    LogControl::get_stop,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "paused",
                    LogControl::has_paused,
                    LogControl::get_paused,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "base_path",
                    LogControl::has_base_path,
                    LogControl::get_base_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "encoding",
                    LogControl::has_encoding,
                    LogControl::get_encoding,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogControl>(
                    "LogControl",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogControl {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_stop();
        self.clear_paused();
        self.clear_base_path();
        self.clear_encoding();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LogControl {
    fn eq(&self, other: &LogControl) -> bool {
        self.start == other.start &&
        self.stop == other.stop &&
        self.paused == other.paused &&
        self.base_path == other.base_path &&
        self.encoding == other.encoding &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LogControl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x6c, 0x6f, 0x67, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x22, 0x5e, 0x0a, 0x0a, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x12, 0x0d,
    0x0a, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0c, 0x0a,
    0x04, 0x73, 0x74, 0x6f, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0e, 0x0a, 0x06, 0x70,
    0x61, 0x75, 0x73, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x12, 0x11, 0x0a, 0x09, 0x62,
    0x61, 0x73, 0x65, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12, 0x10,
    0x0a, 0x08, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09,
    0x4a, 0x83, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0d,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x08, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x08, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08,
    0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x02, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x09, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0a, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0a,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x10, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x0b, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0b, 0x22,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x0c, 0x22, 0x23,
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
