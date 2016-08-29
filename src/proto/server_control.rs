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
pub struct ServerControl {
    // message fields
    save_world_name: ::protobuf::SingularField<::std::string::String>,
    save_filename: ::protobuf::SingularField<::std::string::String>,
    open_filename: ::protobuf::SingularField<::std::string::String>,
    new_world: ::std::option::Option<bool>,
    stop: ::std::option::Option<bool>,
    clone: ::std::option::Option<bool>,
    new_port: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerControl {}

impl ServerControl {
    pub fn new() -> ServerControl {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerControl {
        static mut instance: ::protobuf::lazy::Lazy<ServerControl> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerControl,
        };
        unsafe {
            instance.get(|| {
                ServerControl {
                    save_world_name: ::protobuf::SingularField::none(),
                    save_filename: ::protobuf::SingularField::none(),
                    open_filename: ::protobuf::SingularField::none(),
                    new_world: ::std::option::Option::None,
                    stop: ::std::option::Option::None,
                    clone: ::std::option::Option::None,
                    new_port: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string save_world_name = 1;

    pub fn clear_save_world_name(&mut self) {
        self.save_world_name.clear();
    }

    pub fn has_save_world_name(&self) -> bool {
        self.save_world_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_world_name(&mut self, v: ::std::string::String) {
        self.save_world_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_save_world_name(&mut self) -> &mut ::std::string::String {
        if self.save_world_name.is_none() {
            self.save_world_name.set_default();
        };
        self.save_world_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_save_world_name(&mut self) -> ::std::string::String {
        self.save_world_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_save_world_name(&self) -> &str {
        match self.save_world_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string save_filename = 2;

    pub fn clear_save_filename(&mut self) {
        self.save_filename.clear();
    }

    pub fn has_save_filename(&self) -> bool {
        self.save_filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_filename(&mut self, v: ::std::string::String) {
        self.save_filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_save_filename(&mut self) -> &mut ::std::string::String {
        if self.save_filename.is_none() {
            self.save_filename.set_default();
        };
        self.save_filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_save_filename(&mut self) -> ::std::string::String {
        self.save_filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_save_filename(&self) -> &str {
        match self.save_filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string open_filename = 3;

    pub fn clear_open_filename(&mut self) {
        self.open_filename.clear();
    }

    pub fn has_open_filename(&self) -> bool {
        self.open_filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_open_filename(&mut self, v: ::std::string::String) {
        self.open_filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_open_filename(&mut self) -> &mut ::std::string::String {
        if self.open_filename.is_none() {
            self.open_filename.set_default();
        };
        self.open_filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_open_filename(&mut self) -> ::std::string::String {
        self.open_filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_open_filename(&self) -> &str {
        match self.open_filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool new_world = 4;

    pub fn clear_new_world(&mut self) {
        self.new_world = ::std::option::Option::None;
    }

    pub fn has_new_world(&self) -> bool {
        self.new_world.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_world(&mut self, v: bool) {
        self.new_world = ::std::option::Option::Some(v);
    }

    pub fn get_new_world(&self) -> bool {
        self.new_world.unwrap_or(false)
    }

    // optional bool stop = 5;

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

    // optional bool clone = 6;

    pub fn clear_clone(&mut self) {
        self.clone = ::std::option::Option::None;
    }

    pub fn has_clone(&self) -> bool {
        self.clone.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clone(&mut self, v: bool) {
        self.clone = ::std::option::Option::Some(v);
    }

    pub fn get_clone(&self) -> bool {
        self.clone.unwrap_or(false)
    }

    // optional uint32 new_port = 7;

    pub fn clear_new_port(&mut self) {
        self.new_port = ::std::option::Option::None;
    }

    pub fn has_new_port(&self) -> bool {
        self.new_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_port(&mut self, v: u32) {
        self.new_port = ::std::option::Option::Some(v);
    }

    pub fn get_new_port(&self) -> u32 {
        self.new_port.unwrap_or(0)
    }
}

impl ::protobuf::Message for ServerControl {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.save_world_name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.save_filename));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.open_filename));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.new_world = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stop = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.clone = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.new_port = ::std::option::Option::Some(tmp);
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
        for value in &self.save_world_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.save_filename {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.open_filename {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if self.new_world.is_some() {
            my_size += 2;
        };
        if self.stop.is_some() {
            my_size += 2;
        };
        if self.clone.is_some() {
            my_size += 2;
        };
        for value in &self.new_port {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.save_world_name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.save_filename.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.open_filename.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.new_world {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.stop {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.clone {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.new_port {
            try!(os.write_uint32(7, v));
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
        ::std::any::TypeId::of::<ServerControl>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerControl {
    fn new() -> ServerControl {
        ServerControl::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerControl>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "save_world_name",
                    ServerControl::has_save_world_name,
                    ServerControl::get_save_world_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "save_filename",
                    ServerControl::has_save_filename,
                    ServerControl::get_save_filename,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "open_filename",
                    ServerControl::has_open_filename,
                    ServerControl::get_open_filename,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "new_world",
                    ServerControl::has_new_world,
                    ServerControl::get_new_world,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stop",
                    ServerControl::has_stop,
                    ServerControl::get_stop,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "clone",
                    ServerControl::has_clone,
                    ServerControl::get_clone,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "new_port",
                    ServerControl::has_new_port,
                    ServerControl::get_new_port,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerControl>(
                    "ServerControl",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerControl {
    fn clear(&mut self) {
        self.clear_save_world_name();
        self.clear_save_filename();
        self.clear_open_filename();
        self.clear_new_world();
        self.clear_stop();
        self.clear_clone();
        self.clear_new_port();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ServerControl {
    fn eq(&self, other: &ServerControl) -> bool {
        self.save_world_name == other.save_world_name &&
        self.save_filename == other.save_filename &&
        self.open_filename == other.open_filename &&
        self.new_world == other.new_world &&
        self.stop == other.stop &&
        self.clone == other.clone &&
        self.new_port == other.new_port &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ServerControl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x22, 0x98, 0x01, 0x0a, 0x0d, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x12, 0x17, 0x0a, 0x0f, 0x73, 0x61, 0x76, 0x65, 0x5f, 0x77, 0x6f,
    0x72, 0x6c, 0x64, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x15,
    0x0a, 0x0d, 0x73, 0x61, 0x76, 0x65, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x15, 0x0a, 0x0d, 0x6f, 0x70, 0x65, 0x6e, 0x5f, 0x66, 0x69,
    0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09,
    0x6e, 0x65, 0x77, 0x5f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x12,
    0x0c, 0x0a, 0x04, 0x73, 0x74, 0x6f, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0d, 0x0a,
    0x05, 0x63, 0x6c, 0x6f, 0x6e, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x10, 0x0a, 0x08,
    0x6e, 0x65, 0x77, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x4a, 0x8d,
    0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x10, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x10, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09,
    0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x24, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0a, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b,
    0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x10,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x24, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0d, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x0d, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x02,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0e, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0e, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x06, 0x12, 0x03, 0x0f, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x0f,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0f, 0x24, 0x25,
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
