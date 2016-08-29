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
pub struct LogicalCameraSensor {
    // message fields
    near_clip: ::std::option::Option<f64>,
    far_clip: ::std::option::Option<f64>,
    horizontal_fov: ::std::option::Option<f64>,
    aspect_ratio: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogicalCameraSensor {}

impl LogicalCameraSensor {
    pub fn new() -> LogicalCameraSensor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogicalCameraSensor {
        static mut instance: ::protobuf::lazy::Lazy<LogicalCameraSensor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogicalCameraSensor,
        };
        unsafe {
            instance.get(|| {
                LogicalCameraSensor {
                    near_clip: ::std::option::Option::None,
                    far_clip: ::std::option::Option::None,
                    horizontal_fov: ::std::option::Option::None,
                    aspect_ratio: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double near_clip = 1;

    pub fn clear_near_clip(&mut self) {
        self.near_clip = ::std::option::Option::None;
    }

    pub fn has_near_clip(&self) -> bool {
        self.near_clip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_near_clip(&mut self, v: f64) {
        self.near_clip = ::std::option::Option::Some(v);
    }

    pub fn get_near_clip(&self) -> f64 {
        self.near_clip.unwrap_or(0.)
    }

    // required double far_clip = 2;

    pub fn clear_far_clip(&mut self) {
        self.far_clip = ::std::option::Option::None;
    }

    pub fn has_far_clip(&self) -> bool {
        self.far_clip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_far_clip(&mut self, v: f64) {
        self.far_clip = ::std::option::Option::Some(v);
    }

    pub fn get_far_clip(&self) -> f64 {
        self.far_clip.unwrap_or(0.)
    }

    // required double horizontal_fov = 3;

    pub fn clear_horizontal_fov(&mut self) {
        self.horizontal_fov = ::std::option::Option::None;
    }

    pub fn has_horizontal_fov(&self) -> bool {
        self.horizontal_fov.is_some()
    }

    // Param is passed by value, moved
    pub fn set_horizontal_fov(&mut self, v: f64) {
        self.horizontal_fov = ::std::option::Option::Some(v);
    }

    pub fn get_horizontal_fov(&self) -> f64 {
        self.horizontal_fov.unwrap_or(0.)
    }

    // required double aspect_ratio = 4;

    pub fn clear_aspect_ratio(&mut self) {
        self.aspect_ratio = ::std::option::Option::None;
    }

    pub fn has_aspect_ratio(&self) -> bool {
        self.aspect_ratio.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aspect_ratio(&mut self, v: f64) {
        self.aspect_ratio = ::std::option::Option::Some(v);
    }

    pub fn get_aspect_ratio(&self) -> f64 {
        self.aspect_ratio.unwrap_or(0.)
    }
}

impl ::protobuf::Message for LogicalCameraSensor {
    fn is_initialized(&self) -> bool {
        if self.near_clip.is_none() {
            return false;
        };
        if self.far_clip.is_none() {
            return false;
        };
        if self.horizontal_fov.is_none() {
            return false;
        };
        if self.aspect_ratio.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.near_clip = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.far_clip = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.horizontal_fov = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.aspect_ratio = ::std::option::Option::Some(tmp);
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
        if self.near_clip.is_some() {
            my_size += 9;
        };
        if self.far_clip.is_some() {
            my_size += 9;
        };
        if self.horizontal_fov.is_some() {
            my_size += 9;
        };
        if self.aspect_ratio.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.near_clip {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.far_clip {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.horizontal_fov {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.aspect_ratio {
            try!(os.write_double(4, v));
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
        ::std::any::TypeId::of::<LogicalCameraSensor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogicalCameraSensor {
    fn new() -> LogicalCameraSensor {
        LogicalCameraSensor::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogicalCameraSensor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "near_clip",
                    LogicalCameraSensor::has_near_clip,
                    LogicalCameraSensor::get_near_clip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "far_clip",
                    LogicalCameraSensor::has_far_clip,
                    LogicalCameraSensor::get_far_clip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "horizontal_fov",
                    LogicalCameraSensor::has_horizontal_fov,
                    LogicalCameraSensor::get_horizontal_fov,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "aspect_ratio",
                    LogicalCameraSensor::has_aspect_ratio,
                    LogicalCameraSensor::get_aspect_ratio,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogicalCameraSensor>(
                    "LogicalCameraSensor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogicalCameraSensor {
    fn clear(&mut self) {
        self.clear_near_clip();
        self.clear_far_clip();
        self.clear_horizontal_fov();
        self.clear_aspect_ratio();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LogicalCameraSensor {
    fn eq(&self, other: &LogicalCameraSensor) -> bool {
        self.near_clip == other.near_clip &&
        self.far_clip == other.far_clip &&
        self.horizontal_fov == other.horizontal_fov &&
        self.aspect_ratio == other.aspect_ratio &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LogicalCameraSensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1b, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x5f, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x22, 0x68, 0x0a, 0x13, 0x4c, 0x6f,
    0x67, 0x69, 0x63, 0x61, 0x6c, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x53, 0x65, 0x6e, 0x73, 0x6f,
    0x72, 0x12, 0x11, 0x0a, 0x09, 0x6e, 0x65, 0x61, 0x72, 0x5f, 0x63, 0x6c, 0x69, 0x70, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x01, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x61, 0x72, 0x5f, 0x63, 0x6c, 0x69, 0x70,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x12, 0x16, 0x0a, 0x0e, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x6f,
    0x6e, 0x74, 0x61, 0x6c, 0x5f, 0x66, 0x6f, 0x76, 0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x12, 0x14,
    0x0a, 0x0c, 0x61, 0x73, 0x70, 0x65, 0x63, 0x74, 0x5f, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x18, 0x04,
    0x20, 0x02, 0x28, 0x01, 0x4a, 0xad, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x13, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x06, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08,
    0x1b, 0x0a, 0x49, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x26, 0x1a, 0x3c,
    0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x4e, 0x65, 0x61, 0x72, 0x20, 0x63, 0x6c,
    0x69, 0x70, 0x70, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x76, 0x69, 0x65, 0x77, 0x20, 0x66, 0x72, 0x75, 0x73, 0x74, 0x75, 0x6d,
    0x20, 0x69, 0x6e, 0x20, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x09, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x09, 0x24, 0x25, 0x0a, 0x48, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02,
    0x26, 0x1a, 0x3b, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x46, 0x61, 0x72, 0x20,
    0x63, 0x6c, 0x69, 0x70, 0x70, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x69, 0x65, 0x77, 0x20, 0x66, 0x72, 0x75, 0x73, 0x74,
    0x75, 0x6d, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0c, 0x24, 0x25, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x0f, 0x02, 0x26, 0x1a, 0x2e, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x48, 0x6f,
    0x72, 0x69, 0x7a, 0x6f, 0x6e, 0x74, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6f,
    0x66, 0x20, 0x76, 0x69, 0x65, 0x77, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e,
    0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x24, 0x25, 0x0a, 0x4f, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x12, 0x02, 0x26, 0x1a, 0x42, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69,
    0x65, 0x66, 0x20, 0x4e, 0x65, 0x61, 0x72, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x66, 0x61, 0x72, 0x20,
    0x63, 0x6c, 0x69, 0x70, 0x70, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x20, 0x61,
    0x73, 0x70, 0x65, 0x63, 0x74, 0x20, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x20, 0x28, 0x77, 0x69, 0x64,
    0x74, 0x68, 0x2f, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x29, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x12, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x12, 0x24, 0x25,
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
