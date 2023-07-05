use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn coitusinterruptus(x: *const i8) {
    let c_str = unsafe { CStr::from_ptr(x) };
    println!("{}", c_str.to_str().unwrap());
}
