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
pub struct WorldControl {
    // message fields
    pause: ::std::option::Option<bool>,
    step: ::std::option::Option<bool>,
    multi_step: ::std::option::Option<u32>,
    reset: ::protobuf::SingularPtrField<super::world_reset::WorldReset>,
    seed: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WorldControl {}

impl WorldControl {
    pub fn new() -> WorldControl {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WorldControl {
        static mut instance: ::protobuf::lazy::Lazy<WorldControl> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WorldControl,
        };
        unsafe {
            instance.get(|| {
                WorldControl {
                    pause: ::std::option::Option::None,
                    step: ::std::option::Option::None,
                    multi_step: ::std::option::Option::None,
                    reset: ::protobuf::SingularPtrField::none(),
                    seed: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool pause = 1;

    pub fn clear_pause(&mut self) {
        self.pause = ::std::option::Option::None;
    }

    pub fn has_pause(&self) -> bool {
        self.pause.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pause(&mut self, v: bool) {
        self.pause = ::std::option::Option::Some(v);
    }

    pub fn get_pause(&self) -> bool {
        self.pause.unwrap_or(false)
    }

    // optional bool step = 2;

    pub fn clear_step(&mut self) {
        self.step = ::std::option::Option::None;
    }

    pub fn has_step(&self) -> bool {
        self.step.is_some()
    }

    // Param is passed by value, moved
    pub fn set_step(&mut self, v: bool) {
        self.step = ::std::option::Option::Some(v);
    }

    pub fn get_step(&self) -> bool {
        self.step.unwrap_or(false)
    }

    // optional uint32 multi_step = 3;

    pub fn clear_multi_step(&mut self) {
        self.multi_step = ::std::option::Option::None;
    }

    pub fn has_multi_step(&self) -> bool {
        self.multi_step.is_some()
    }

    // Param is passed by value, moved
    pub fn set_multi_step(&mut self, v: u32) {
        self.multi_step = ::std::option::Option::Some(v);
    }

    pub fn get_multi_step(&self) -> u32 {
        self.multi_step.unwrap_or(0)
    }

    // optional .gazebo.msgs.WorldReset reset = 4;

    pub fn clear_reset(&mut self) {
        self.reset.clear();
    }

    pub fn has_reset(&self) -> bool {
        self.reset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reset(&mut self, v: super::world_reset::WorldReset) {
        self.reset = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reset(&mut self) -> &mut super::world_reset::WorldReset {
        if self.reset.is_none() {
            self.reset.set_default();
        };
        self.reset.as_mut().unwrap()
    }

    // Take field
    pub fn take_reset(&mut self) -> super::world_reset::WorldReset {
        self.reset.take().unwrap_or_else(|| super::world_reset::WorldReset::new())
    }

    pub fn get_reset(&self) -> &super::world_reset::WorldReset {
        self.reset.as_ref().unwrap_or_else(|| super::world_reset::WorldReset::default_instance())
    }

    // optional uint32 seed = 5;

    pub fn clear_seed(&mut self) {
        self.seed = ::std::option::Option::None;
    }

    pub fn has_seed(&self) -> bool {
        self.seed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seed(&mut self, v: u32) {
        self.seed = ::std::option::Option::Some(v);
    }

    pub fn get_seed(&self) -> u32 {
        self.seed.unwrap_or(0)
    }
}

impl ::protobuf::Message for WorldControl {
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
                    self.pause = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.step = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.multi_step = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reset));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.seed = ::std::option::Option::Some(tmp);
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
        if self.pause.is_some() {
            my_size += 2;
        };
        if self.step.is_some() {
            my_size += 2;
        };
        for value in &self.multi_step {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.reset {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.seed {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pause {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.step {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.multi_step {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.reset.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.seed {
            try!(os.write_uint32(5, v));
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
        ::std::any::TypeId::of::<WorldControl>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WorldControl {
    fn new() -> WorldControl {
        WorldControl::new()
    }

    fn descriptor_static(_: ::std::option::Option<WorldControl>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "pause",
                    WorldControl::has_pause,
                    WorldControl::get_pause,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "step",
                    WorldControl::has_step,
                    WorldControl::get_step,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "multi_step",
                    WorldControl::has_multi_step,
                    WorldControl::get_multi_step,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "reset",
                    WorldControl::has_reset,
                    WorldControl::get_reset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "seed",
                    WorldControl::has_seed,
                    WorldControl::get_seed,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WorldControl>(
                    "WorldControl",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WorldControl {
    fn clear(&mut self) {
        self.clear_pause();
        self.clear_step();
        self.clear_multi_step();
        self.clear_reset();
        self.clear_seed();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WorldControl {
    fn eq(&self, other: &WorldControl) -> bool {
        self.pause == other.pause &&
        self.step == other.step &&
        self.multi_step == other.multi_step &&
        self.reset == other.reset &&
        self.seed == other.seed &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WorldControl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73,
    0x67, 0x73, 0x1a, 0x11, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x65, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x75, 0x0a, 0x0c, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x12, 0x0d, 0x0a, 0x05, 0x70, 0x61, 0x75, 0x73, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x73, 0x74, 0x65, 0x70, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x12, 0x0a, 0x0a, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x5f, 0x73, 0x74, 0x65, 0x70,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x26, 0x0a, 0x05, 0x72, 0x65, 0x73, 0x65, 0x74, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x52, 0x65, 0x73, 0x65, 0x74, 0x12, 0x0c,
    0x0a, 0x04, 0x73, 0x65, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x4a, 0x8e, 0x03, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x0f, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08,
    0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x1a, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x08, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x0b, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x10, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0c, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x0c, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x02, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x0e, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0e, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0e, 0x22, 0x23,
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
