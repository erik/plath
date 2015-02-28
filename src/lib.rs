#![feature(asm)]
#![feature(libc)]
#![allow(dead_code)]

#[macro_use]
extern crate syscall;
extern crate libc;

mod clone;
mod futex;
mod mutex;
#[macro_use]
mod thread;

#[test]
fn it_works() {
    use libc;
    use std::ptr;
    use std::mem;
    use std::old_io::timer;
    use std::time::duration::Duration;
    use thread::Thread;

    extern fn test(_: *mut libc::c_void) -> libc::c_int {
        println!("Hello! I am a thread");

        unsafe {
            let mut thd = get_thread_mem!(Thread, 0);
            println!("hello, TLS from child {:?} {:?}", thd, *thd );
        }

        0
    }

    let thd: &mut Thread = unsafe { std::mem::transmute(libc::calloc(1, mem::size_of::<Thread>() as u64)) };

    let id = unsafe {
        let stack = libc::malloc(1024 * 1024);
        clone::clone(test,
                     std::mem::transmute(stack.offset(1024 * 1024)),
                     clone::flags::COMMON,
                     ptr::null_mut(), // mem::transmute(&mut thd)
                     &mut thd.pid,
                     ptr::null_mut(), // &mut thd,
                     &mut thd.tid)
    };
    println!("child id = {}, {}, {}", id, thd.tid, thd.pid);
    println!("thd from parent{:?} ", thd );

    timer::sleep(Duration::seconds(15));

    println!("And I'm the parent!");
}
