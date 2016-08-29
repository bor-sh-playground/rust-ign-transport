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
pub struct PosesStamped {
    // message fields
    time: ::protobuf::SingularPtrField<super::time::Time>,
    pose: ::protobuf::RepeatedField<super::pose::Pose>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PosesStamped {}

impl PosesStamped {
    pub fn new() -> PosesStamped {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PosesStamped {
        static mut instance: ::protobuf::lazy::Lazy<PosesStamped> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PosesStamped,
        };
        unsafe {
            instance.get(|| {
                PosesStamped {
                    time: ::protobuf::SingularPtrField::none(),
                    pose: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Time time = 1;

    pub fn clear_time(&mut self) {
        self.time.clear();
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: super::time::Time) {
        self.time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_time(&mut self) -> &mut super::time::Time {
        if self.time.is_none() {
            self.time.set_default();
        };
        self.time.as_mut().unwrap()
    }

    // Take field
    pub fn take_time(&mut self) -> super::time::Time {
        self.time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_time(&self) -> &super::time::Time {
        self.time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // repeated .gazebo.msgs.Pose pose = 2;

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
}

impl ::protobuf::Message for PosesStamped {
    fn is_initialized(&self) -> bool {
        if self.time.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.time));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pose));
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
        for value in &self.time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.pose {
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
        ::std::any::TypeId::of::<PosesStamped>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PosesStamped {
    fn new() -> PosesStamped {
        PosesStamped::new()
    }

    fn descriptor_static(_: ::std::option::Option<PosesStamped>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "time",
                    PosesStamped::has_time,
                    PosesStamped::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "pose",
                    PosesStamped::get_pose,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PosesStamped>(
                    "PosesStamped",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PosesStamped {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_pose();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PosesStamped {
    fn eq(&self, other: &PosesStamped) -> bool {
        self.time == other.time &&
        self.pose == other.pose &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PosesStamped {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x70, 0x6f, 0x73, 0x65, 0x73, 0x5f, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x65, 0x64, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x1a, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a,
    0x70, 0x6f, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x50, 0x0a, 0x0c, 0x50, 0x6f,
    0x73, 0x65, 0x73, 0x53, 0x74, 0x61, 0x6d, 0x70, 0x65, 0x64, 0x12, 0x1f, 0x0a, 0x04, 0x74, 0x69,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62,
    0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x04, 0x70,
    0x6f, 0x73, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65,
    0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x4a, 0xca, 0x01, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x0d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08,
    0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09,
    0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0b, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0b, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x17, 0x18,
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
