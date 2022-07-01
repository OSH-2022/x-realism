// https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust/
// https://github.com/cfsamson/example-greenthreads
#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user_lib;

use user_lib::{exit, yield_task, Runtime};

#[no_mangle]
pub fn main() {
    println!("RUN TIME STARTING");
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        println!("THREAD 1 STARTING");
        let id = 1;
        for i in 0..4 {
            println!("THREAD: {} count: {}", id, i);
            yield_task();
        }
        println!("THREAD 1 FINISHED");
    });
    runtime.spawn(|| {
        println!("THREAD 2 STARTING");
        let id = 2;
        for i in 0..8 {
            println!("THREAD: {} count: {}", id, i);
            yield_task();
        }
        println!("THREAD 2 FINISHED");
    });
    runtime.spawn(|| {
        println!("THREAD 3 STARTING");
        let id = 3;
        for i in 0..12 {
            println!("THREAD: {} count: {}", id, i);
            yield_task();
        }
        println!("THREAD 3 FINISHED");
    });
    runtime.spawn(|| {
        println!("THREAD 4 STARTING");
        let id = 4;
        for i in 0..16 {
            println!("THREAD: {} count: {}", id, i);
            yield_task();
        }
        println!("THREAD 4 FINISHED");
    });
    runtime.run();
    exit(0);
}
