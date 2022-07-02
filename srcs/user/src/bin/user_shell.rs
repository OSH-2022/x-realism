#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use user_lib::console::getchar;
use user_lib::{close, exec, fork, open, read, waitpid, write, OpenFlags};

/// get tokens for a command
fn get_tokens(command: String) -> Vec<String> {
    command
        .split_whitespace()
        .map(|token| token.to_string())
        .collect()
}

/// touch and input
fn touch(path: &str) -> i32 {
    let fd = open(path, OpenFlags::CREATE | OpenFlags::WRONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    let mut line: String = String::new();
    let mut buf: String = String::new();
    let mut flag = false;
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                if flag == false {
                    flag = true;
                    println!("");
                    line.push(10 as char);
                    buf.push_str(line.as_str());
                } else {
                    write(fd, buf.as_bytes());
                    return 0;
                }
                line.clear();
            }
            BS | DL => {
                flag = false;
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            _ => {
                flag = false;
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}

/// cat file
fn cat(path: &str) -> i32 {
    let fd = open(path, OpenFlags::RDONLY);
    if fd == -1 {
        panic!("Error occured when opening file");
    }
    let fd = fd as usize;
    let mut buf = [0u8; 256];
    loop {
        let size = read(fd, &mut buf) as usize;
        if size == 0 {
            break;
        }
        print!("{}", core::str::from_utf8(&buf[..size]).unwrap());
    }
    close(fd);
    0
}

#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    let mut line: String = String::new();
    print!(">> ");
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    line.push('\0');
                    let pid = fork();
                    if pid == 0 {
                        // child process
                        let tokens = get_tokens(line.clone());
                        if tokens[0] == "touch" {
                            return touch(tokens[1].as_str());
                        } else if tokens[0] == "cat" {
                            return cat(tokens[1].as_str());
                        }
                        if exec(line.as_str()) == -1 {
                            println!("Error when executing!");
                            return -4;
                        }
                        unreachable!();
                    } else {
                        let mut exit_code: i32 = 0;
                        let exit_pid = waitpid(pid as usize, &mut exit_code);
                        assert_eq!(pid, exit_pid);
                        println!("Shell: Process {} exited with code {}", pid, exit_code);
                    }
                    line.clear();
                }
                print!(">> ");
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
