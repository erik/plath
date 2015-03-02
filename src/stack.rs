use libc;
use std::ptr::null_mut;

/// 16k is enough for anybody.
pub const STACK_SIZE: u64 = 16384;

pub fn allocate_stack() -> *mut libc::c_void {
    let prot = libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE;
    let flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_STACK;

    let stack = unsafe {
        libc::mmap(null_mut(), STACK_SIZE, prot, flags, -1, 0)
    };

    if stack == libc::MAP_FAILED || stack.is_null() {
        panic!("couldn't mmap space for stack");
    }

    stack
}
