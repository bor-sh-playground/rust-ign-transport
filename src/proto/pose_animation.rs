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
pub struct PoseAnimation {
    // message fields
    model_name: ::protobuf::SingularField<::std::string::String>,
    model_id: ::std::option::Option<u32>,
    pose: ::protobuf::RepeatedField<super::pose::Pose>,
    time: ::protobuf::RepeatedField<super::time::Time>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoseAnimation {}

impl PoseAnimation {
    pub fn new() -> PoseAnimation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoseAnimation {
        static mut instance: ::protobuf::lazy::Lazy<PoseAnimation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoseAnimation,
        };
        unsafe {
            instance.get(|| {
                PoseAnimation {
                    model_name: ::protobuf::SingularField::none(),
                    model_id: ::std::option::Option::None,
                    pose: ::protobuf::RepeatedField::new(),
                    time: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string model_name = 1;

    pub fn clear_model_name(&mut self) {
        self.model_name.clear();
    }

    pub fn has_model_name(&self) -> bool {
        self.model_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_name(&mut self, v: ::std::string::String) {
        self.model_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_model_name(&mut self) -> &mut ::std::string::String {
        if self.model_name.is_none() {
            self.model_name.set_default();
        };
        self.model_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_model_name(&mut self) -> ::std::string::String {
        self.model_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_model_name(&self) -> &str {
        match self.model_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 model_id = 2;

    pub fn clear_model_id(&mut self) {
        self.model_id = ::std::option::Option::None;
    }

    pub fn has_model_id(&self) -> bool {
        self.model_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_id(&mut self, v: u32) {
        self.model_id = ::std::option::Option::Some(v);
    }

    pub fn get_model_id(&self) -> u32 {
        self.model_id.unwrap_or(0)
    }

    // repeated .gazebo.msgs.Pose pose = 3;

    pub fn clear_pose(&mut self) {
        self.pose.clear();
    }

    // Param is passed by value, moved
    pub fn set_pose(&mut self, v: ::protobuf::RepeatedField<super::pose::Pose>) {
        self.pose = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pose(&mut self) -> &mut ::protobuf::RepeatedField<super::pose::Pose> {
        &mut self.pose
    }

    // Take field
    pub fn take_pose(&mut self) -> ::protobuf::RepeatedField<super::pose::Pose> {
        ::std::mem::replace(&mut self.pose, ::protobuf::RepeatedField::new())
    }

    pub fn get_pose(&self) -> &[super::pose::Pose] {
        &self.pose
    }

    // repeated .gazebo.msgs.Time time = 4;

    pub fn clear_time(&mut self) {
        self.time.clear();
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: ::protobuf::RepeatedField<super::time::Time>) {
        self.time = v;
    }

    // Mutable pointer to the field.
    pub fn mut_time(&mut self) -> &mut ::protobuf::RepeatedField<super::time::Time> {
        &mut self.time
    }

    // Take field
    pub fn take_time(&mut self) -> ::protobuf::RepeatedField<super::time::Time> {
        ::std::mem::replace(&mut self.time, ::protobuf::RepeatedField::new())
    }

    pub fn get_time(&self) -> &[super::time::Time] {
        &self.time
    }
}

impl ::protobuf::Message for PoseAnimation {
    fn is_initialized(&self) -> bool {
        if self.model_name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.model_name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.model_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pose));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.time));
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
        for value in &self.model_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.model_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.model_name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.model_id {
            try!(os.write_uint32(2, v));
        };
        for v in &self.pose {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.time {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<PoseAnimation>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PoseAnimation {
    fn new() -> PoseAnimation {
        PoseAnimation::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoseAnimation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "model_name",
                    PoseAnimation::has_model_name,
                    PoseAnimation::get_model_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "model_id",
                    PoseAnimation::has_model_id,
                    PoseAnimation::get_model_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "pose",
                    PoseAnimation::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "time",
                    PoseAnimation::get_time,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoseAnimation>(
                    "PoseAnimation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoseAnimation {
    fn clear(&mut self) {
        self.clear_model_name();
        self.clear_model_id();
        self.clear_pose();
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PoseAnimation {
    fn eq(&self, other: &PoseAnimation) -> bool {
        self.model_name == other.model_name &&
        self.model_id == other.model_id &&
        self.pose == other.pose &&
        self.time == other.time &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PoseAnimation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x70, 0x6f, 0x73, 0x65, 0x5f, 0x61, 0x6e, 0x69, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x1a, 0x0a, 0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x77, 0x0a, 0x0d, 0x50,
    0x6f, 0x73, 0x65, 0x41, 0x6e, 0x69, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x0a,
    0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x10, 0x0a, 0x08, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x1f, 0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50,
    0x6f, 0x73, 0x65, 0x12, 0x1f, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x54, 0x69, 0x6d, 0x65, 0x4a, 0xd4, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x10, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x07, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x13, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0c, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0d, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x02, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x0e, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x0f, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0f, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x10, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0f, 0x24, 0x25,
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
