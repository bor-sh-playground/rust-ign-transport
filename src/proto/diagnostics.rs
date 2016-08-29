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
pub struct Diagnostics {
    // message fields
    time: ::protobuf::RepeatedField<Diagnostics_DiagTime>,
    real_time: ::protobuf::SingularPtrField<super::time::Time>,
    sim_time: ::protobuf::SingularPtrField<super::time::Time>,
    real_time_factor: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Diagnostics {}

impl Diagnostics {
    pub fn new() -> Diagnostics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Diagnostics {
        static mut instance: ::protobuf::lazy::Lazy<Diagnostics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Diagnostics,
        };
        unsafe {
            instance.get(|| {
                Diagnostics {
                    time: ::protobuf::RepeatedField::new(),
                    real_time: ::protobuf::SingularPtrField::none(),
                    sim_time: ::protobuf::SingularPtrField::none(),
                    real_time_factor: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .gazebo.msgs.Diagnostics.DiagTime time = 1;

    pub fn clear_time(&mut self) {
        self.time.clear();
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: ::protobuf::RepeatedField<Diagnostics_DiagTime>) {
        self.time = v;
    }

    // Mutable pointer to the field.
    pub fn mut_time(&mut self) -> &mut ::protobuf::RepeatedField<Diagnostics_DiagTime> {
        &mut self.time
    }

    // Take field
    pub fn take_time(&mut self) -> ::protobuf::RepeatedField<Diagnostics_DiagTime> {
        ::std::mem::replace(&mut self.time, ::protobuf::RepeatedField::new())
    }

    pub fn get_time(&self) -> &[Diagnostics_DiagTime] {
        &self.time
    }

    // required .gazebo.msgs.Time real_time = 2;

    pub fn clear_real_time(&mut self) {
        self.real_time.clear();
    }

    pub fn has_real_time(&self) -> bool {
        self.real_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_real_time(&mut self, v: super::time::Time) {
        self.real_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_real_time(&mut self) -> &mut super::time::Time {
        if self.real_time.is_none() {
            self.real_time.set_default();
        };
        self.real_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_real_time(&mut self) -> super::time::Time {
        self.real_time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_real_time(&self) -> &super::time::Time {
        self.real_time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // required .gazebo.msgs.Time sim_time = 3;

    pub fn clear_sim_time(&mut self) {
        self.sim_time.clear();
    }

    pub fn has_sim_time(&self) -> bool {
        self.sim_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sim_time(&mut self, v: super::time::Time) {
        self.sim_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sim_time(&mut self) -> &mut super::time::Time {
        if self.sim_time.is_none() {
            self.sim_time.set_default();
        };
        self.sim_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_sim_time(&mut self) -> super::time::Time {
        self.sim_time.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_sim_time(&self) -> &super::time::Time {
        self.sim_time.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // required double real_time_factor = 4;

    pub fn clear_real_time_factor(&mut self) {
        self.real_time_factor = ::std::option::Option::None;
    }

    pub fn has_real_time_factor(&self) -> bool {
        self.real_time_factor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_real_time_factor(&mut self, v: f64) {
        self.real_time_factor = ::std::option::Option::Some(v);
    }

    pub fn get_real_time_factor(&self) -> f64 {
        self.real_time_factor.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Diagnostics {
    fn is_initialized(&self) -> bool {
        if self.real_time.is_none() {
            return false;
        };
        if self.sim_time.is_none() {
            return false;
        };
        if self.real_time_factor.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.time));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.real_time));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sim_time));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.real_time_factor = ::std::option::Option::Some(tmp);
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
        for value in &self.real_time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sim_time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.real_time_factor.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.time {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.real_time.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.sim_time.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.real_time_factor {
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
        ::std::any::TypeId::of::<Diagnostics>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Diagnostics {
    fn new() -> Diagnostics {
        Diagnostics::new()
    }

    fn descriptor_static(_: ::std::option::Option<Diagnostics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "time",
                    Diagnostics::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "real_time",
                    Diagnostics::has_real_time,
                    Diagnostics::get_real_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "sim_time",
                    Diagnostics::has_sim_time,
                    Diagnostics::get_sim_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "real_time_factor",
                    Diagnostics::has_real_time_factor,
                    Diagnostics::get_real_time_factor,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Diagnostics>(
                    "Diagnostics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Diagnostics {
    fn clear(&mut self) {
        self.clear_time();
        self.clear_real_time();
        self.clear_sim_time();
        self.clear_real_time_factor();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Diagnostics {
    fn eq(&self, other: &Diagnostics) -> bool {
        self.time == other.time &&
        self.real_time == other.real_time &&
        self.sim_time == other.sim_time &&
        self.real_time_factor == other.real_time_factor &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Diagnostics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Diagnostics_DiagTime {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    elapsed: ::protobuf::SingularPtrField<super::time::Time>,
    wall: ::protobuf::SingularPtrField<super::time::Time>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Diagnostics_DiagTime {}

impl Diagnostics_DiagTime {
    pub fn new() -> Diagnostics_DiagTime {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Diagnostics_DiagTime {
        static mut instance: ::protobuf::lazy::Lazy<Diagnostics_DiagTime> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Diagnostics_DiagTime,
        };
        unsafe {
            instance.get(|| {
                Diagnostics_DiagTime {
                    name: ::protobuf::SingularField::none(),
                    elapsed: ::protobuf::SingularPtrField::none(),
                    wall: ::protobuf::SingularPtrField::none(),
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

    // required .gazebo.msgs.Time elapsed = 2;

    pub fn clear_elapsed(&mut self) {
        self.elapsed.clear();
    }

    pub fn has_elapsed(&self) -> bool {
        self.elapsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_elapsed(&mut self, v: super::time::Time) {
        self.elapsed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_elapsed(&mut self) -> &mut super::time::Time {
        if self.elapsed.is_none() {
            self.elapsed.set_default();
        };
        self.elapsed.as_mut().unwrap()
    }

    // Take field
    pub fn take_elapsed(&mut self) -> super::time::Time {
        self.elapsed.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_elapsed(&self) -> &super::time::Time {
        self.elapsed.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }

    // required .gazebo.msgs.Time wall = 3;

    pub fn clear_wall(&mut self) {
        self.wall.clear();
    }

    pub fn has_wall(&self) -> bool {
        self.wall.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wall(&mut self, v: super::time::Time) {
        self.wall = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wall(&mut self) -> &mut super::time::Time {
        if self.wall.is_none() {
            self.wall.set_default();
        };
        self.wall.as_mut().unwrap()
    }

    // Take field
    pub fn take_wall(&mut self) -> super::time::Time {
        self.wall.take().unwrap_or_else(|| super::time::Time::new())
    }

    pub fn get_wall(&self) -> &super::time::Time {
        self.wall.as_ref().unwrap_or_else(|| super::time::Time::default_instance())
    }
}

impl ::protobuf::Message for Diagnostics_DiagTime {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.elapsed.is_none() {
            return false;
        };
        if self.wall.is_none() {
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.elapsed));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.wall));
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
        for value in &self.elapsed {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.wall {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.elapsed.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.wall.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Diagnostics_DiagTime>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Diagnostics_DiagTime {
    fn new() -> Diagnostics_DiagTime {
        Diagnostics_DiagTime::new()
    }

    fn descriptor_static(_: ::std::option::Option<Diagnostics_DiagTime>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Diagnostics_DiagTime::has_name,
                    Diagnostics_DiagTime::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "elapsed",
                    Diagnostics_DiagTime::has_elapsed,
                    Diagnostics_DiagTime::get_elapsed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "wall",
                    Diagnostics_DiagTime::has_wall,
                    Diagnostics_DiagTime::get_wall,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Diagnostics_DiagTime>(
                    "Diagnostics_DiagTime",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Diagnostics_DiagTime {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_elapsed();
        self.clear_wall();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Diagnostics_DiagTime {
    fn eq(&self, other: &Diagnostics_DiagTime) -> bool {
        self.name == other.name &&
        self.elapsed == other.elapsed &&
        self.wall == other.wall &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Diagnostics_DiagTime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x64, 0x69, 0x61, 0x67, 0x6e, 0x6f, 0x73, 0x74, 0x69, 0x63, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73,
    0x1a, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x82, 0x02, 0x0a,
    0x0b, 0x44, 0x69, 0x61, 0x67, 0x6e, 0x6f, 0x73, 0x74, 0x69, 0x63, 0x73, 0x12, 0x2f, 0x0a, 0x04,
    0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x67, 0x61, 0x7a,
    0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x44, 0x69, 0x61, 0x67, 0x6e, 0x6f, 0x73,
    0x74, 0x69, 0x63, 0x73, 0x2e, 0x44, 0x69, 0x61, 0x67, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x24, 0x0a,
    0x09, 0x72, 0x65, 0x61, 0x6c, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54,
    0x69, 0x6d, 0x65, 0x12, 0x23, 0x0a, 0x08, 0x73, 0x69, 0x6d, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d,
    0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x10, 0x72, 0x65, 0x61, 0x6c,
    0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x02,
    0x28, 0x01, 0x1a, 0x5d, 0x0a, 0x08, 0x44, 0x69, 0x61, 0x67, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x0c,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x22, 0x0a, 0x07,
    0x65, 0x6c, 0x61, 0x70, 0x73, 0x65, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65,
    0x12, 0x1f, 0x0a, 0x04, 0x77, 0x61, 0x6c, 0x6c, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d,
    0x65, 0x4a, 0xd2, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x16, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07,
    0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03,
    0x00, 0x12, 0x04, 0x0b, 0x02, 0x10, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01,
    0x12, 0x03, 0x0b, 0x0a, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0d, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0d, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x14, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0d, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x0e, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x0e, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x0e, 0x0d, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0e, 0x12, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0e, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f,
    0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f,
    0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0f,
    0x0d, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f,
    0x12, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x02, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x12, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x13, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x13,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x13, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x10, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x14, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x14, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x14, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x14, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x15, 0x02, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x15, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x15, 0x25, 0x26,
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
