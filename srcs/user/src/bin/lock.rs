#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user_lib;

use user_lib::{fork, lock_acquire, lock_add, lock_get, lock_signal, lock_wait, wait};

#[no_mangle]
pub fn main() -> i32 {
    println!("lock test start:");
    let signal = lock_acquire();
    let mutex = lock_acquire();
    lock_signal(mutex);
    let pid = fork() as usize;
    if pid == 0 {
        // child process
        loop {
            lock_wait(mutex);
            let num = lock_get(signal);
            println!("child: {}", num);
            lock_add(signal, 1);
            lock_signal(mutex);
            if num > 2000 {
                break;
            }
        }

        // exit code
        100
    } else {
        // parent process
        loop {
            lock_wait(mutex);
            let num = lock_get(signal);
            println!("parent: {}", num);
            lock_add(signal, 1);
            lock_signal(mutex);
            if num > 2000 {
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
