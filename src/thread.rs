use libc;

use std::mem;
use std::ops::Drop;

const STACK_SPACE : usize = 1024 * 1024;

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
        let stack_space = libc::malloc(STACK_SPACE);
        Thread { stack: stack_space }
    }
}
