use libc;
use libc::{c_int, c_void, timespec};

use std::mem::transmute;
use std::ptr::null;
use std::result::Result;

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

/// Wraps futex(2):
///
/// int futex(int *uaddr, int op, int val, const struct timespec *timeout,
///           int *uaddr2, int val3);
//#[link(name="libc")]
//extern {
unsafe fn futex(addr1:    *mut   c_int,
         op:              c_int,
         val:             c_int,
         timespec: *const timespec,
         addr2:    *mut   c_int,
         val3:            c_int)
         -> c_int {
syscall!(FUTEX, addr1, op, val, timespec, addr2, val3) as c_int
         }
//}

const OP_WAIT        : i32 = 0;
const OP_WAKE        : i32 = 1;
// deprecated: const OP_FD          : i32 = 2;
// deprecated: const OP_REQUEUE     : i32 = 3;
const OP_CMP_REQUEUE : i32 = 4;


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

/// Verifies that the futex in `addr` still contains the value `val`, and then
/// sleeps the thread awaiting a FUTEX_WAKE.
pub fn wait(addr: &mut i32, val: i32) -> Result<i32, Errors> {
    unsafe {
        get_result(
            futex(addr as (*mut i32),
                  OP_WAIT,
                  val,
                  null::<timespec>(),
                  transmute(null::<i32>()),
                  0))
    }
}

/// Same as `wait`, except only sleeps for the given number of seconds.
/// If the wait times out, Err(Errors::WaitTimeout) will be returned.
pub fn time_wait(addr: &mut i32, val: i32, wait_secs: u32) -> Result<i32, Errors> {
    let ts = timespec { tv_sec: wait_secs as i64, tv_nsec: 0 };

    let ret = unsafe {
        futex(addr as (*mut i32),
              OP_WAIT,
              val,
              &ts,
              transmute(null::<i32>()),
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
              OP_WAKE,
              nprocs as i32,
              null::<timespec>(),
              transmute(null::<i32>()),
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
        futex(addr as (*mut i32),
              OP_CMP_REQUEUE,
              nprocs as i32,
              null::<timespec>(),
              transmute(requeue_addr),
              val)
    };

    get_result(ret)
}
