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
pub struct Shadows {
    // message fields
    field_type: ::std::option::Option<Shadows_ShadowType>,
    color: ::protobuf::SingularPtrField<super::color::Color>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Shadows {}

impl Shadows {
    pub fn new() -> Shadows {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Shadows {
        static mut instance: ::protobuf::lazy::Lazy<Shadows> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Shadows,
        };
        unsafe {
            instance.get(|| {
                Shadows {
                    field_type: ::std::option::Option::None,
                    color: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gazebo.msgs.Shadows.ShadowType type = 5;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Shadows_ShadowType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Shadows_ShadowType {
        self.field_type.unwrap_or(Shadows_ShadowType::STENCIL_ADDITIVE)
    }

    // optional .gazebo.msgs.Color color = 6;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: super::color::Color) {
        self.color = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut super::color::Color {
        if self.color.is_none() {
            self.color.set_default();
        };
        self.color.as_mut().unwrap()
    }

    // Take field
    pub fn take_color(&mut self) -> super::color::Color {
        self.color.take().unwrap_or_else(|| super::color::Color::new())
    }

    pub fn get_color(&self) -> &super::color::Color {
        self.color.as_ref().unwrap_or_else(|| super::color::Color::default_instance())
    }
}

impl ::protobuf::Message for Shadows {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.color));
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        for value in &self.color {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(5, v.value()));
        };
        if let Some(v) = self.color.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Shadows>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Shadows {
    fn new() -> Shadows {
        Shadows::new()
    }

    fn descriptor_static(_: ::std::option::Option<Shadows>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Shadows::has_field_type,
                    Shadows::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "color",
                    Shadows::has_color,
                    Shadows::get_color,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Shadows>(
                    "Shadows",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Shadows {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_color();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Shadows {
    fn eq(&self, other: &Shadows) -> bool {
        self.field_type == other.field_type &&
        self.color == other.color &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Shadows {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Shadows_ShadowType {
    STENCIL_ADDITIVE = 1,
    STENCIL_MODULATIVE = 2,
    TEXTURE_ADDITIVE = 3,
    TEXTURE_MODULATIVE = 4,
}

impl ::protobuf::ProtobufEnum for Shadows_ShadowType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Shadows_ShadowType> {
        match value {
            1 => ::std::option::Option::Some(Shadows_ShadowType::STENCIL_ADDITIVE),
            2 => ::std::option::Option::Some(Shadows_ShadowType::STENCIL_MODULATIVE),
            3 => ::std::option::Option::Some(Shadows_ShadowType::TEXTURE_ADDITIVE),
            4 => ::std::option::Option::Some(Shadows_ShadowType::TEXTURE_MODULATIVE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Shadows_ShadowType] = &[
            Shadows_ShadowType::STENCIL_ADDITIVE,
            Shadows_ShadowType::STENCIL_MODULATIVE,
            Shadows_ShadowType::TEXTURE_ADDITIVE,
            Shadows_ShadowType::TEXTURE_MODULATIVE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Shadows_ShadowType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Shadows_ShadowType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Shadows_ShadowType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0b, 0x63, 0x6f,
    0x6c, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc5, 0x01, 0x0a, 0x07, 0x53, 0x68,
    0x61, 0x64, 0x6f, 0x77, 0x73, 0x12, 0x2d, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x1f, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x2e, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x73, 0x2e, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x21, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67,
    0x73, 0x2e, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x22, 0x68, 0x0a, 0x0a, 0x53, 0x68, 0x61, 0x64, 0x6f,
    0x77, 0x54, 0x79, 0x70, 0x65, 0x12, 0x14, 0x0a, 0x10, 0x53, 0x54, 0x45, 0x4e, 0x43, 0x49, 0x4c,
    0x5f, 0x41, 0x44, 0x44, 0x49, 0x54, 0x49, 0x56, 0x45, 0x10, 0x01, 0x12, 0x16, 0x0a, 0x12, 0x53,
    0x54, 0x45, 0x4e, 0x43, 0x49, 0x4c, 0x5f, 0x4d, 0x4f, 0x44, 0x55, 0x4c, 0x41, 0x54, 0x49, 0x56,
    0x45, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x54, 0x45, 0x58, 0x54, 0x55, 0x52, 0x45, 0x5f, 0x41,
    0x44, 0x44, 0x49, 0x54, 0x49, 0x56, 0x45, 0x10, 0x03, 0x12, 0x16, 0x0a, 0x12, 0x54, 0x45, 0x58,
    0x54, 0x55, 0x52, 0x45, 0x5f, 0x4d, 0x4f, 0x44, 0x55, 0x4c, 0x41, 0x54, 0x49, 0x56, 0x45, 0x10,
    0x04, 0x4a, 0x97, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x14, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07,
    0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04,
    0x00, 0x12, 0x04, 0x0b, 0x02, 0x11, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x0b, 0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0d, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x0d, 0x19, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x0e, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0e, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x0e, 0x19, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f,
    0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f,
    0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f,
    0x19, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x10, 0x04,
    0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x04,
    0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x10, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x02, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x12, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x13, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x13, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x13, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x19, 0x1a,
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
