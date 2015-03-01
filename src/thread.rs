use libc;

use std::ptr::null_mut;

/// 16k is enough for anybody.
const STACK_SIZE: u64 = 16384;

/// Get the offset in bytes of some particular struct member.
///
/// Idea is simply to see what the address of that member would be if the
/// struct was created at address 0x0 (which gives its offset in bytes).
macro_rules! offset_of {
    ($kind:ty, $member:ident) => {
        unsafe {
            let ptr = 0x0usize as *const $kind;
            let member_addr = (&(*ptr).$member as *const _) as usize;

            member_addr
        }
    };
}

/// Return a pointer to the TLS value at the given offset.
macro_rules! get_thread_mem {
    ($offset:expr, $kind:ty) => {{
        let dest_ptr: *mut $kind;

        // We can't use constant segment offsets here due to some odd
        // asm! behavior, so just use indirect (it's slower, oh well).
        asm!("mov %fs:($1), $0"
             : "=r"(dest_ptr)
             : "r" ($offset)
             :: "volatile");

        dest_ptr
    }};
}

macro_rules! set_thread_mem {
    ($offset:expr, $expression:expr) => {
        asm!("movl $1, %fs:($0)" :
             : "r"($offset), "i"($expression)
             :: "volatile");
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
    let thd_ptr: *mut Thread = get_thread_mem!(0, Thread);
    if thd_ptr.is_null() { panic!("TLS thread return NULL"); }

    let thd = &*thd_ptr;

    println!("magic = {}", thd.magic);
}

pub unsafe fn allocate_stack() -> *mut libc::c_void {
    let prot = libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE;
    let flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_STACK;

    let stack = libc::mmap(null_mut(), STACK_SIZE, prot, flags, -1, 0);

    if stack == libc::MAP_FAILED || stack.is_null() {
        panic!("couldn't mmap space for stack");
    }

    stack
}
