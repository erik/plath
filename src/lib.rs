#![feature(asm)]
#![feature(libc)]
#![allow(dead_code)]

#[macro_use]
extern crate syscall;
extern crate libc;

mod clone;
mod futex;
mod mutex;
mod thread;

#[test]
fn it_works() {
    use libc;
    use std::ptr;
    use std::mem;
    use std::old_io::timer;
    use std::time::duration::Duration;

    extern fn test(_: *mut libc::c_void) -> libc::c_int {
        println!("Hello! I am a thread");
        0
    }

    let id = unsafe {
        let stack = libc::malloc(1024 * 1024);

        clone::clone(test,
                     std::mem::transmute(stack.offset(1024 * 1024)),
                     clone::flags::COMMON,
                     ptr::null_mut())
    };
    println!("child id = {}", id);

    timer::sleep(Duration::seconds(2));

    println!("And I'm the parent!");
}
