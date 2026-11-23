fn main()
{
    /* simple static integers */
    let mut x: i32 = 2147483646;
    let y: u64 = 1209123123;

    x += 1;

    println!("Signed int: {}", x);
    println!("unsigned size_t {}", y);


    let arr: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];

    println!("signed array: {:?}", arr);

    /* string array */

}
