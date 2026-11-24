mod operations;

fn main() {
    println!("return value of add: {} ", operations::add(1, 2));

    /* no i of type i32 declaration
     * it is not needed for an iterator */
    for i in 1..10
    {
        println!("{}", i % 2 == 0); /* print a boolean as string */
        println!("{}", i);
    }

    printt("string".to_string());
}

fn printt(string: String)
{
    println!("{}", string);
}
