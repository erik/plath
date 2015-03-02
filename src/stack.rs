use libc;
use std::ptr::null_mut;

use thread;

/// 16k is enough for anybody.
pub const SIZE: isize = 16384;

/// Set at bottom of stack to detect overruns.
pub const CANARY: usize = 0xABCDABCD;

pub struct Stack {
    /// Top of the stack, what clone expects
    pub stack_top: *mut libc::c_void,
    pub stack_bot: *mut libc::c_void,
    pub size: isize
}

// TODO: add impl Drop to munmap

impl Stack {
    pub fn new() -> Stack {
        let base_ptr = Stack::allocate_mem(SIZE);

        unsafe {
            *(base_ptr as *mut usize) = CANARY;

            Stack {
                stack_bot: base_ptr,
                stack_top: base_ptr.offset(SIZE - 256),
                size: SIZE
            }
        }
    }

    fn allocate_mem(size: isize) -> *mut libc::c_void {
        let flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_STACK;
        let prot = libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE;

        let stack = unsafe {
            libc::mmap(null_mut(), size as u64, prot, flags, -1, 0)
        };

        if stack == libc::MAP_FAILED || stack.is_null() {
            panic!("couldn't mmap space for stack");
        }

        stack
    }

    pub fn is_valid(&self) -> bool {
        unsafe { *(self.stack_bot as *mut usize) == CANARY }
    }

    pub fn install_thread_block<'a>(&'a self) -> &'a mut thread::Thread {
        let thd = unsafe {
            let top = self.stack_bot.offset(self.size) as *mut thread::Thread;
            let thd_ptr = top.offset(-1);

            &mut *thd_ptr
        };

        thd.magic = 100;
        thd.stack = self as *const Stack;

        thd
    }
}
