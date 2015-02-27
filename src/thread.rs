use libc;

use std::ops::Drop;

const STACK_SPACE : u64 = 1024 * 1024;

pub unsafe fn get_self() -> () {
    let out: *mut u8 = get_thread_mem(0);

    println!("out = {}", *out);
}

pub unsafe fn get_thread_mem<T>(offset: u8) -> *mut T {
    let result: *mut T;

    asm!("mov $0, fs:$1"
         : /* out */ "=r"(result)
         : /* in */  "r" (offset)
         : /* clobber */
         : /* opts */    "intel"
         );

    result
}

pub struct Thread {
    stack: *mut libc::c_void,
}


impl Drop for Thread {
    fn drop(&mut self) {
        unsafe { libc::free(self.stack); }
    }
}


impl Thread {
    pub fn new() -> Thread {
        let stack_space = unsafe {
            let m = libc::malloc(STACK_SPACE);

            if m.is_null() { panic!("couldn't alloc stack space"); }

            m
        };

        Thread { stack: stack_space }
    }
}
