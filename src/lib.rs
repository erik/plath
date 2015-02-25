#[macro_use]
extern crate syscall;
extern crate libc;
extern crate alloc;

//mod ffi;
//mod thread;
mod futex;
//mod clone;

#[test]
fn it_works() {
    let mut f = 0i32;

    futex::wait(&mut f, 1);
}
