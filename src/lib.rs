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

    extern fn test(arg: *mut libc::c_void) -> libc::c_int {
        println!("Hello! I am a thread");

        unsafe {
            let thd: *const Thread = std::mem::transmute(arg);
            println!("Arg to child: 0x{:x} ", (*thd).magic);
        }

        let offset = offset_of!(Thread, tid);
        println!("tid offset = {}", offset);

        unsafe {
            let thd = thread::get_current_thread();
            println!("Child TLS: {:?} ", thd);

            thread::set_tls_mem(offset, 123456);

            let thd = thread::get_current_thread();
            println!("i should have set tid = {:?}", thd.tid );
        }

        0
    }

    let stack = Stack::new();
    let mut thd = stack.install_thread_block();
    thd.magic = 0x55005500;

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
