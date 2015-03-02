#![feature(asm, libc)]
#![allow(dead_code, raw_pointer_derive)]

#[macro_use]
extern crate syscall;
extern crate libc;

mod clone;
mod futex;
mod mutex;
#[macro_use]
mod thread;
mod stack;

#[test]
fn it_works() {
    use libc;
    use std::ptr;
    use std::mem;
    use thread::Thread;
    use stack::Stack;

    extern fn test(a: *mut libc::c_void) -> libc::c_int {
        println!("Hello! I am a thread");

        let thd: *const Thread = unsafe { std::mem::transmute(a) };
        unsafe {println!("here's my arg {:?} ", *thd);}

        let x = offset_of!(Thread, tid);
        println!("tid offset = {}", x);

        unsafe {
            let thd = thread::get_current_thread();
            println!("hello, TLS from child {:?} ", thd);

            thread::set_tls_mem(x, 123456);

            let thd = thread::get_current_thread();
            println!("i should have set tid = {:?}", thd.tid );
        }

        0
    }

    let mut stack = Stack::new();
    let mut thd = stack.install_thread_block();

    let id = unsafe {
        clone::clone(test,
                     stack.stack_top,
                     clone::flags::COMMON,
                     thd as *mut _ as *mut _,
                     &mut thd.pid,
                     std::ptr::null_mut(),
                     &mut thd.tid)
    };

    println!("child id = {}, {}, {}", id, thd.tid, thd.pid);
    println!("stack is valid? {}", stack.is_valid());

    loop {}
}
