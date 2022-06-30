#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user_lib;

use user_lib::{fork, lock_acquire, lock_add, lock_get, lock_set, lock_wait, sleep, wait, yield_};

#[no_mangle]
pub fn main() -> i32 {
    println!("lock test start:");
    let signal = lock_acquire();
    let pid = fork() as usize;
    if pid == 0 {
        // child process
        loop {
            let num = lock_get(signal);
            println!("child: {}", num);
            lock_add(signal, 1);
            if num > 1145 {
                break;
            }
        }

        // exit code
        100
    } else {
        // parent process
        loop {
            let num = lock_get(signal);
            println!("parent: {}", num);
            lock_add(signal, 1);
            if num > 1145 {
                break;
            }
        }

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
