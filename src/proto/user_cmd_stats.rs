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
pub struct UserCmdStats {
    // message fields
    undo_cmd_count: ::std::option::Option<u32>,
    redo_cmd_count: ::std::option::Option<u32>,
    undo_cmd: ::protobuf::RepeatedField<super::user_cmd::UserCmd>,
    redo_cmd: ::protobuf::RepeatedField<super::user_cmd::UserCmd>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserCmdStats {}

impl UserCmdStats {
    pub fn new() -> UserCmdStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserCmdStats {
        static mut instance: ::protobuf::lazy::Lazy<UserCmdStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserCmdStats,
        };
        unsafe {
            instance.get(|| {
                UserCmdStats {
                    undo_cmd_count: ::std::option::Option::None,
                    redo_cmd_count: ::std::option::Option::None,
                    undo_cmd: ::protobuf::RepeatedField::new(),
                    redo_cmd: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 undo_cmd_count = 1;

    pub fn clear_undo_cmd_count(&mut self) {
        self.undo_cmd_count = ::std::option::Option::None;
    }

    pub fn has_undo_cmd_count(&self) -> bool {
        self.undo_cmd_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_undo_cmd_count(&mut self, v: u32) {
        self.undo_cmd_count = ::std::option::Option::Some(v);
    }

    pub fn get_undo_cmd_count(&self) -> u32 {
        self.undo_cmd_count.unwrap_or(0)
    }

    // required uint32 redo_cmd_count = 2;

    pub fn clear_redo_cmd_count(&mut self) {
        self.redo_cmd_count = ::std::option::Option::None;
    }

    pub fn has_redo_cmd_count(&self) -> bool {
        self.redo_cmd_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_redo_cmd_count(&mut self, v: u32) {
        self.redo_cmd_count = ::std::option::Option::Some(v);
    }

    pub fn get_redo_cmd_count(&self) -> u32 {
        self.redo_cmd_count.unwrap_or(0)
    }

    // repeated .gazebo.msgs.UserCmd undo_cmd = 3;

    pub fn clear_undo_cmd(&mut self) {
        self.undo_cmd.clear();
    }

    // Param is passed by value, moved
    pub fn set_undo_cmd(&mut self, v: ::protobuf::RepeatedField<super::user_cmd::UserCmd>) {
        self.undo_cmd = v;
    }

    // Mutable pointer to the field.
    pub fn mut_undo_cmd(&mut self) -> &mut ::protobuf::RepeatedField<super::user_cmd::UserCmd> {
        &mut self.undo_cmd
    }

    // Take field
    pub fn take_undo_cmd(&mut self) -> ::protobuf::RepeatedField<super::user_cmd::UserCmd> {
        ::std::mem::replace(&mut self.undo_cmd, ::protobuf::RepeatedField::new())
    }

    pub fn get_undo_cmd(&self) -> &[super::user_cmd::UserCmd] {
        &self.undo_cmd
    }

    // repeated .gazebo.msgs.UserCmd redo_cmd = 4;

    pub fn clear_redo_cmd(&mut self) {
        self.redo_cmd.clear();
    }

    // Param is passed by value, moved
    pub fn set_redo_cmd(&mut self, v: ::protobuf::RepeatedField<super::user_cmd::UserCmd>) {
        self.redo_cmd = v;
    }

    // Mutable pointer to the field.
    pub fn mut_redo_cmd(&mut self) -> &mut ::protobuf::RepeatedField<super::user_cmd::UserCmd> {
        &mut self.redo_cmd
    }

    // Take field
    pub fn take_redo_cmd(&mut self) -> ::protobuf::RepeatedField<super::user_cmd::UserCmd> {
        ::std::mem::replace(&mut self.redo_cmd, ::protobuf::RepeatedField::new())
    }

    pub fn get_redo_cmd(&self) -> &[super::user_cmd::UserCmd] {
        &self.redo_cmd
    }
}

impl ::protobuf::Message for UserCmdStats {
    fn is_initialized(&self) -> bool {
        if self.undo_cmd_count.is_none() {
            return false;
        };
        if self.redo_cmd_count.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.undo_cmd_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.redo_cmd_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.undo_cmd));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.redo_cmd));
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
        for value in &self.undo_cmd_count {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.redo_cmd_count {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.undo_cmd {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.redo_cmd {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.undo_cmd_count {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.redo_cmd_count {
            try!(os.write_uint32(2, v));
        };
        for v in &self.undo_cmd {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.redo_cmd {
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
        ::std::any::TypeId::of::<UserCmdStats>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UserCmdStats {
    fn new() -> UserCmdStats {
        UserCmdStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserCmdStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "undo_cmd_count",
                    UserCmdStats::has_undo_cmd_count,
                    UserCmdStats::get_undo_cmd_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "redo_cmd_count",
                    UserCmdStats::has_redo_cmd_count,
                    UserCmdStats::get_redo_cmd_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "undo_cmd",
                    UserCmdStats::get_undo_cmd,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "redo_cmd",
                    UserCmdStats::get_redo_cmd,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserCmdStats>(
                    "UserCmdStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserCmdStats {
    fn clear(&mut self) {
        self.clear_undo_cmd_count();
        self.clear_redo_cmd_count();
        self.clear_undo_cmd();
        self.clear_redo_cmd();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UserCmdStats {
    fn eq(&self, other: &UserCmdStats) -> bool {
        self.undo_cmd_count == other.undo_cmd_count &&
        self.redo_cmd_count == other.redo_cmd_count &&
        self.undo_cmd == other.undo_cmd &&
        self.redo_cmd == other.redo_cmd &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UserCmdStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x63, 0x6d, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x1a, 0x0e, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x63, 0x6d, 0x64, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x8e, 0x01, 0x0a, 0x0c, 0x55, 0x73, 0x65, 0x72, 0x43, 0x6d, 0x64, 0x53,
    0x74, 0x61, 0x74, 0x73, 0x12, 0x16, 0x0a, 0x0e, 0x75, 0x6e, 0x64, 0x6f, 0x5f, 0x63, 0x6d, 0x64,
    0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x16, 0x0a, 0x0e,
    0x72, 0x65, 0x64, 0x6f, 0x5f, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0d, 0x12, 0x26, 0x0a, 0x08, 0x75, 0x6e, 0x64, 0x6f, 0x5f, 0x63, 0x6d, 0x64,
    0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e,
    0x6d, 0x73, 0x67, 0x73, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x43, 0x6d, 0x64, 0x12, 0x26, 0x0a, 0x08,
    0x72, 0x65, 0x64, 0x6f, 0x5f, 0x63, 0x6d, 0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14,
    0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x55, 0x73, 0x65,
    0x72, 0x43, 0x6d, 0x64, 0x4a, 0x89, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x15, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x06, 0x07, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x15, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x14, 0x0a, 0x40, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x25, 0x1a, 0x33, 0x2f, 0x20, 0x5c, 0x62, 0x72,
    0x69, 0x65, 0x66, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x75, 0x73,
    0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x75, 0x6e, 0x64, 0x6f, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x0e, 0x02, 0x25, 0x1a, 0x33, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x64,
    0x6f, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e,
    0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x23, 0x24,
    0x0a, 0x36, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x11, 0x02, 0x20, 0x1a, 0x29, 0x2f,
    0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x64,
    0x6f, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x11, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11,
    0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x11, 0x1e, 0x1f,
    0x0a, 0x36, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x14, 0x02, 0x20, 0x1a, 0x29, 0x2f,
    0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x64,
    0x6f, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x14, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x14,
    0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x14, 0x1e, 0x1f,
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
