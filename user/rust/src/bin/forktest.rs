#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exit, fork, wait};

const MAX_CHILD: usize = 30;

#[no_mangle]
pub fn main() -> i32 {
    for i in 0..MAX_CHILD {
        let pid = fork();
        if pid == 0 {
            println!("I am child {}", i);
            exit(0);
        } else {
            println!("forked child pid = {}", pid);
        }
        assert!(pid > 0);
    }
    for _ in 0..MAX_CHILD {
        if wait(None) <= 0 {
            panic!("wait stopped early");
        }
    }
    if wait(None) > 0 {
        panic!("wait got too many");
    }
    println!("forktest passed!");
    0
}