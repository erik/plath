use std::cell::UnsafeCell;
use std::fmt::{Debug, Error, Formatter};
use std::sync::atomic::{AtomicUsize, Ordering};

use ffi::futex;
use thread;


pub struct Mutex {
    inner: UnsafeCell<MutexInner>
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

struct MutexInner {
    lock: AtomicUsize,
    futex: i32
}

// TODO: a scoped / RAII lock would be useful.
impl Mutex {
    pub fn new() -> Mutex {
        Mutex { inner: UnsafeCell::new(MutexInner::new()) }
    }

    fn get_inner(&self) -> &mut MutexInner {
        unsafe { &mut *self.inner.get() }
    }

    pub fn try_lock(&mut self) -> bool {
        self.get_inner().try_lock()
    }

    pub fn lock(&mut self) {
        self.get_inner().lock()
    }

    pub fn unlock(&mut self) {
        self.get_inner().unlock()
    }
}


impl Debug for Mutex {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let tid = unsafe {
            let inner = &mut *self.inner.get();
            inner.lock.load(Ordering::Relaxed)
        };

        if tid == LOCK_FREE {
            fmt.write_str("Mutex { unlocked }")
        } else {
            fmt.write_fmt(format_args!("Mutex {{ owned by {} }}", tid))
        }
    }
}


const LOCK_FREE: usize = 0xa5a5a5a5;

/// TODO: handle all let _ = ... error cases.
impl MutexInner {
    pub fn new() -> MutexInner {
        MutexInner {
            lock: AtomicUsize::new(LOCK_FREE),
            futex: 0
        }
    }

    pub fn unlock(&mut self) {
        let old_val = self.lock.load(Ordering::Relaxed);

        if old_val != thread::get_current_thread().tid as usize {
            panic!("trying to unlock a mutex we don't own");
        }

        // mark the lock as free
        self.lock.store(LOCK_FREE, Ordering::Relaxed);

        // wake up 1 thread waiting to lock
        let _ = futex::wake(&mut self.futex, 1);
    }

    pub fn try_lock(&mut self) -> bool {
        let tid = thread::get_current_thread().tid;
        let swap = self.lock.compare_and_swap(LOCK_FREE, tid as usize, Ordering::Relaxed);

        // If compare_and_swap returned LOCK_FREE, we were the first ones
        // to grab the lock, so we currently own the mutex.
        swap == LOCK_FREE
    }

    pub fn lock(&mut self) {
        while !self.try_lock() {
            // We didn't get the lock this time, wait for owner to unlock
            let _ = futex::wait(&mut self.futex, 0);
        }
    }
}
