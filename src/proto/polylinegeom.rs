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
pub struct Polyline {
    // message fields
    height: ::std::option::Option<f64>,
    point: ::protobuf::RepeatedField<super::vector2d::Vector2d>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Polyline {}

impl Polyline {
    pub fn new() -> Polyline {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Polyline {
        static mut instance: ::protobuf::lazy::Lazy<Polyline> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Polyline,
        };
        unsafe {
            instance.get(|| {
                Polyline {
                    height: ::std::option::Option::None,
                    point: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double height = 1;

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

    // repeated .gazebo.msgs.Vector2d point = 2;

    pub fn clear_point(&mut self) {
        self.point.clear();
    }

    // Param is passed by value, moved
    pub fn set_point(&mut self, v: ::protobuf::RepeatedField<super::vector2d::Vector2d>) {
        self.point = v;
    }

    // Mutable pointer to the field.
    pub fn mut_point(&mut self) -> &mut ::protobuf::RepeatedField<super::vector2d::Vector2d> {
        &mut self.point
    }

    // Take field
    pub fn take_point(&mut self) -> ::protobuf::RepeatedField<super::vector2d::Vector2d> {
        ::std::mem::replace(&mut self.point, ::protobuf::RepeatedField::new())
    }

    pub fn get_point(&self) -> &[super::vector2d::Vector2d] {
        &self.point
    }
}

impl ::protobuf::Message for Polyline {
    fn is_initialized(&self) -> bool {
        if self.height.is_none() {
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
                    self.height = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.point));
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
        if self.height.is_some() {
            my_size += 9;
        };
        for value in &self.point {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.height {
            try!(os.write_double(1, v));
        };
        for v in &self.point {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Polyline>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Polyline {
    fn new() -> Polyline {
        Polyline::new()
    }

    fn descriptor_static(_: ::std::option::Option<Polyline>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "height",
                    Polyline::has_height,
                    Polyline::get_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "point",
                    Polyline::get_point,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Polyline>(
                    "Polyline",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Polyline {
    fn clear(&mut self) {
        self.clear_height();
        self.clear_point();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Polyline {
    fn eq(&self, other: &Polyline) -> bool {
        self.height == other.height &&
        self.point == other.point &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Polyline {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x70, 0x6f, 0x6c, 0x79, 0x6c, 0x69, 0x6e, 0x65, 0x67, 0x65, 0x6f, 0x6d, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x1a, 0x0e, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x40, 0x0a, 0x08, 0x50, 0x6f, 0x6c, 0x79, 0x6c, 0x69, 0x6e, 0x65, 0x12, 0x0e, 0x0a,
    0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x12, 0x24, 0x0a,
    0x05, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x32, 0x64, 0x4a, 0xbf, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0c, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x06, 0x07, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x1c, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0b, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0b, 0x1c, 0x1d,
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
