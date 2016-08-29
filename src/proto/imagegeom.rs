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
pub struct ImageGeom {
    // message fields
    uri: ::protobuf::SingularField<::std::string::String>,
    scale: ::std::option::Option<f64>,
    threshold: ::std::option::Option<i32>,
    height: ::std::option::Option<f64>,
    granularity: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImageGeom {}

impl ImageGeom {
    pub fn new() -> ImageGeom {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImageGeom {
        static mut instance: ::protobuf::lazy::Lazy<ImageGeom> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImageGeom,
        };
        unsafe {
            instance.get(|| {
                ImageGeom {
                    uri: ::protobuf::SingularField::none(),
                    scale: ::std::option::Option::None,
                    threshold: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    granularity: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string uri = 1;

    pub fn clear_uri(&mut self) {
        self.uri.clear();
    }

    pub fn has_uri(&self) -> bool {
        self.uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uri(&mut self) -> &mut ::std::string::String {
        if self.uri.is_none() {
            self.uri.set_default();
        };
        self.uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_uri(&mut self) -> ::std::string::String {
        self.uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_uri(&self) -> &str {
        match self.uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional double scale = 2;

    pub fn clear_scale(&mut self) {
        self.scale = ::std::option::Option::None;
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: f64) {
        self.scale = ::std::option::Option::Some(v);
    }

    pub fn get_scale(&self) -> f64 {
        self.scale.unwrap_or(0.)
    }

    // optional int32 threshold = 3;

    pub fn clear_threshold(&mut self) {
        self.threshold = ::std::option::Option::None;
    }

    pub fn has_threshold(&self) -> bool {
        self.threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_threshold(&mut self, v: i32) {
        self.threshold = ::std::option::Option::Some(v);
    }

    pub fn get_threshold(&self) -> i32 {
        self.threshold.unwrap_or(255i32)
    }

    // optional double height = 4;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: f64) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> f64 {
        self.height.unwrap_or(0.)
    }

    // optional int32 granularity = 5;

    pub fn clear_granularity(&mut self) {
        self.granularity = ::std::option::Option::None;
    }

    pub fn has_granularity(&self) -> bool {
        self.granularity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_granularity(&mut self, v: i32) {
        self.granularity = ::std::option::Option::Some(v);
    }

    pub fn get_granularity(&self) -> i32 {
        self.granularity.unwrap_or(0)
    }
}

impl ::protobuf::Message for ImageGeom {
    fn is_initialized(&self) -> bool {
        if self.uri.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uri));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.scale = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.threshold = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.height = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.granularity = ::std::option::Option::Some(tmp);
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
        for value in &self.uri {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.scale.is_some() {
            my_size += 9;
        };
        for value in &self.threshold {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.height.is_some() {
            my_size += 9;
        };
        for value in &self.granularity {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uri.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.scale {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.threshold {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.height {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.granularity {
            try!(os.write_int32(5, v));
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
        ::std::any::TypeId::of::<ImageGeom>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ImageGeom {
    fn new() -> ImageGeom {
        ImageGeom::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImageGeom>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "uri",
                    ImageGeom::has_uri,
                    ImageGeom::get_uri,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "scale",
                    ImageGeom::has_scale,
                    ImageGeom::get_scale,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "threshold",
                    ImageGeom::has_threshold,
                    ImageGeom::get_threshold,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "height",
                    ImageGeom::has_height,
                    ImageGeom::get_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "granularity",
                    ImageGeom::has_granularity,
                    ImageGeom::get_granularity,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImageGeom>(
                    "ImageGeom",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImageGeom {
    fn clear(&mut self) {
        self.clear_uri();
        self.clear_scale();
        self.clear_threshold();
        self.clear_height();
        self.clear_granularity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ImageGeom {
    fn eq(&self, other: &ImageGeom) -> bool {
        self.uri == other.uri &&
        self.scale == other.scale &&
        self.threshold == other.threshold &&
        self.height == other.height &&
        self.granularity == other.granularity &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ImageGeom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x67, 0x65, 0x6f, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0x64,
    0x0a, 0x09, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x47, 0x65, 0x6f, 0x6d, 0x12, 0x0b, 0x0a, 0x03, 0x75,
    0x72, 0x69, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x63, 0x61, 0x6c,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x12, 0x16, 0x0a, 0x09, 0x74, 0x68, 0x72, 0x65, 0x73,
    0x68, 0x6f, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x03, 0x32, 0x35, 0x35, 0x12,
    0x0e, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x12,
    0x13, 0x0a, 0x0b, 0x67, 0x72, 0x61, 0x6e, 0x75, 0x6c, 0x61, 0x72, 0x69, 0x74, 0x79, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x05, 0x4a, 0x9f, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x07, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08,
    0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x09, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x0a, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x0b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b,
    0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x0b, 0x22, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x07, 0x12, 0x03, 0x0b, 0x2d, 0x30, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x0c, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x20,
    0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x0d, 0x20, 0x21,
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
