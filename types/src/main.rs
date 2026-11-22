fn main()
{
    let mut x: i32 = 2147483647;
    let y: u64 = 1209123123;

    x += 1;

    println!("Signed int: {}", x);
    println!("unsigned size_t {}", y);
}
