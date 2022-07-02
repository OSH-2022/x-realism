#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{open, OpenFlags};

#[no_mangle]
pub fn main() -> i32 {
    let fd = open("root\0", OpenFlags::RDONLY);
    0
}
