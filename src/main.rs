extern crate protobuf; // depend on rust-protobuf runtime
extern crate libc;

mod proto;

use libc::{c_int, c_char};
use std::ffi::CString;


#[link(name = "gz_wrapper")]
extern {
    pub fn run();
    pub fn pub_pos();
    pub fn set_pos(s: *const c_char, x: c_int, y: c_int, z: c_int);
}

fn main() {
    let model_name = CString::new("box").unwrap();
    unsafe {
        set_pos(model_name.as_ptr(), 4,0,0);
    }
}