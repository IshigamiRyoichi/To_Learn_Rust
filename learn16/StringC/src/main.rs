use std::os::raw::c_char;
use std::ffi::CString;

extern {
    fn puts(s: *const c_char);
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    let s = "hello rust world.";
    let s_null_terminated = CString::new(s).unwrap();
    unsafe {
        puts(s_null_terminated.as_ptr());
    }
    let n = unsafe {
        strlen(s_null_terminated.as_ptr())
    };
    println!("s.len is {}",n);
}