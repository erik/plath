/// FIXME: TLS / Thread naming distinction needs to be resolved

use libc;

use std;
use std::simd;
use std::ptr::null_mut;

use ffi;

use stack::Stack;
use mutex::Mutex;

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

#[inline(always)]
pub unsafe fn get_tls_mem<T>(offset: usize) -> T {
    let dest: T;

    // We can't use constant segment offsets here due to some odd
    // asm! behavior, so just use indirect (it's slower, oh well).
    asm!("mov %fs:($1), $0"
         : "=r"(dest)
         : "r" (offset)
         :: "volatile");

    dest
}

#[inline(always)]
pub unsafe fn set_tls_mem<T>(offset: usize, expr: T) {
    asm!("movl $1, %fs:($0)" :
         : "r"(offset), "r"(expr)
         :: "volatile");
}

/// TCB (thread control block). We unfortunately need to match glibc here, so
/// there's a ton of unused vars.
#[repr(C, packed)]
#[derive(Debug)]
pub struct TcbHead {
    pub tcb: *mut libc::c_void,
    pub dtv: *mut libc::c_void,
    pub thread_self: *mut Thread,
    pub multiple_threads: i32,
    pub gscope_flag: i32,
    pub sysinfo: *mut u32,
    pub stack_guard: *mut u32,
    pub pointer_guard: *mut u32,
    pub vgetcpu_cache: [*mut u64; 2],

    __unused_1: [u32; 2],
    __unused_2: [*const libc::c_void; 5],
    __unused_3: u64,
    __unused_4: [simd::f32x4; 32],
    __padding: [*const libc::c_void; 8],
}


#[repr(C, packed)]
#[derive(Debug)]
pub struct Thread {
    pub header: TcbHead,

    // required by glibc
    _list: [*const libc::c_void; 2],
    /// This thread's id
    pub tid: libc::pid_t,
    /// This thread's parent pid
    pub pid: libc::pid_t,
    /// more glibc requirements
    /// TODO: find exact size
    _padding: [*const libc::c_void; 10],

    pub stack: *const Stack,
    pub magic: usize,
    pub mutex: Mutex
}

/// TODO: joinhandle-esque
pub fn spawn<F>(f: F) where F: Fn(), F: Sync + Send {
    let stack = Stack::new();
    let mut thd = stack.install_thread_block();

    // Make sure thread doesn't start until we're ready
    thd.mutex.lock();

    unsafe {
        ffi::clone(run_thread,
                   stack.stack_top,
                   ffi::clone::flags::COMMON,
                   &f as *const _ as *mut libc::c_void,
                   &mut thd.pid,
                   null_mut(),
                   &mut thd.tid)
    };

    thd.mutex.unlock();
}


extern fn run_thread(arg: *mut libc::c_void) -> libc::c_int {
    let mut thd = get_current_thread();

    println!("thread started");

    // Synchronize with caller
    thd.mutex.lock();

    // Now we can run
    unsafe {
        println!("okay, time to run thd function");
        let thd_fn: *const &Fn() = std::mem::transmute(arg);
        (*thd_fn)();
    }

    thd.mutex.unlock();

    0
}


pub fn get_current_thread() -> &'static mut Thread {
    let thread_offset = offset_of!(TcbHead, thread_self);

    let thd_ptr: *mut Thread = unsafe { get_tls_mem(thread_offset) };
    if thd_ptr.is_null() {
        panic!("TLS thread return NULL");
    }

    unsafe { &mut *thd_ptr }
}


/// Have the current thread voluntarily yield the rest of it's scheduled run
/// time.
pub fn yield_now() {
    extern { fn sched_yield() -> libc::c_int; }

    unsafe { sched_yield(); }
}


#[test]
fn test_sanity() {
    spawn(move || {
        println!("hi, i'm a thread");
        loop{}
    });

    println!("hello i am a parrent");
    loop {}
}
