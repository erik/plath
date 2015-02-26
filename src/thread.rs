use libc;

use std::ops::Drop;

const STACK_SPACE : u64 = 1024 * 1024;

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
