use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

use futex;
use thread;

pub struct Mutex {
    inner: UnsafeCell<MutexInner>
}

struct MutexInner {
    lock: AtomicUsize,
    guard: i32
}

impl Mutex {
    pub fn new() -> Mutex {
        Mutex { inner: UnsafeCell::new(MutexInner::new()) }
    }

    pub fn lock(&mut self) {
        unsafe {
            let inner = &mut *self.inner.get();
            inner.lock()
        }
    }

    pub fn unlock(&mut self) {
        unsafe {
            let inner = &mut *self.inner.get();
            inner.unlock()
        }
    }
}


const LOCK_FREE: usize = 0xa5a5a5a5;

/// TODO: handle all let _ = ... error cases.
impl MutexInner {
    pub fn new() -> MutexInner {
        MutexInner {
            lock: AtomicUsize::new(LOCK_FREE),
            guard: 0
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
        let _ = futex::wake(&mut self.guard, 1);
    }

    pub fn lock(&mut self) {
        loop {
            let tid = thread::get_current_thread().tid;
            let swap = self.lock.compare_and_swap(LOCK_FREE, tid as usize, Ordering::Relaxed);

            // If compare_and_swap didn't return LOCK_FREE, we weren't the ones
            // to set it, so go to sleep and spin.
            if swap != LOCK_FREE {
                // Go to sleep, wait for someone to unlock
                let _ = futex::wait(&mut self.guard, 0);
            }
            // Otherwise we've got the mutex, time to return.
            else { break; }
        }
    }
}
