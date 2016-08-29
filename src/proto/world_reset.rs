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
pub struct WorldReset {
    // message fields
    all: ::std::option::Option<bool>,
    time_only: ::std::option::Option<bool>,
    model_only: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WorldReset {}

impl WorldReset {
    pub fn new() -> WorldReset {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WorldReset {
        static mut instance: ::protobuf::lazy::Lazy<WorldReset> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WorldReset,
        };
        unsafe {
            instance.get(|| {
                WorldReset {
                    all: ::std::option::Option::None,
                    time_only: ::std::option::Option::None,
                    model_only: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool all = 1;

    pub fn clear_all(&mut self) {
        self.all = ::std::option::Option::None;
    }

    pub fn has_all(&self) -> bool {
        self.all.is_some()
    }

    // Param is passed by value, moved
    pub fn set_all(&mut self, v: bool) {
        self.all = ::std::option::Option::Some(v);
    }

    pub fn get_all(&self) -> bool {
        self.all.unwrap_or(true)
    }

    // optional bool time_only = 2;

    pub fn clear_time_only(&mut self) {
        self.time_only = ::std::option::Option::None;
    }

    pub fn has_time_only(&self) -> bool {
        self.time_only.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_only(&mut self, v: bool) {
        self.time_only = ::std::option::Option::Some(v);
    }

    pub fn get_time_only(&self) -> bool {
        self.time_only.unwrap_or(false)
    }

    // optional bool model_only = 3;

    pub fn clear_model_only(&mut self) {
        self.model_only = ::std::option::Option::None;
    }

    pub fn has_model_only(&self) -> bool {
        self.model_only.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_only(&mut self, v: bool) {
        self.model_only = ::std::option::Option::Some(v);
    }

    pub fn get_model_only(&self) -> bool {
        self.model_only.unwrap_or(false)
    }
}

impl ::protobuf::Message for WorldReset {
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
                    self.all = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.time_only = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.model_only = ::std::option::Option::Some(tmp);
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
        if self.all.is_some() {
            my_size += 2;
        };
        if self.time_only.is_some() {
            my_size += 2;
        };
        if self.model_only.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.all {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.time_only {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.model_only {
            try!(os.write_bool(3, v));
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
        ::std::any::TypeId::of::<WorldReset>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WorldReset {
    fn new() -> WorldReset {
        WorldReset::new()
    }

    fn descriptor_static(_: ::std::option::Option<WorldReset>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "all",
                    WorldReset::has_all,
                    WorldReset::get_all,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "time_only",
                    WorldReset::has_time_only,
                    WorldReset::get_time_only,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "model_only",
                    WorldReset::has_model_only,
                    WorldReset::get_model_only,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WorldReset>(
                    "WorldReset",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WorldReset {
    fn clear(&mut self) {
        self.clear_all();
        self.clear_time_only();
        self.clear_model_only();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WorldReset {
    fn eq(&self, other: &WorldReset) -> bool {
        self.all == other.all &&
        self.time_only == other.time_only &&
        self.model_only == other.model_only &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WorldReset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x65, 0x74, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x22, 0x54, 0x0a, 0x0a, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x52, 0x65, 0x73, 0x65, 0x74, 0x12, 0x11,
    0x0a, 0x03, 0x61, 0x6c, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75,
    0x65, 0x12, 0x18, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x6f, 0x6e, 0x6c, 0x79, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x19, 0x0a, 0x0a, 0x6d,
    0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x6f, 0x6e, 0x6c, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x3a,
    0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x4a, 0xcd, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0c,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x07, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x07, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x10, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x08, 0x12, 0x03, 0x09, 0x20, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x07, 0x12,
    0x03, 0x09, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02,
    0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x08, 0x12, 0x03, 0x0a, 0x20, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x07,
    0x12, 0x03, 0x0a, 0x2b, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b,
    0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x08, 0x12, 0x03, 0x0b, 0x20, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x07, 0x12, 0x03, 0x0b, 0x2b, 0x30,
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
