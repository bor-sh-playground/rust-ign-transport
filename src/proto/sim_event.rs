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
pub struct SimEvent {
    // message fields
    id: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    world_statistics: ::protobuf::SingularPtrField<super::world_stats::WorldStatistics>,
    data: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SimEvent {}

impl SimEvent {
    pub fn new() -> SimEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SimEvent {
        static mut instance: ::protobuf::lazy::Lazy<SimEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SimEvent,
        };
        unsafe {
            instance.get(|| {
                SimEvent {
                    id: ::std::option::Option::None,
                    field_type: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    world_statistics: ::protobuf::SingularPtrField::none(),
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // required string type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .gazebo.msgs.WorldStatistics world_statistics = 4;

    pub fn clear_world_statistics(&mut self) {
        self.world_statistics.clear();
    }

    pub fn has_world_statistics(&self) -> bool {
        self.world_statistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_statistics(&mut self, v: super::world_stats::WorldStatistics) {
        self.world_statistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_statistics(&mut self) -> &mut super::world_stats::WorldStatistics {
        if self.world_statistics.is_none() {
            self.world_statistics.set_default();
        };
        self.world_statistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_world_statistics(&mut self) -> super::world_stats::WorldStatistics {
        self.world_statistics.take().unwrap_or_else(|| super::world_stats::WorldStatistics::new())
    }

    pub fn get_world_statistics(&self) -> &super::world_stats::WorldStatistics {
        self.world_statistics.as_ref().unwrap_or_else(|| super::world_stats::WorldStatistics::default_instance())
    }

    // optional string data = 5;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::string::String) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::string::String {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::string::String {
        self.data.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data(&self) -> &str {
        match self.data.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SimEvent {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        if self.world_statistics.is_none() {
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
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.world_statistics));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data));
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
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.world_statistics {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.data {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.world_statistics.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.data.as_ref() {
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
        ::std::any::TypeId::of::<SimEvent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SimEvent {
    fn new() -> SimEvent {
        SimEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<SimEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    SimEvent::has_id,
                    SimEvent::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "type",
                    SimEvent::has_field_type,
                    SimEvent::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    SimEvent::has_name,
                    SimEvent::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "world_statistics",
                    SimEvent::has_world_statistics,
                    SimEvent::get_world_statistics,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "data",
                    SimEvent::has_data,
                    SimEvent::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SimEvent>(
                    "SimEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SimEvent {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_field_type();
        self.clear_name();
        self.clear_world_statistics();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SimEvent {
    fn eq(&self, other: &SimEvent) -> bool {
        self.id == other.id &&
        self.field_type == other.field_type &&
        self.name == other.name &&
        self.world_statistics == other.world_statistics &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SimEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x73, 0x69, 0x6d, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x11,
    0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x78, 0x0a, 0x08, 0x53, 0x69, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x0a, 0x0a,
    0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x36, 0x0a, 0x10, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x73,
    0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x57, 0x6f,
    0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x12, 0x0c, 0x0a,
    0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x4a, 0xb8, 0x04, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x17, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x07, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x07, 0x08, 0x10, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x30,
    0x1a, 0x22, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x49, 0x44, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x2e, 0x2f, 0x0a, 0x28, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x30, 0x1a, 0x1b, 0x2f, 0x20, 0x5c, 0x62, 0x72,
    0x69, 0x65, 0x66, 0x20, 0x54, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x69, 0x6d, 0x20,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x2e, 0x2f, 0x0a, 0x28,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x02, 0x30, 0x1a, 0x1b, 0x2f, 0x20, 0x5c,
    0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x69,
    0x6d, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x2e, 0x2f,
    0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x13, 0x02, 0x30, 0x1a, 0x21, 0x2f,
    0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x53, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69,
    0x63, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x13, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x13, 0x1b, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x13, 0x2e, 0x2f, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x16, 0x02, 0x30, 0x1a, 0x27, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20,
    0x44, 0x61, 0x74, 0x61, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x69, 0x6e, 0x67, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x69, 0x6d, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x16, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x16, 0x2e, 0x2f,
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
