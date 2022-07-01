#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user_lib;

use alloc::string::ToString;
use user_lib::{fork, get_pid, getpid, recv, send, sleep, wait};

#[no_mangle]
pub fn main() -> i32 {
    println!("send test start:");
    let pid = fork() as usize;
    let size = 17;
    if pid == 0 {
        // child process
        println!("hello child process!");
        let mut buffer = [0_u8; 17];
        let ptr = buffer.as_mut_ptr() as usize;
        let pid = getpid() as usize;

        // sleep
        sleep(10);

        // recv
        recv(pid, ptr, size);
        println!("child recv: {}", buffer.escape_ascii().to_string());
        println!("child exit");
        100
    } else {
        // parent process
        let message = "Hello, x-realism!";
        let curr_pid = getpid() as usize;
        let to_pid = get_pid(curr_pid, pid);
        let ptr = message.as_ptr() as usize;
        println!("parent send: {}", message);

        // send
        send(to_pid, ptr, size);

        // wait
        let mut exit_code: i32 = 0;
        println!("ready waiting on parent process!");
        assert_eq!(pid, wait(&mut exit_code) as usize);
        assert_eq!(exit_code, 100);
        println!(
            "parent wait: child process pid = {}, exit code = {}",
            pid, exit_code
        );
        0
    }
}
