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
pub struct Hydra {
    // message fields
    right: ::protobuf::SingularPtrField<Hydra_Paddle>,
    left: ::protobuf::SingularPtrField<Hydra_Paddle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Hydra {}

impl Hydra {
    pub fn new() -> Hydra {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Hydra {
        static mut instance: ::protobuf::lazy::Lazy<Hydra> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Hydra,
        };
        unsafe {
            instance.get(|| {
                Hydra {
                    right: ::protobuf::SingularPtrField::none(),
                    left: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Hydra.Paddle right = 1;

    pub fn clear_right(&mut self) {
        self.right.clear();
    }

    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right(&mut self, v: Hydra_Paddle) {
        self.right = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_right(&mut self) -> &mut Hydra_Paddle {
        if self.right.is_none() {
            self.right.set_default();
        };
        self.right.as_mut().unwrap()
    }

    // Take field
    pub fn take_right(&mut self) -> Hydra_Paddle {
        self.right.take().unwrap_or_else(|| Hydra_Paddle::new())
    }

    pub fn get_right(&self) -> &Hydra_Paddle {
        self.right.as_ref().unwrap_or_else(|| Hydra_Paddle::default_instance())
    }

    // required .gazebo.msgs.Hydra.Paddle left = 2;

    pub fn clear_left(&mut self) {
        self.left.clear();
    }

    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }

    // Param is passed by value, moved
    pub fn set_left(&mut self, v: Hydra_Paddle) {
        self.left = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_left(&mut self) -> &mut Hydra_Paddle {
        if self.left.is_none() {
            self.left.set_default();
        };
        self.left.as_mut().unwrap()
    }

    // Take field
    pub fn take_left(&mut self) -> Hydra_Paddle {
        self.left.take().unwrap_or_else(|| Hydra_Paddle::new())
    }

    pub fn get_left(&self) -> &Hydra_Paddle {
        self.left.as_ref().unwrap_or_else(|| Hydra_Paddle::default_instance())
    }
}

impl ::protobuf::Message for Hydra {
    fn is_initialized(&self) -> bool {
        if self.right.is_none() {
            return false;
        };
        if self.left.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.right));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.left));
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
        for value in &self.right {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.left {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.right.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.left.as_ref() {
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
        ::std::any::TypeId::of::<Hydra>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Hydra {
    fn new() -> Hydra {
        Hydra::new()
    }

    fn descriptor_static(_: ::std::option::Option<Hydra>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "right",
                    Hydra::has_right,
                    Hydra::get_right,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "left",
                    Hydra::has_left,
                    Hydra::get_left,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Hydra>(
                    "Hydra",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Hydra {
    fn clear(&mut self) {
        self.clear_right();
        self.clear_left();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Hydra {
    fn eq(&self, other: &Hydra) -> bool {
        self.right == other.right &&
        self.left == other.left &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Hydra {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Hydra_Paddle {
    // message fields
    pose: ::protobuf::SingularPtrField<super::pose::Pose>,
    button_bumper: ::std::option::Option<bool>,
    button_1: ::std::option::Option<bool>,
    button_2: ::std::option::Option<bool>,
    button_3: ::std::option::Option<bool>,
    button_4: ::std::option::Option<bool>,
    button_joy: ::std::option::Option<bool>,
    button_center: ::std::option::Option<bool>,
    joy_x: ::std::option::Option<f64>,
    joy_y: ::std::option::Option<f64>,
    trigger: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Hydra_Paddle {}

impl Hydra_Paddle {
    pub fn new() -> Hydra_Paddle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Hydra_Paddle {
        static mut instance: ::protobuf::lazy::Lazy<Hydra_Paddle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Hydra_Paddle,
        };
        unsafe {
            instance.get(|| {
                Hydra_Paddle {
                    pose: ::protobuf::SingularPtrField::none(),
                    button_bumper: ::std::option::Option::None,
                    button_1: ::std::option::Option::None,
                    button_2: ::std::option::Option::None,
                    button_3: ::std::option::Option::None,
                    button_4: ::std::option::Option::None,
                    button_joy: ::std::option::Option::None,
                    button_center: ::std::option::Option::None,
                    joy_x: ::std::option::Option::None,
                    joy_y: ::std::option::Option::None,
                    trigger: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gazebo.msgs.Pose pose = 1;

    pub fn clear_pose(&mut self) {
        self.pose.clear();
    }

    pub fn has_pose(&self) -> bool {
        self.pose.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pose(&mut self, v: super::pose::Pose) {
        self.pose = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pose(&mut self) -> &mut super::pose::Pose {
        if self.pose.is_none() {
            self.pose.set_default();
        };
        self.pose.as_mut().unwrap()
    }

    // Take field
    pub fn take_pose(&mut self) -> super::pose::Pose {
        self.pose.take().unwrap_or_else(|| super::pose::Pose::new())
    }

    pub fn get_pose(&self) -> &super::pose::Pose {
        self.pose.as_ref().unwrap_or_else(|| super::pose::Pose::default_instance())
    }

    // required bool button_bumper = 2;

    pub fn clear_button_bumper(&mut self) {
        self.button_bumper = ::std::option::Option::None;
    }

    pub fn has_button_bumper(&self) -> bool {
        self.button_bumper.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button_bumper(&mut self, v: bool) {
        self.button_bumper = ::std::option::Option::Some(v);
    }

    pub fn get_button_bumper(&self) -> bool {
        self.button_bumper.unwrap_or(false)
    }

    // required bool button_1 = 3;

    pub fn clear_button_1(&mut self) {
        self.button_1 = ::std::option::Option::None;
    }

    pub fn has_button_1(&self) -> bool {
        self.button_1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button_1(&mut self, v: bool) {
        self.button_1 = ::std::option::Option::Some(v);
    }

    pub fn get_button_1(&self) -> bool {
        self.button_1.unwrap_or(false)
    }

    // required bool button_2 = 4;

    pub fn clear_button_2(&mut self) {
        self.button_2 = ::std::option::Option::None;
    }

    pub fn has_button_2(&self) -> bool {
        self.button_2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button_2(&mut self, v: bool) {
        self.button_2 = ::std::option::Option::Some(v);
    }

    pub fn get_button_2(&self) -> bool {
        self.button_2.unwrap_or(false)
    }

    // required bool button_3 = 5;

    pub fn clear_button_3(&mut self) {
        self.button_3 = ::std::option::Option::None;
    }

    pub fn has_button_3(&self) -> bool {
        self.button_3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button_3(&mut self, v: bool) {
        self.button_3 = ::std::option::Option::Some(v);
    }

    pub fn get_button_3(&self) -> bool {
        self.button_3.unwrap_or(false)
    }

    // required bool button_4 = 6;

    pub fn clear_button_4(&mut self) {
        self.button_4 = ::std::option::Option::None;
    }

    pub fn has_button_4(&self) -> bool {
        self.button_4.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button_4(&mut self, v: bool) {
        self.button_4 = ::std::option::Option::Some(v);
    }

    pub fn get_button_4(&self) -> bool {
        self.button_4.unwrap_or(false)
    }

    // required bool button_joy = 7;

    pub fn clear_button_joy(&mut self) {
        self.button_joy = ::std::option::Option::None;
    }

    pub fn has_button_joy(&self) -> bool {
        self.button_joy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button_joy(&mut self, v: bool) {
        self.button_joy = ::std::option::Option::Some(v);
    }

    pub fn get_button_joy(&self) -> bool {
        self.button_joy.unwrap_or(false)
    }

    // required bool button_center = 8;

    pub fn clear_button_center(&mut self) {
        self.button_center = ::std::option::Option::None;
    }

    pub fn has_button_center(&self) -> bool {
        self.button_center.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button_center(&mut self, v: bool) {
        self.button_center = ::std::option::Option::Some(v);
    }

    pub fn get_button_center(&self) -> bool {
        self.button_center.unwrap_or(false)
    }

    // required double joy_x = 9;

    pub fn clear_joy_x(&mut self) {
        self.joy_x = ::std::option::Option::None;
    }

    pub fn has_joy_x(&self) -> bool {
        self.joy_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_joy_x(&mut self, v: f64) {
        self.joy_x = ::std::option::Option::Some(v);
    }

    pub fn get_joy_x(&self) -> f64 {
        self.joy_x.unwrap_or(0.)
    }

    // required double joy_y = 10;

    pub fn clear_joy_y(&mut self) {
        self.joy_y = ::std::option::Option::None;
    }

    pub fn has_joy_y(&self) -> bool {
        self.joy_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_joy_y(&mut self, v: f64) {
        self.joy_y = ::std::option::Option::Some(v);
    }

    pub fn get_joy_y(&self) -> f64 {
        self.joy_y.unwrap_or(0.)
    }

    // required double trigger = 11;

    pub fn clear_trigger(&mut self) {
        self.trigger = ::std::option::Option::None;
    }

    pub fn has_trigger(&self) -> bool {
        self.trigger.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trigger(&mut self, v: f64) {
        self.trigger = ::std::option::Option::Some(v);
    }

    pub fn get_trigger(&self) -> f64 {
        self.trigger.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Hydra_Paddle {
    fn is_initialized(&self) -> bool {
        if self.pose.is_none() {
            return false;
        };
        if self.button_bumper.is_none() {
            return false;
        };
        if self.button_1.is_none() {
            return false;
        };
        if self.button_2.is_none() {
            return false;
        };
        if self.button_3.is_none() {
            return false;
        };
        if self.button_4.is_none() {
            return false;
        };
        if self.button_joy.is_none() {
            return false;
        };
        if self.button_center.is_none() {
            return false;
        };
        if self.joy_x.is_none() {
            return false;
        };
        if self.joy_y.is_none() {
            return false;
        };
        if self.trigger.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pose));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.button_bumper = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.button_1 = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.button_2 = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.button_3 = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.button_4 = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.button_joy = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.button_center = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.joy_x = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.joy_y = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.trigger = ::std::option::Option::Some(tmp);
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
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.button_bumper.is_some() {
            my_size += 2;
        };
        if self.button_1.is_some() {
            my_size += 2;
        };
        if self.button_2.is_some() {
            my_size += 2;
        };
        if self.button_3.is_some() {
            my_size += 2;
        };
        if self.button_4.is_some() {
            my_size += 2;
        };
        if self.button_joy.is_some() {
            my_size += 2;
        };
        if self.button_center.is_some() {
            my_size += 2;
        };
        if self.joy_x.is_some() {
            my_size += 9;
        };
        if self.joy_y.is_some() {
            my_size += 9;
        };
        if self.trigger.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pose.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.button_bumper {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.button_1 {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.button_2 {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.button_3 {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.button_4 {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.button_joy {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.button_center {
            try!(os.write_bool(8, v));
        };
        if let Some(v) = self.joy_x {
            try!(os.write_double(9, v));
        };
        if let Some(v) = self.joy_y {
            try!(os.write_double(10, v));
        };
        if let Some(v) = self.trigger {
            try!(os.write_double(11, v));
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
        ::std::any::TypeId::of::<Hydra_Paddle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Hydra_Paddle {
    fn new() -> Hydra_Paddle {
        Hydra_Paddle::new()
    }

    fn descriptor_static(_: ::std::option::Option<Hydra_Paddle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pose",
                    Hydra_Paddle::has_pose,
                    Hydra_Paddle::get_pose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "button_bumper",
                    Hydra_Paddle::has_button_bumper,
                    Hydra_Paddle::get_button_bumper,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "button_1",
                    Hydra_Paddle::has_button_1,
                    Hydra_Paddle::get_button_1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "button_2",
                    Hydra_Paddle::has_button_2,
                    Hydra_Paddle::get_button_2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "button_3",
                    Hydra_Paddle::has_button_3,
                    Hydra_Paddle::get_button_3,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "button_4",
                    Hydra_Paddle::has_button_4,
                    Hydra_Paddle::get_button_4,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "button_joy",
                    Hydra_Paddle::has_button_joy,
                    Hydra_Paddle::get_button_joy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "button_center",
                    Hydra_Paddle::has_button_center,
                    Hydra_Paddle::get_button_center,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "joy_x",
                    Hydra_Paddle::has_joy_x,
                    Hydra_Paddle::get_joy_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "joy_y",
                    Hydra_Paddle::has_joy_y,
                    Hydra_Paddle::get_joy_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "trigger",
                    Hydra_Paddle::has_trigger,
                    Hydra_Paddle::get_trigger,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Hydra_Paddle>(
                    "Hydra_Paddle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Hydra_Paddle {
    fn clear(&mut self) {
        self.clear_pose();
        self.clear_button_bumper();
        self.clear_button_1();
        self.clear_button_2();
        self.clear_button_3();
        self.clear_button_4();
        self.clear_button_joy();
        self.clear_button_center();
        self.clear_joy_x();
        self.clear_joy_y();
        self.clear_trigger();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Hydra_Paddle {
    fn eq(&self, other: &Hydra_Paddle) -> bool {
        self.pose == other.pose &&
        self.button_bumper == other.button_bumper &&
        self.button_1 == other.button_1 &&
        self.button_2 == other.button_2 &&
        self.button_3 == other.button_3 &&
        self.button_4 == other.button_4 &&
        self.button_joy == other.button_joy &&
        self.button_center == other.button_center &&
        self.joy_x == other.joy_x &&
        self.joy_y == other.joy_y &&
        self.trigger == other.trigger &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Hydra_Paddle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x68, 0x79, 0x64, 0x72, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x0a, 0x70, 0x6f, 0x73, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xbf, 0x02, 0x0a, 0x05, 0x48, 0x79, 0x64, 0x72, 0x61,
    0x12, 0x28, 0x0a, 0x05, 0x72, 0x69, 0x67, 0x68, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x19, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x48, 0x79,
    0x64, 0x72, 0x61, 0x2e, 0x50, 0x61, 0x64, 0x64, 0x6c, 0x65, 0x12, 0x27, 0x0a, 0x04, 0x6c, 0x65,
    0x66, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x61, 0x7a, 0x65, 0x62,
    0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x48, 0x79, 0x64, 0x72, 0x61, 0x2e, 0x50, 0x61, 0x64,
    0x64, 0x6c, 0x65, 0x1a, 0xe2, 0x01, 0x0a, 0x06, 0x50, 0x61, 0x64, 0x64, 0x6c, 0x65, 0x12, 0x1f,
    0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67,
    0x61, 0x7a, 0x65, 0x62, 0x6f, 0x2e, 0x6d, 0x73, 0x67, 0x73, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x12,
    0x15, 0x0a, 0x0d, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x5f, 0x62, 0x75, 0x6d, 0x70, 0x65, 0x72,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x08, 0x12, 0x10, 0x0a, 0x08, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e,
    0x5f, 0x31, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x12, 0x10, 0x0a, 0x08, 0x62, 0x75, 0x74, 0x74,
    0x6f, 0x6e, 0x5f, 0x32, 0x18, 0x04, 0x20, 0x02, 0x28, 0x08, 0x12, 0x10, 0x0a, 0x08, 0x62, 0x75,
    0x74, 0x74, 0x6f, 0x6e, 0x5f, 0x33, 0x18, 0x05, 0x20, 0x02, 0x28, 0x08, 0x12, 0x10, 0x0a, 0x08,
    0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x5f, 0x34, 0x18, 0x06, 0x20, 0x02, 0x28, 0x08, 0x12, 0x12,
    0x0a, 0x0a, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x5f, 0x6a, 0x6f, 0x79, 0x18, 0x07, 0x20, 0x02,
    0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x5f, 0x63, 0x65, 0x6e,
    0x74, 0x65, 0x72, 0x18, 0x08, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6a, 0x6f, 0x79,
    0x5f, 0x78, 0x18, 0x09, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0d, 0x0a, 0x05, 0x6a, 0x6f, 0x79, 0x5f,
    0x79, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x72, 0x69, 0x67, 0x67,
    0x65, 0x72, 0x18, 0x0b, 0x20, 0x02, 0x28, 0x01, 0x4a, 0xe5, 0x0b, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x33, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x13, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x08, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x0a, 0x02, 0x2c, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x0a, 0x10, 0x0a, 0x23, 0x0a, 0x06,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x24, 0x1a, 0x14, 0x20, 0x50, 0x6f,
    0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x64, 0x64, 0x6c, 0x65,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x04,
    0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0d, 0x0d,
    0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x12,
    0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x22,
    0x23, 0x0a, 0x26, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x10, 0x04, 0x24,
    0x1a, 0x17, 0x20, 0x54, 0x68, 0x65, 0x20, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x20, 0x6c, 0x61,
    0x62, 0x65, 0x6c, 0x65, 0x64, 0x20, 0x4c, 0x42, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x10, 0x0d, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x12, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x22, 0x23, 0x0a, 0x19, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x13, 0x04, 0x24, 0x1a, 0x0a, 0x20, 0x42, 0x75, 0x74, 0x74, 0x6f,
    0x6e, 0x20, 0x31, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x13, 0x0d, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x13, 0x12, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x13, 0x22, 0x23, 0x0a, 0x19, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x16, 0x04, 0x24, 0x1a, 0x0a, 0x20, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x20, 0x32, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x11, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x16, 0x12, 0x1a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x16, 0x22, 0x23, 0x0a,
    0x19, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04, 0x12, 0x03, 0x19, 0x04, 0x24, 0x1a, 0x0a,
    0x20, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x20, 0x33, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x19, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x19, 0x0d, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x19, 0x12, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x19, 0x22, 0x23, 0x0a, 0x19, 0x0a, 0x06, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x04, 0x24, 0x1a, 0x0a, 0x20, 0x42, 0x75, 0x74, 0x74,
    0x6f, 0x6e, 0x20, 0x34, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x1c, 0x0d, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x1c, 0x12, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x1c, 0x22, 0x23, 0x0a, 0x4b, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06, 0x12,
    0x03, 0x1f, 0x04, 0x24, 0x1a, 0x3c, 0x20, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x61, 0x74, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x64, 0x6f, 0x77, 0x6e,
    0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6a, 0x6f, 0x79, 0x73, 0x74, 0x69, 0x63, 0x6b,
    0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1f,
    0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x1f,
    0x0d, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1f,
    0x12, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1f,
    0x22, 0x23, 0x0a, 0x3b, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07, 0x12, 0x03, 0x22, 0x04,
    0x24, 0x1a, 0x2c, 0x20, 0x54, 0x68, 0x65, 0x20, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x20, 0x6c,
    0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x62,
    0x75, 0x74, 0x74, 0x6f, 0x6e, 0x20, 0x31, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x32, 0x2e, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x22, 0x0d, 0x11, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x22, 0x12, 0x1f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x22, 0x22, 0x23, 0x0a,
    0x42, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x12, 0x03, 0x25, 0x04, 0x24, 0x1a, 0x33,
    0x20, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x28, 0x2d, 0x31, 0x2c, 0x20, 0x31, 0x29, 0x20, 0x77, 0x68,
    0x65, 0x72, 0x65, 0x20, 0x2d, 0x31, 0x20, 0x3d, 0x3d, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x2c, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x2b, 0x31, 0x20, 0x3d, 0x3d, 0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72,
    0x64, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03,
    0x25, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x25, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x25, 0x14, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x25, 0x22, 0x23, 0x0a, 0x40, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x12, 0x03, 0x28,
    0x04, 0x25, 0x1a, 0x31, 0x20, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x28, 0x2d, 0x31, 0x2c, 0x20, 0x31,
    0x29, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x2d, 0x31, 0x20, 0x3d, 0x3d, 0x20, 0x6c, 0x65,
    0x66, 0x74, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x2b, 0x31, 0x20, 0x3d, 0x3d, 0x20, 0x72, 0x69,
    0x67, 0x68, 0x74, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x04,
    0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x05,
    0x12, 0x03, 0x28, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x01,
    0x12, 0x03, 0x28, 0x14, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x28, 0x22, 0x24, 0x0a, 0x46, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x0a, 0x12,
    0x03, 0x2b, 0x04, 0x25, 0x1a, 0x37, 0x20, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x28, 0x30, 0x2c, 0x20,
    0x31, 0x29, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x30, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f,
    0x20, 0x70, 0x72, 0x65, 0x73, 0x73, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x31, 0x20, 0x69, 0x73,
    0x20, 0x66, 0x75, 0x6c, 0x6c, 0x20, 0x70, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x2b, 0x0d, 0x13, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x2b, 0x14, 0x1b, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x2b, 0x22, 0x24, 0x0a, 0x28, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x02, 0x1c, 0x1a, 0x1b, 0x20, 0x49, 0x6e, 0x66,
    0x6f, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20,
    0x70, 0x61, 0x64, 0x64, 0x6c, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x2f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x1a, 0x1b, 0x0a,
    0x27, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x32, 0x02, 0x1c, 0x1a, 0x1a, 0x20, 0x49,
    0x6e, 0x66, 0x6f, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x66, 0x74,
    0x20, 0x70, 0x61, 0x64, 0x64, 0x6c, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x32, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x32,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x32, 0x1a, 0x1b,
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
