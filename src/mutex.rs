use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

use futex;

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
            let ref mut inner = *self.inner.get();
            inner.lock()
        }
    }

    pub fn unlock(&mut self) {
        unsafe {
            let ref mut inner = *self.inner.get();
            inner.unlock()
        }
    }
}


impl MutexInner {
    pub fn new() -> MutexInner {
        MutexInner {
            lock: AtomicUsize::new(0),
            guard: 0
        }
    }

    pub fn unlock(&mut self) {
        let old_val = self.lock.load(Ordering::Relaxed);

        if old_val != /* TODO: get thread id */ 0 {
            panic!("trying to unlock a mutex we don't own");
        }

        // TODO: handle this
        let _ = futex::wake(&mut self.guard, 1);
    }

    pub fn lock(&mut self) {
        let old_val = self.lock.load(Ordering::Relaxed);

        loop {
            // TODO: get thread id.
            let new_val = 0;

            let swap = self.lock.compare_and_swap(old_val, new_val, Ordering::Relaxed);

            // If CAS didn't return old_val, it didn't succeed, so futex and spin
            if swap != old_val {
                // TODO: handle possible error
                let _ = futex::wait(&mut self.guard, 0);
            }

            // Otherwise we got the mutex
            else { break; }
        }
    }
}
