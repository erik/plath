#![feature(asm, core, libc)]
#![allow(dead_code, raw_pointer_derive)]

// We only work on 64 bit linux
#![cfg(all(target_os = "linux", target_pointer_width = "64"))]

#[macro_use]
extern crate syscall;
extern crate libc;

mod ffi {
    pub mod clone;
    pub mod futex;

    pub use ffi::clone::clone;
}

mod mutex;
#[macro_use]
mod thread;
mod stack;
