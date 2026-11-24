fn main() {
    println!("Entering the unsafe part of the code");

    let a: i64 = 10;
    let b: i64 = 20;

    let sum: i64;
    unsafe {
        hello_world();
        sum = add_operation(a, b);
    }

    println!("the sum is : \'{}\'", sum);
}

unsafe extern "C" {
    fn hello_world();  /* call */
    fn add_operation(a: i64, b: i64) -> i64;
}
