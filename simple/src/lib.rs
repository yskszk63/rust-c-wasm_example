use std::os::raw::c_int;

extern "C" {
    fn add(left: c_int, right: c_int) -> c_int;
}

#[no_mangle]
pub extern fn plus1(n: c_int) -> c_int {
    unsafe { add(n, 1) }
}
