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
pub struct WorldModify {
    // message fields
    world_name: ::protobuf::SingularField<::std::string::String>,
    remove: ::std::option::Option<bool>,
    create: ::std::option::Option<bool>,
    cloned: ::std::option::Option<bool>,
    cloned_uri: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WorldModify {}

impl WorldModify {
    pub fn new() -> WorldModify {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WorldModify {
        static mut instance: ::protobuf::lazy::Lazy<WorldModify> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WorldModify,
        };
        unsafe {
            instance.get(|| {
                WorldModify {
                    world_name: ::protobuf::SingularField::none(),
                    remove: ::std::option::Option::None,
                    create: ::std::option::Option::None,
                    cloned: ::std::option::Option::None,
                    cloned_uri: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string world_name = 1;

    pub fn clear_world_name(&mut self) {
        self.world_name.clear();
    }

    pub fn has_world_name(&self) -> bool {
        self.world_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_name(&mut self, v: ::std::string::String) {
        self.world_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_name(&mut self) -> &mut ::std::string::String {
        if self.world_name.is_none() {
            self.world_name.set_default();
        };
        self.world_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_world_name(&mut self) -> ::std::string::String {
        self.world_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_world_name(&self) -> &str {
        match self.world_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool remove = 2;

    pub fn clear_remove(&mut self) {
        self.remove = ::std::option::Option::None;
    }

    pub fn has_remove(&self) -> bool {
        self.remove.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remove(&mut self, v: bool) {
        self.remove = ::std::option::Option::Some(v);
    }

    pub fn get_remove(&self) -> bool {
        self.remove.unwrap_or(false)
    }

    // optional bool create = 3;

    pub fn clear_create(&mut self) {
        self.create = ::std::option::Option::None;
    }

    pub fn has_create(&self) -> bool {
        self.create.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create(&mut self, v: bool) {
        self.create = ::std::option::Option::Some(v);
    }

    pub fn get_create(&self) -> bool {
        self.create.unwrap_or(false)
    }

    // optional bool cloned = 4;

    pub fn clear_cloned(&mut self) {
        self.cloned = ::std::option::Option::None;
    }

    pub fn has_cloned(&self) -> bool {
        self.cloned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cloned(&mut self, v: bool) {
        self.cloned = ::std::option::Option::Some(v);
    }

    pub fn get_cloned(&self) -> bool {
        self.cloned.unwrap_or(false)
    }

    // optional string cloned_uri = 5;

    pub fn clear_cloned_uri(&mut self) {
        self.cloned_uri.clear();
    }

    pub fn has_cloned_uri(&self) -> bool {
        self.cloned_uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cloned_uri(&mut self, v: ::std::string::String) {
        self.cloned_uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cloned_uri(&mut self) -> &mut ::std::string::String {
        if self.cloned_uri.is_none() {
            self.cloned_uri.set_default();
        };
        self.cloned_uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_cloned_uri(&mut self) -> ::std::string::String {
        self.cloned_uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cloned_uri(&self) -> &str {
        match self.cloned_uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for WorldModify {
    fn is_initialized(&self) -> bool {
        if self.world_name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.world_name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.remove = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.create = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.cloned = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cloned_uri));
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
        for value in &self.world_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.remove.is_some() {
            my_size += 2;
        };
        if self.create.is_some() {
            my_size += 2;
        };
        if self.cloned.is_some() {
            my_size += 2;
        };
        for value in &self.cloned_uri {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.world_name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.remove {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.create {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.cloned {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.cloned_uri.as_ref() {
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
        ::std::any::TypeId::of::<WorldModify>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WorldModify {
    fn new() -> WorldModify {
        WorldModify::new()
    }

    fn descriptor_static(_: ::std::option::Option<WorldModify>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "world_name",
                    WorldModify::has_world_name,
                    WorldModify::get_world_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "remove",
                    WorldModify::has_remove,
                    WorldModify::get_remove,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "create",
                    WorldModify::has_create,
                    WorldModify::get_create,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "cloned",
                    WorldModify::has_cloned,
                    WorldModify::get_cloned,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cloned_uri",
                    WorldModify::has_cloned_uri,
                    WorldModify::get_cloned_uri,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WorldModify>(
                    "WorldModify",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WorldModify {
    fn clear(&mut self) {
        self.clear_world_name();
        self.clear_remove();
        self.clear_create();
        self.clear_cloned();
        self.clear_cloned_uri();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WorldModify {
    fn eq(&self, other: &WorldModify) -> bool {
        self.world_name == other.world_name &&
        self.remove == other.remove &&
        self.create == other.create &&
        self.cloned == other.cloned &&
        self.cloned_uri == other.cloned_uri &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WorldModify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x79, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x22, 0x65, 0x0a, 0x0b, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x79,
    0x12, 0x12, 0x0a, 0x0a, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x6c, 0x6f, 0x6e, 0x65, 0x64, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6c, 0x6f, 0x6e, 0x65, 0x64, 0x5f, 0x75,
    0x72, 0x69, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x4a, 0x83, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x07, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09,
    0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x0a, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x10,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x19, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x0b, 0x10, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x0b, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x10, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x0d, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0d,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x1f, 0x20,
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
