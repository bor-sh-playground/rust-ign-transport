extern crate protobuf; // depend on rust-protobuf runtime
extern crate libc;

mod proto;

use libc::{c_int, c_char};
use std::ffi::CString;

extern "C" fn contact_callback( x: c_int, y: c_int, z: c_int ) {
    println!("Called with value {0} {1} {2}", x, y, z);
}

#[link(name = "gz_wrapper")]
extern {
    pub fn run();
    pub fn pub_pos();
    pub fn set_pos(s: *const c_char, x: c_int, y: c_int, z: c_int);
    pub fn subscribe(cb: extern fn(c_int, c_int, c_int), s: *const c_char);
}

fn main() {
    let model_name = CString::new("box").unwrap();
    let topic_name = CString::new("/gazebo/default/box/link/my_contact").unwrap();
    unsafe {
        subscribe(contact_callback, topic_name.as_ptr());
        set_pos(model_name.as_ptr(), 4,0,0);
    }
}