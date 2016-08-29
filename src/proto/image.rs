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
pub struct Image {
    // message fields
    width: ::std::option::Option<u32>,
    height: ::std::option::Option<u32>,
    pixel_format: ::std::option::Option<u32>,
    step: ::std::option::Option<u32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Image {}

impl Image {
    pub fn new() -> Image {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Image {
        static mut instance: ::protobuf::lazy::Lazy<Image> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Image,
        };
        unsafe {
            instance.get(|| {
                Image {
                    width: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    pixel_format: ::std::option::Option::None,
                    step: ::std::option::Option::None,
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 width = 1;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: u32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> u32 {
        self.width.unwrap_or(0)
    }

    // required uint32 height = 2;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> u32 {
        self.height.unwrap_or(0)
    }

    // required uint32 pixel_format = 3;

    pub fn clear_pixel_format(&mut self) {
        self.pixel_format = ::std::option::Option::None;
    }

    pub fn has_pixel_format(&self) -> bool {
        self.pixel_format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pixel_format(&mut self, v: u32) {
        self.pixel_format = ::std::option::Option::Some(v);
    }

    pub fn get_pixel_format(&self) -> u32 {
        self.pixel_format.unwrap_or(0)
    }

    // required uint32 step = 4;

    pub fn clear_step(&mut self) {
        self.step = ::std::option::Option::None;
    }

    pub fn has_step(&self) -> bool {
        self.step.is_some()
    }

    // Param is passed by value, moved
    pub fn set_step(&mut self, v: u32) {
        self.step = ::std::option::Option::Some(v);
    }

    pub fn get_step(&self) -> u32 {
        self.step.unwrap_or(0)
    }

    // required bytes data = 5;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Image {
    fn is_initialized(&self) -> bool {
        if self.width.is_none() {
            return false;
        };
        if self.height.is_none() {
            return false;
        };
        if self.pixel_format.is_none() {
            return false;
        };
        if self.step.is_none() {
            return false;
        };
        if self.data.is_none() {
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
                    self.width = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.height = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pixel_format = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.step = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
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
        for value in &self.width {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.height {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pixel_format {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.step {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.data {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.width {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.height {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.pixel_format {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.step {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(5, &v));
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
        ::std::any::TypeId::of::<Image>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Image {
    fn new() -> Image {
        Image::new()
    }

    fn descriptor_static(_: ::std::option::Option<Image>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "width",
                    Image::has_width,
                    Image::get_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "height",
                    Image::has_height,
                    Image::get_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pixel_format",
                    Image::has_pixel_format,
                    Image::get_pixel_format,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "step",
                    Image::has_step,
                    Image::get_step,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    Image::has_data,
                    Image::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Image>(
                    "Image",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Image {
    fn clear(&mut self) {
        self.clear_width();
        self.clear_height();
        self.clear_pixel_format();
        self.clear_step();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Image {
    fn eq(&self, other: &Image) -> bool {
        self.width == other.width &&
        self.height == other.height &&
        self.pixel_format == other.pixel_format &&
        self.step == other.step &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Image {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0x58, 0x0a, 0x05, 0x49, 0x6d,
    0x61, 0x67, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x70, 0x69, 0x78, 0x65, 0x6c, 0x5f, 0x66, 0x6f, 0x72, 0x6d,
    0x61, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x73, 0x74, 0x65, 0x70,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x05,
    0x20, 0x02, 0x28, 0x0c, 0x4a, 0xff, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0f, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x07, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08,
    0x0d, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x24, 0x22, 0x21,
    0x20, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x20, 0x77, 0x69, 0x64, 0x74, 0x68, 0x20, 0x28, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x29,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x22, 0x23, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x0a, 0x02, 0x24, 0x22, 0x1f, 0x20, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x20, 0x68,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x20, 0x28, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66,
    0x20, 0x72, 0x6f, 0x77, 0x73, 0x29, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x12,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x22, 0x23, 0x0a,
    0x35, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x24, 0x22, 0x28, 0x20, 0x43,
    0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x49, 0x6d,
    0x61, 0x67, 0x65, 0x3a, 0x3a, 0x50, 0x69, 0x78, 0x65, 0x6c, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x20, 0x65, 0x6e, 0x75, 0x6d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x22, 0x23, 0x0a, 0x27,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x24, 0x22, 0x1a, 0x20, 0x46, 0x75,
    0x6c, 0x6c, 0x20, 0x72, 0x6f, 0x77, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20, 0x69, 0x6e,
    0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x22, 0x23, 0x0a,
    0x7d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x23, 0x1a, 0x4a, 0x20, 0x72,
    0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x20, 0x64,
    0x61, 0x74, 0x61, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x3d, 0x20, 0x35,
    0x3b, 0x20, 0x2f, 0x2f, 0x20, 0x41, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x64, 0x61, 0x74, 0x61,
    0x2c, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x69, 0x66, 0x20, 0x28, 0x73, 0x74, 0x65, 0x70, 0x20,
    0x2a, 0x20, 0x72, 0x6f, 0x77, 0x73, 0x29, 0x0a, 0x22, 0x24, 0x20, 0x41, 0x63, 0x74, 0x75, 0x61,
    0x6c, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2c, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x69, 0x66, 0x20,
    0x28, 0x73, 0x74, 0x65, 0x70, 0x20, 0x2a, 0x20, 0x72, 0x6f, 0x77, 0x73, 0x29, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x0e, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x0e, 0x21, 0x22,
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
