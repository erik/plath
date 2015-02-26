use libc;
use libc::c_int;

use std::ptr;
use std::result::Result;

/// Possible error conditions that futex(2) can return
pub enum Errors {
    /// No read access to futex memory
    FutexRead,
    /// Futex value does not equal given expected value (e.g. in race condition)
    FutexValue,
    /// Wait was interrupted by a signal
    Signal,
    /// Oops.
    InvalidArg,
    /// Too many open files
    TooManyFiles,
    /// Oops.
    InvalidOp,
    /// Futex hit wait timeout
    WaitTimeout
}


/// Wraps syscall_futex:
///
/// int futex(int *uaddr, int op, int val, const struct timespec *timeout,
///           int *uaddr2, int val3);
unsafe fn futex(addr1: *mut c_int, op: c_int, val: c_int,
                timespec: *const libc::timespec,
                addr2: *mut c_int, val3: c_int) -> c_int {

    // XXX: syscall returns usize, we expect i32
    syscall!(FUTEX, addr1, op, val, timespec, addr2, val3) as c_int
}


#[inline(always)]
fn get_result(retval: i32) -> Result<i32, Errors> {
    use libc::{EACCES, EAGAIN, EINTR, EINVAL, ENOSYS, ETIMEDOUT};

    if retval >= 0 {
        return Ok(retval);
    }

    match retval {
        EACCES => Err(Errors::FutexRead),
        EINTR  => Err(Errors::Signal),
        EINVAL => Err(Errors::InvalidArg),
        ENOSYS => Err(Errors::InvalidOp),
        EAGAIN => Err(Errors::FutexValue),
        ETIMEDOUT => Err(Errors::WaitTimeout),
        _ => panic!("unexpected futex return value: {}", retval)
    }
}


/// Futex operations
mod op {
    pub const WAIT: i32 = 0;
    pub const WAKE: i32 = 1;
    pub const CMP_REQUEUE: i32 = 4;
    // deprecated: FD(2), REQUEUE(3)
}


/// Verifies that the futex in `addr` still contains the value `val`, and then
/// sleeps the thread awaiting a FUTEX_WAKE.
pub fn wait(addr: &mut i32, val: i32) -> Result<i32, Errors> {
    let ret = unsafe {
        futex(addr as (*mut i32),
              op::WAIT,
              val,
              ptr::null::<libc::timespec>(),
              ptr::null_mut(),
              0)
    };

    get_result(ret)
}

/// Same as `wait`, except only sleeps for the given number of seconds.
/// If the wait times out, Err(Errors::WaitTimeout) will be returned.
pub fn time_wait(addr: &mut i32, val: i32, wait_secs: u32) -> Result<i32, Errors> {
    let ts = libc::timespec { tv_sec: wait_secs as i64, tv_nsec: 0 };

    let ret = unsafe {
        futex(addr as (*mut i32),
              op::WAIT,
              val,
              &ts,
              ptr::null_mut(),
              0)
    };

    get_result(ret)
}


/// This operation wakes at most `nprocs` processes waiting on this
/// futex address (i.e., inside FUTEX_WAIT).
///
/// Results the number of processes woken up or error
pub fn wake(addr: &mut i32, nprocs: u32) -> Result<i32, Errors> {
    let ret = unsafe {
        futex(addr as (*mut i32),
              op::WAKE,
              nprocs as i32,
              ptr::null::<libc::timespec>(),
              ptr::null_mut(),
              0)
    };

    get_result(ret)
}


/// This operation was introduced in order to avoid a "thundering herd" effect
/// when `wake` is used and all processes woken up need to acquire another
/// futex. This call wakes up `nprocs` processes, and requeues all other
/// waiters on the futex at address `requeue_addr`.
///
/// TODO: explain val.
pub fn requeue(addr: &mut i32, requeue_addr: &mut i32, val: i32, nprocs: u32) -> Result<i32, Errors> {
    let ret = unsafe {
        futex(addr as *mut i32,
              op::CMP_REQUEUE,
              nprocs as i32,
              ptr::null::<libc::timespec>(),
              requeue_addr as *mut i32,
              val)
    };

    get_result(ret)
}
