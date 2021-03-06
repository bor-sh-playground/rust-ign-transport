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
pub struct TrackVisual {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    id: ::std::option::Option<u32>,
    inherit_orientation: ::std::option::Option<bool>,
    min_dist: ::std::option::Option<f64>,
    max_dist: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TrackVisual {}

impl TrackVisual {
    pub fn new() -> TrackVisual {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TrackVisual {
        static mut instance: ::protobuf::lazy::Lazy<TrackVisual> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TrackVisual,
        };
        unsafe {
            instance.get(|| {
                TrackVisual {
                    name: ::protobuf::SingularField::none(),
                    id: ::std::option::Option::None,
                    inherit_orientation: ::std::option::Option::None,
                    min_dist: ::std::option::Option::None,
                    max_dist: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

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

    // optional uint32 id = 2;

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

    // optional bool inherit_orientation = 3;

    pub fn clear_inherit_orientation(&mut self) {
        self.inherit_orientation = ::std::option::Option::None;
    }

    pub fn has_inherit_orientation(&self) -> bool {
        self.inherit_orientation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inherit_orientation(&mut self, v: bool) {
        self.inherit_orientation = ::std::option::Option::Some(v);
    }

    pub fn get_inherit_orientation(&self) -> bool {
        self.inherit_orientation.unwrap_or(false)
    }

    // optional double min_dist = 4;

    pub fn clear_min_dist(&mut self) {
        self.min_dist = ::std::option::Option::None;
    }

    pub fn has_min_dist(&self) -> bool {
        self.min_dist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_dist(&mut self, v: f64) {
        self.min_dist = ::std::option::Option::Some(v);
    }

    pub fn get_min_dist(&self) -> f64 {
        self.min_dist.unwrap_or(0.)
    }

    // optional double max_dist = 5;

    pub fn clear_max_dist(&mut self) {
        self.max_dist = ::std::option::Option::None;
    }

    pub fn has_max_dist(&self) -> bool {
        self.max_dist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_dist(&mut self, v: f64) {
        self.max_dist = ::std::option::Option::Some(v);
    }

    pub fn get_max_dist(&self) -> f64 {
        self.max_dist.unwrap_or(0.)
    }
}

impl ::protobuf::Message for TrackVisual {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.inherit_orientation = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.min_dist = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.max_dist = ::std::option::Option::Some(tmp);
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.inherit_orientation.is_some() {
            my_size += 2;
        };
        if self.min_dist.is_some() {
            my_size += 9;
        };
        if self.max_dist.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.id {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.inherit_orientation {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.min_dist {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.max_dist {
            try!(os.write_double(5, v));
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
        ::std::any::TypeId::of::<TrackVisual>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TrackVisual {
    fn new() -> TrackVisual {
        TrackVisual::new()
    }

    fn descriptor_static(_: ::std::option::Option<TrackVisual>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    TrackVisual::has_name,
                    TrackVisual::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    TrackVisual::has_id,
                    TrackVisual::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "inherit_orientation",
                    TrackVisual::has_inherit_orientation,
                    TrackVisual::get_inherit_orientation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "min_dist",
                    TrackVisual::has_min_dist,
                    TrackVisual::get_min_dist,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "max_dist",
                    TrackVisual::has_max_dist,
                    TrackVisual::get_max_dist,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TrackVisual>(
                    "TrackVisual",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TrackVisual {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_inherit_orientation();
        self.clear_min_dist();
        self.clear_max_dist();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TrackVisual {
    fn eq(&self, other: &TrackVisual) -> bool {
        self.name == other.name &&
        self.id == other.id &&
        self.inherit_orientation == other.inherit_orientation &&
        self.min_dist == other.min_dist &&
        self.max_dist == other.max_dist &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TrackVisual {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x5f, 0x76, 0x69, 0x73, 0x75, 0x61, 0x6c, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x22, 0x68, 0x0a, 0x0b, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x56, 0x69, 0x73, 0x75, 0x61, 0x6c,
    0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0a,
    0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1b, 0x0a, 0x13, 0x69, 0x6e,
    0x68, 0x65, 0x72, 0x69, 0x74, 0x5f, 0x6f, 0x72, 0x69, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x69, 0x6e, 0x5f, 0x64,
    0x69, 0x73, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x61, 0x78,
    0x5f, 0x64, 0x69, 0x73, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x4a, 0x83, 0x03, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x09, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x26, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0a, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a,
    0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x10, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0b, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x0c, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x0d, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x26,
    0x27,
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
