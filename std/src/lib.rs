pub fn coitusinterruptus(s: *const i8) {
    unsafe {
        libc::puts(s);
    }
}
