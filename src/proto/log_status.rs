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
pub struct LogStatus {
    // message fields
    sim_time: ::protobuf::SingularPtrField<super::time::Time>,
    log_file: ::protobuf::SingularPtrField<LogStatus_LogFile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogStatus {}

impl LogStatus {
    pub fn new() -> LogStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogStatus {
        static mut instance: ::protobuf::lazy::Lazy<LogStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogStatus,
        };
        unsafe {
            instance.get(|| {
                LogStatus {
                    sim_time: ::protobuf::SingularPtrField::none(),
                    log_file: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gazebo.msgs.Time sim_time = 1;

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

    // optional .gazebo.msgs.LogStatus.LogFile log_file = 2;

    pub fn clear_log_file(&mut self) {
        self.log_file.clear();
    }

    pub fn has_log_file(&self) -> bool {
        self.log_file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_log_file(&mut self, v: LogStatus_LogFile) {
        self.log_file = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_file(&mut self) -> &mut LogStatus_LogFile {
        if self.log_file.is_none() {
            self.log_file.set_default();
        };
        self.log_file.as_mut().unwrap()
    }

    // Take field
    pub fn take_log_file(&mut self) -> LogStatus_LogFile {
        self.log_file.take().unwrap_or_else(|| LogStatus_LogFile::new())
    }

    pub fn get_log_file(&self) -> &LogStatus_LogFile {
        self.log_file.as_ref().unwrap_or_else(|| LogStatus_LogFile::default_instance())
    }
}

impl ::protobuf::Message for LogStatus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sim_time));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.log_file));
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
        for value in &self.sim_time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.log_file {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sim_time.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.log_file.as_ref() {
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
        ::std::any::TypeId::of::<LogStatus>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogStatus {
    fn new() -> LogStatus {
        LogStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "sim_time",
                    LogStatus::has_sim_time,
                    LogStatus::get_sim_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "log_file",
                    LogStatus::has_log_file,
                    LogStatus::get_log_file,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogStatus>(
                    "LogStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogStatus {
    fn clear(&mut self) {
        self.clear_sim_time();
        self.clear_log_file();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LogStatus {
    fn eq(&self, other: &LogStatus) -> bool {
        self.sim_time == other.sim_time &&
        self.log_file == other.log_file &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LogStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LogStatus_LogFile {
    // message fields
    uri: ::protobuf::SingularField<::std::string::String>,
    base_path: ::protobuf::SingularField<::std::string::String>,
    full_path: ::protobuf::SingularField<::std::string::String>,
    size: ::std::option::Option<f32>,
    size_units: ::std::option::Option<LogStatus_LogFile_Units>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogStatus_LogFile {}

impl LogStatus_LogFile {
    pub fn new() -> LogStatus_LogFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogStatus_LogFile {
        static mut instance: ::protobuf::lazy::Lazy<LogStatus_LogFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogStatus_LogFile,
        };
        unsafe {
            instance.get(|| {
                LogStatus_LogFile {
                    uri: ::protobuf::SingularField::none(),
                    base_path: ::protobuf::SingularField::none(),
                    full_path: ::protobuf::SingularField::none(),
                    size: ::std::option::Option::None,
                    size_units: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string uri = 1;

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

    // optional string base_path = 2;

    pub fn clear_base_path(&mut self) {
        self.base_path.clear();
    }

    pub fn has_base_path(&self) -> bool {
        self.base_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_path(&mut self, v: ::std::string::String) {
        self.base_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base_path(&mut self) -> &mut ::std::string::String {
        if self.base_path.is_none() {
            self.base_path.set_default();
        };
        self.base_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_base_path(&mut self) -> ::std::string::String {
        self.base_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_base_path(&self) -> &str {
        match self.base_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string full_path = 3;

    pub fn clear_full_path(&mut self) {
        self.full_path.clear();
    }

    pub fn has_full_path(&self) -> bool {
        self.full_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_full_path(&mut self, v: ::std::string::String) {
        self.full_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_full_path(&mut self) -> &mut ::std::string::String {
        if self.full_path.is_none() {
            self.full_path.set_default();
        };
        self.full_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_full_path(&mut self) -> ::std::string::String {
        self.full_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_full_path(&self) -> &str {
        match self.full_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional float size = 4;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: f32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> f32 {
        self.size.unwrap_or(0.)
    }

    // optional .gazebo.msgs.LogStatus.LogFile.Units size_units = 5;

    pub fn clear_size_units(&mut self) {
        self.size_units = ::std::option::Option::None;
    }

    pub fn has_size_units(&self) -> bool {
        self.size_units.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size_units(&mut self, v: LogStatus_LogFile_Units) {
        self.size_units = ::std::option::Option::Some(v);
    }

    pub fn get_size_units(&self) -> LogStatus_LogFile_Units {
        self.size_units.unwrap_or(LogStatus_LogFile_Units::BYTES)
    }
}

impl ::protobuf::Message for LogStatus_LogFile {
    fn is_initialized(&self) -> bool {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.base_path));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.full_path));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.size = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.size_units = ::std::option::Option::Some(tmp);
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
        for value in &self.base_path {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.full_path {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if self.size.is_some() {
            my_size += 5;
        };
        for value in &self.size_units {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uri.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.base_path.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.full_path.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.size {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.size_units {
            try!(os.write_enum(5, v.value()));
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
        ::std::any::TypeId::of::<LogStatus_LogFile>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogStatus_LogFile {
    fn new() -> LogStatus_LogFile {
        LogStatus_LogFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogStatus_LogFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "uri",
                    LogStatus_LogFile::has_uri,
                    LogStatus_LogFile::get_uri,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "base_path",
                    LogStatus_LogFile::has_base_path,
                    LogStatus_LogFile::get_base_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "full_path",
                    LogStatus_LogFile::has_full_path,
                    LogStatus_LogFile::get_full_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "size",
                    LogStatus_LogFile::has_size,
                    LogStatus_LogFile::get_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "size_units",
                    LogStatus_LogFile::has_size_units,
                    LogStatus_LogFile::get_size_units,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogStatus_LogFile>(
                    "LogStatus_LogFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogStatus_LogFile {
    fn clear(&mut self) {
        self.clear_uri();
        self.clear_base_path();
        self.clear_full_path();
        self.clear_size();
        self.clear_size_units();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LogStatus_LogFile {
    fn eq(&self, other: &LogStatus_LogFile) -> bool {
        self.uri == other.uri &&
        self.base_path == other.base_path &&
        self.full_path == other.full_path &&
        self.size == other.size &&
        self.size_units == other.size_units &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LogStatus_LogFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum LogStatus_LogFile_Units {
    BYTES = 1,
    K_BYTES = 2,
    M_BYTES = 3,
    G_BYTES = 4,
}

impl ::protobuf::ProtobufEnum for LogStatus_LogFile_Units {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LogStatus_LogFile_Units> {
        match value {
            1 => ::std::option::Option::Some(LogStatus_LogFile_Units::BYTES),
            2 => ::std::option::Option::Some(LogStatus_LogFile_Units::K_BYTES),
            3 => ::std::option::Option::Some(LogStatus_LogFile_Units::M_BYTES),
            4 => ::std::option::Option::Some(LogStatus_LogFile_Units::G_BYTES),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [LogStatus_LogFile_Units] = &[
            LogStatus_LogFile_Units::BYTES,
            LogStatus_LogFile_Units::K_BYTES,
            LogStatus_LogFile_Units::M_BYTES,
            LogStatus_LogFile_Units::G_BYTES,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<LogStatus_LogFile_Units>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("LogStatus_LogFile_Units", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for LogStatus_LogFile_Units {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x6c, 0x6f, 0x67, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0b, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a,
    0x0a, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa4, 0x02, 0x0a, 0x09,
    0x4c, 0x6f, 0x67, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x23, 0x0a, 0x08, 0x73, 0x69, 0x6d,
    0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61,
    0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x30,
    0x0a, 0x08, 0x6c, 0x6f, 0x67, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1e, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x4c,
    0x6f, 0x67, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x4c, 0x6f, 0x67, 0x46, 0x69, 0x6c, 0x65,
    0x1a, 0xbf, 0x01, 0x0a, 0x07, 0x4c, 0x6f, 0x67, 0x46, 0x69, 0x6c, 0x65, 0x12, 0x0b, 0x0a, 0x03,
    0x75, 0x72, 0x69, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x62, 0x61, 0x73,
    0x65, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09,
    0x66, 0x75, 0x6c, 0x6c, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x0c, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x12, 0x38, 0x0a,
    0x0a, 0x73, 0x69, 0x7a, 0x65, 0x5f, 0x75, 0x6e, 0x69, 0x74, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x24, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e,
    0x4c, 0x6f, 0x67, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x4c, 0x6f, 0x67, 0x46, 0x69, 0x6c,
    0x65, 0x2e, 0x55, 0x6e, 0x69, 0x74, 0x73, 0x22, 0x39, 0x0a, 0x05, 0x55, 0x6e, 0x69, 0x74, 0x73,
    0x12, 0x09, 0x0a, 0x05, 0x42, 0x59, 0x54, 0x45, 0x53, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x4b,
    0x5f, 0x42, 0x59, 0x54, 0x45, 0x53, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x4d, 0x5f, 0x42, 0x59,
    0x54, 0x45, 0x53, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x47, 0x5f, 0x42, 0x59, 0x54, 0x45, 0x53,
    0x10, 0x04, 0x4a, 0xda, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1d, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06,
    0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x1d, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00,
    0x03, 0x00, 0x12, 0x04, 0x0a, 0x02, 0x19, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x0a, 0x0a, 0x11, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00,
    0x12, 0x04, 0x0c, 0x04, 0x12, 0x05, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0c, 0x09, 0x0e, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x0e, 0x06, 0x10, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x06, 0x0b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03,
    0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x0e, 0x0f, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x00, 0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x06, 0x12, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x00, 0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x06, 0x0d, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0f, 0x10, 0x11,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x06,
    0x12, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x10, 0x06, 0x0d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x10, 0x10, 0x11, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x11, 0x06, 0x12, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x06, 0x0d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x03, 0x00,
    0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x11, 0x10, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x04, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x14, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x15, 0x04, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x14, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x16, 0x04, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x16, 0x14, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x16, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x17, 0x04, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x17, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x17, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x17, 0x13, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x17, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x18, 0x04, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x18, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x18, 0x13, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x18, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b,
    0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1b, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x1c, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x13,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x1f, 0x20,
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
