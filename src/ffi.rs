use libc;


pub fn alloc(sz: usize) -> *mut libc::c_void {
    unsafe {
        let ptr = libc::malloc(sz as libc::size_t);
        if ptr.is_null() {
            panic!("malloc failed!");
        }

        ptr
    }
}
