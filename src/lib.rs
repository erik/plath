#![feature(asm, core, libc)]
#![allow(dead_code, raw_pointer_derive)]

// We only work on 64 bit linux
#![cfg(all(target_os = "linux", target_pointer_width = "64"))]

#[macro_use]
extern crate syscall;
extern crate libc;

mod ffi {
    pub mod clone;
    pub mod futex;

    pub use ffi::clone::clone;
}

mod mutex;
#[macro_use]
mod thread;
mod stack;

#[test]
fn it_works() {
    use libc;

    use thread::Thread;
    use stack::Stack;
    use mutex::Mutex;


    extern fn test(arg: *mut libc::c_void) -> libc::c_int {
        unsafe {
            //let thd: *const Thread = std::mem::transmute(arg);
            //println!("Arg to child: 0x{:x}, pid:{}", (*thd).magic, (*thd).pid);
            //println!("Set {:?}", *thd);

            let mutex: *mut Mutex = std::mem::transmute(arg);
            (*mutex).lock();

            println!("child holds the mutex");
            (*mutex).unlock();
        }

        println!("Hello! I am a thread");

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
    let mut mutex = Mutex::new();

    mutex.lock();
    println!("parent holds the mutex");

    let id = unsafe {
        ffi::clone(test,
                   stack.stack_top,
                   ffi::clone::flags::COMMON,
                   &mut mutex as *mut _ as *mut _,
                   &mut thd.pid,
                   std::ptr::null_mut(),
                   &mut thd.tid)
    };

    println!("child id = {}, {}, {}", id, thd.tid, thd.pid);
    println!("stack is valid? {}", stack.is_valid());

    println!("parent releasing mutex...");
    mutex.unlock();

    loop {}
}
