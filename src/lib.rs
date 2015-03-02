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

    extern fn test(_: *mut libc::c_void) -> libc::c_int {
        println!("Hello! I am a thread");

        let x = offset_of!(Thread, tid);
        println!("tid offset = {}", x);

        unsafe {
            let thd = thread::get_current_thread();
            println!("hello, TLS from child {:?} ", thd);

            thread::set_tls_mem(x, 123456);

            let thd = thread::get_current_thread();
            println!("i should have set tid = {:x}", thd.tid );
        }

        0
    }

    let thd: &mut Thread = unsafe { std::mem::transmute(libc::calloc(1, mem::size_of::<Thread>() as u64)) };

    let id = unsafe {
        let stack = stack::allocate_stack();
        clone::clone(test,
                     std::mem::transmute(stack.offset(16384)),
                     clone::flags::COMMON,
                     ptr::null_mut(), // mem::transmute(&mut thd)
                     &mut thd.pid,
                     ptr::null_mut(), // &mut thd,
                     &mut thd.tid)
    };
    println!("child id = {}, {}, {}", id, thd.tid, thd.pid);

    loop {}
}
