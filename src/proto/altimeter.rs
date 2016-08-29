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
pub struct Altimeter {
    // message fields
    time: ::protobuf::SingularPtrField<super::time::Time>,
    vertical_position: ::std::option::Option<f64>,
    vertical_velocity: ::std::option::Option<f64>,
    vertical_reference: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Altimeter {}

impl Altimeter {
    pub fn new() -> Altimeter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Altimeter {
        static mut instance: ::protobuf::lazy::Lazy<Altimeter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Altimeter,
        };
        unsafe {
            instance.get(|| {
                Altimeter {
                    time: ::protobuf::SingularPtrField::none(),
                    vertical_position: ::std::option::Option::None,
                    vertical_velocity: ::std::option::Option::None,
                    vertical_reference: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Time time = 1;

    pub fn clear_time(&mut self) {
        self.time.clear();
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: super::time::Time) {
        self.time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_time(&mut self) -> &mut super::time::Time {
        if self.time.is_none() {
            self.time.set_default();
        };
        self.time.as_mut().unwrap()
    }

    // Take field
    pub fn take_time(&mut self) -> super::time::Time {
        self.time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_time(&self) -> &super::time::Time {
        self.time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // required double vertical_position = 2;

    pub fn clear_vertical_position(&mut self) {
        self.vertical_position = ::std::option::Option::None;
    }

    pub fn has_vertical_position(&self) -> bool {
        self.vertical_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_position(&mut self, v: f64) {
        self.vertical_position = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_position(&self) -> f64 {
        self.vertical_position.unwrap_or(0.)
    }

    // required double vertical_velocity = 3;

    pub fn clear_vertical_velocity(&mut self) {
        self.vertical_velocity = ::std::option::Option::None;
    }

    pub fn has_vertical_velocity(&self) -> bool {
        self.vertical_velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_velocity(&mut self, v: f64) {
        self.vertical_velocity = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_velocity(&self) -> f64 {
        self.vertical_velocity.unwrap_or(0.)
    }

    // required double vertical_reference = 4;

    pub fn clear_vertical_reference(&mut self) {
        self.vertical_reference = ::std::option::Option::None;
    }

    pub fn has_vertical_reference(&self) -> bool {
        self.vertical_reference.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_reference(&mut self, v: f64) {
        self.vertical_reference = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_reference(&self) -> f64 {
        self.vertical_reference.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Altimeter {
    fn is_initialized(&self) -> bool {
        if self.time.is_none() {
            return false;
        };
        if self.vertical_position.is_none() {
            return false;
        };
        if self.vertical_velocity.is_none() {
            return false;
        };
        if self.vertical_reference.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.time));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vertical_position = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vertical_velocity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vertical_reference = ::std::option::Option::Some(tmp);
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
        for value in &self.time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.vertical_position.is_some() {
            my_size += 9;
        };
        if self.vertical_velocity.is_some() {
            my_size += 9;
        };
        if self.vertical_reference.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.vertical_position {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.vertical_velocity {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.vertical_reference {
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
        ::std::any::TypeId::of::<Altimeter>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Altimeter {
    fn new() -> Altimeter {
        Altimeter::new()
    }

    fn descriptor_static(_: ::std::option::Option<Altimeter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "time",
                    Altimeter::has_time,
                    Altimeter::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vertical_position",
                    Altimeter::has_vertical_position,
                    Altimeter::get_vertical_position,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vertical_velocity",
                    Altimeter::has_vertical_velocity,
                    Altimeter::get_vertical_velocity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vertical_reference",
                    Altimeter::has_vertical_reference,
                    Altimeter::get_vertical_reference,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Altimeter>(
                    "Altimeter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Altimeter {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_vertical_position();
        self.clear_vertical_velocity();
        self.clear_vertical_reference();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Altimeter {
    fn eq(&self, other: &Altimeter) -> bool {
        self.time == other.time &&
        self.vertical_position == other.vertical_position &&
        self.vertical_velocity == other.vertical_velocity &&
        self.vertical_reference == other.vertical_reference &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Altimeter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x61, 0x6c, 0x74, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a,
    0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x7e, 0x0a, 0x09, 0x41, 0x6c,
    0x74, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x12, 0x1f, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x19, 0x0a, 0x11, 0x76, 0x65, 0x72, 0x74,
    0x69, 0x63, 0x61, 0x6c, 0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x01, 0x12, 0x19, 0x0a, 0x11, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f,
    0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x12, 0x1a,
    0x0a, 0x12, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x66, 0x65, 0x72,
    0x65, 0x6e, 0x63, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x01, 0x4a, 0x97, 0x04, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x16, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x09, 0x00, 0x16, 0x01, 0x1a, 0x1f, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66,
    0x20, 0x41, 0x6c, 0x74, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x20, 0x73, 0x65, 0x6e, 0x73, 0x6f,
    0x72, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x08, 0x11, 0x0a, 0x36, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x29,
    0x1a, 0x29, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x6c, 0x74, 0x69,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0c, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0c, 0x27, 0x28, 0x0a, 0x39, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x29,
    0x1a, 0x2c, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x56, 0x65, 0x72, 0x74, 0x69,
    0x63, 0x61, 0x6c, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x2c, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0f, 0x27, 0x28, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x12, 0x02, 0x29, 0x1a, 0x33, 0x2f, 0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x56, 0x65,
    0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x20, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x20,
    0x64, 0x61, 0x74, 0x61, 0x2c, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2f,
    0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12,
    0x12, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x27, 0x28,
    0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x15, 0x02, 0x29, 0x1a, 0x1d, 0x2f,
    0x20, 0x5c, 0x62, 0x72, 0x69, 0x65, 0x66, 0x20, 0x56, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c,
    0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x15, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x15, 0x27, 0x28,
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
