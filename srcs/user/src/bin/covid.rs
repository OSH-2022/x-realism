#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user_lib;

use user_lib::{fork, lock_acquire, lock_add, lock_get, wait, Mutex};

#[no_mangle]
pub fn main() -> i32 {
    println!("covid test start:");
    let mut mutex = Mutex::new();
    let waiting = lock_acquire();
    let total = lock_acquire();
    let pid = fork() as usize;
    if pid == 0 {
        // child process
        loop {
            mutex.lock();
            let waiting_cnt = lock_get(waiting);
            let total_cnt = lock_get(total);
            if total_cnt >= 1145 && waiting_cnt == 0 {
                mutex.unlock();
                break;
            }
            if waiting_cnt > 0 {
                let new_waiting = lock_add(waiting, -1);
                println!("child: served one. now {} waiting!", new_waiting);
            }
            mutex.unlock();
        }

        // exit code
        100
    } else {
        // parent process
        loop {
            mutex.lock();
            let waiting_cnt = lock_add(waiting, 1);
            let total_cnt = lock_add(total, 1);
            println!("parent: {} in total, {} waiting!", total_cnt, waiting_cnt);
            if total_cnt >= 1145 {
                mutex.unlock();
                break;
            }
            mutex.unlock();
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
