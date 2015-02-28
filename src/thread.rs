use libc;

use std::ops::Drop;

const STACK_SPACE : u64 = 1024 * 1024;

macro_rules! get_thread_mem {
    ( $($kind:ty, $offset:expr),* ) => {{
        $(
            let raw_ptr: *mut $kind;

            asm!("mov $0, fs:$1"
                 : /* out */ "=r"(raw_ptr)
                 : /* in */  "i" ($offset)
                 : /* clobber */
                 : "intel");

            raw_ptr
          )*
        }
    };
}


#[repr(C, packed)]
#[derive(Debug)]
pub struct ControlBlockHead {
    tcb: *mut libc::c_void,
    dtv: *mut libc::c_void,
    thread_self: *mut Thread,

    /// We don't actually care about the rest.
    padding: [*mut libc::c_void; 21],
}


#[repr(C, packed)]
#[derive(Debug)]
pub struct Thread {
    header: ControlBlockHead,

    /// This thread's id
    pub tid: libc::pid_t,

    /// This thread's parent pid
    pub pid: libc::pid_t,

    pub magic: usize
}

unsafe fn get_thread_tcb() {
}

pub unsafe fn get_current_thread() -> () {
    let thd_ptr: *mut Thread = get_thread_mem!(Thread, 0);
    if thd_ptr.is_null() { panic!("TLS thread return NULL"); }

    let thd = &*thd_ptr;

    println!("magic = {}", thd.magic);
}

pub struct ThreadBlarg {
    stack: *mut libc::c_void,
}


impl Drop for ThreadBlarg {
    fn drop(&mut self) {
        unsafe { libc::free(self.stack); }
    }
}


impl ThreadBlarg {
    pub fn new() -> ThreadBlarg {
        let stack_space = unsafe {
            let m = libc::malloc(STACK_SPACE);

            if m.is_null() { panic!("couldn't alloc stack space"); }

            m
        };

        ThreadBlarg { stack: stack_space }
    }
}
