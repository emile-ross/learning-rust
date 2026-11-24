use std::ffi::{CStr, c_char};

fn main() {
    println!("Entering the unsafe part of the code");

    let a: i64 = 30;
    let b: i64 = 20;

    let sum: i64;
    let divided: f64;

    let c_string: *mut c_char;
    let rust_string: String;
    unsafe {
        hello_world();
        sum = add_operation(a, b);
        divided = divide(a, b);
        c_string = get_input();
        rust_string = CStr::from_ptr(c_string).to_string_lossy().into_owned();
    }

    println!("the sum is : \'{}\'", sum);
    println!("the answer : \'{:.6}\'", divided);

    println!("input: {}", rust_string);

    unsafe {
        free(c_string.cast());
    }
}

unsafe extern "C" {
    fn get_input() -> *mut c_char;
    fn hello_world();  /* call */
    fn add_operation(a: i64, b: i64) -> i64;
    fn divide(a: i64, b: i64) -> f64;
    fn free(ptr: *mut std::ffi::c_void);
}
