fn main() {
    println!("Entering the unsafe part of the code");

    let a: i64 = 30;
    let b: i64 = 20;

    let sum: i64;
    let divided: f64;
    unsafe {
        hello_world();
        sum = add_operation(a, b);
        divided = divide(a, b);
    }

    println!("the sum is : \'{}\'", sum);
    println!("the answer : \'{:.6}\'", divided);
}

unsafe extern "C" {
    fn hello_world();  /* call */
    fn add_operation(a: i64, b: i64) -> i64;
    fn divide(a: i64, b: i64) -> f64;
}
