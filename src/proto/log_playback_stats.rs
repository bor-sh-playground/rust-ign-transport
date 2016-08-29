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
pub struct LogPlaybackStatistics {
    // message fields
    start_time: ::protobuf::SingularPtrField<super::time::Time>,
    end_time: ::protobuf::SingularPtrField<super::time::Time>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogPlaybackStatistics {}

impl LogPlaybackStatistics {
    pub fn new() -> LogPlaybackStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogPlaybackStatistics {
        static mut instance: ::protobuf::lazy::Lazy<LogPlaybackStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogPlaybackStatistics,
        };
        unsafe {
            instance.get(|| {
                LogPlaybackStatistics {
                    start_time: ::protobuf::SingularPtrField::none(),
                    end_time: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Time start_time = 1;

    pub fn clear_start_time(&mut self) {
        self.start_time.clear();
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: super::time::Time) {
        self.start_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_time(&mut self) -> &mut super::time::Time {
        if self.start_time.is_none() {
            self.start_time.set_default();
        };
        self.start_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_time(&mut self) -> super::time::Time {
        self.start_time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_start_time(&self) -> &super::time::Time {
        self.start_time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // required .gazebo.msgs.Time end_time = 2;

    pub fn clear_end_time(&mut self) {
        self.end_time.clear();
    }

    pub fn has_end_time(&self) -> bool {
        self.end_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_time(&mut self, v: super::time::Time) {
        self.end_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_time(&mut self) -> &mut super::time::Time {
        if self.end_time.is_none() {
            self.end_time.set_default();
        };
        self.end_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_time(&mut self) -> super::time::Time {
        self.end_time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_end_time(&self) -> &super::time::Time {
        self.end_time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }
}

impl ::protobuf::Message for LogPlaybackStatistics {
    fn is_initialized(&self) -> bool {
        if self.start_time.is_none() {
            return false;
        };
        if self.end_time.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start_time));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end_time));
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
        for value in &self.start_time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.end_time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_time.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.end_time.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<LogPlaybackStatistics>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogPlaybackStatistics {
    fn new() -> LogPlaybackStatistics {
        LogPlaybackStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogPlaybackStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "start_time",
                    LogPlaybackStatistics::has_start_time,
                    LogPlaybackStatistics::get_start_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "end_time",
                    LogPlaybackStatistics::has_end_time,
                    LogPlaybackStatistics::get_end_time,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogPlaybackStatistics>(
                    "LogPlaybackStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogPlaybackStatistics {
    fn clear(&mut self) {
        self.clear_start_time();
        self.clear_end_time();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LogPlaybackStatistics {
    fn eq(&self, other: &LogPlaybackStatistics) -> bool {
        self.start_time == other.start_time &&
        self.end_time == other.end_time &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LogPlaybackStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x6c, 0x6f, 0x67, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x73,
    0x74, 0x61, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65,
    0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x63, 0x0a, 0x15, 0x4c, 0x6f, 0x67, 0x50, 0x6c, 0x61, 0x79, 0x62, 0x61,
    0x63, 0x6b, 0x53, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x12, 0x25, 0x0a, 0x0a,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54,
    0x69, 0x6d, 0x65, 0x12, 0x23, 0x0a, 0x08, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x4a, 0xf1, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x10, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x09, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x1d,
    0x0a, 0x25, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x1f, 0x1a, 0x18, 0x2f,
    0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x10,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x1d, 0x1e, 0x0a,
    0x23, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x1f, 0x1a, 0x16, 0x2f, 0x20,
    0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x74,
    0x69, 0x6d, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0f,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0f, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x10, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x1d, 0x1e,
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
