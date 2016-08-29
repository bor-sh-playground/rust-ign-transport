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
pub struct TopicInfo {
    // message fields
    msg_type: ::protobuf::SingularField<::std::string::String>,
    publisher: ::protobuf::RepeatedField<super::publish::Publish>,
    subscriber: ::protobuf::RepeatedField<super::subscribe::Subscribe>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopicInfo {}

impl TopicInfo {
    pub fn new() -> TopicInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopicInfo {
        static mut instance: ::protobuf::lazy::Lazy<TopicInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopicInfo,
        };
        unsafe {
            instance.get(|| {
                TopicInfo {
                    msg_type: ::protobuf::SingularField::none(),
                    publisher: ::protobuf::RepeatedField::new(),
                    subscriber: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type.clear();
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: ::std::string::String) {
        self.msg_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_type(&mut self) -> &mut ::std::string::String {
        if self.msg_type.is_none() {
            self.msg_type.set_default();
        };
        self.msg_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg_type(&mut self) -> ::std::string::String {
        self.msg_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg_type(&self) -> &str {
        match self.msg_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .gazebo.msgs.Publish publisher = 2;

    pub fn clear_publisher(&mut self) {
        self.publisher.clear();
    }

    // Param is passed by value, moved
    pub fn set_publisher(&mut self, v: ::protobuf::RepeatedField<super::publish::Publish>) {
        self.publisher = v;
    }

    // Mutable pointer to the field.
    pub fn mut_publisher(&mut self) -> &mut ::protobuf::RepeatedField<super::publish::Publish> {
        &mut self.publisher
    }

    // Take field
    pub fn take_publisher(&mut self) -> ::protobuf::RepeatedField<super::publish::Publish> {
        ::std::mem::replace(&mut self.publisher, ::protobuf::RepeatedField::new())
    }

    pub fn get_publisher(&self) -> &[super::publish::Publish] {
        &self.publisher
    }

    // repeated .gazebo.msgs.Subscribe subscriber = 3;

    pub fn clear_subscriber(&mut self) {
        self.subscriber.clear();
    }

    // Param is passed by value, moved
    pub fn set_subscriber(&mut self, v: ::protobuf::RepeatedField<super::subscribe::Subscribe>) {
        self.subscriber = v;
    }

    // Mutable pointer to the field.
    pub fn mut_subscriber(&mut self) -> &mut ::protobuf::RepeatedField<super::subscribe::Subscribe> {
        &mut self.subscriber
    }

    // Take field
    pub fn take_subscriber(&mut self) -> ::protobuf::RepeatedField<super::subscribe::Subscribe> {
        ::std::mem::replace(&mut self.subscriber, ::protobuf::RepeatedField::new())
    }

    pub fn get_subscriber(&self) -> &[super::subscribe::Subscribe] {
        &self.subscriber
    }
}

impl ::protobuf::Message for TopicInfo {
    fn is_initialized(&self) -> bool {
        if self.msg_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg_type));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.publisher));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.subscriber));
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
        for value in &self.msg_type {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.publisher {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.subscriber {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_type.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.publisher {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.subscriber {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<TopicInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TopicInfo {
    fn new() -> TopicInfo {
        TopicInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopicInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "msg_type",
                    TopicInfo::has_msg_type,
                    TopicInfo::get_msg_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "publisher",
                    TopicInfo::get_publisher,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "subscriber",
                    TopicInfo::get_subscriber,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopicInfo>(
                    "TopicInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopicInfo {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_publisher();
        self.clear_subscriber();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TopicInfo {
    fn eq(&self, other: &TopicInfo) -> bool {
        self.msg_type == other.msg_type &&
        self.publisher == other.publisher &&
        self.subscriber == other.subscriber &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TopicInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x74, 0x6f, 0x70, 0x69, 0x63, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a,
    0x0d, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f,
    0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x72, 0x0a, 0x09, 0x54, 0x6f, 0x70, 0x69, 0x63, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x10, 0x0a, 0x08,
    0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x27,
    0x0a, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x65, 0x72, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x14, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x12, 0x2a, 0x0a, 0x0a, 0x73, 0x75, 0x62, 0x73, 0x63,
    0x72, 0x69, 0x62, 0x65, 0x72, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72,
    0x69, 0x62, 0x65, 0x4a, 0x8f, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0f, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x07, 0x07, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x18, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0a, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0c, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x0d, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d,
    0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x02, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0e, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0e, 0x15, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0e, 0x22, 0x23,
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
