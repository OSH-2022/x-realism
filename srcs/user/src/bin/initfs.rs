#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use easy_fs::{BlockDevice, EasyFileSystem, Inode};

#[no_mangle]
fn main() -> i32 {
    // let block_device = Arc::new(BlockDeviceImpl::new());
    // let efs = EasyFileSystem::open(block_device);
    0
}
