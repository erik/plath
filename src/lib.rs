#[macro_use]
extern crate syscall;
extern crate libc;
extern crate alloc;

mod thread;
mod futex;
mod clone;

#[test]
fn it_works() {
}
