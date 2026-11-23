use std::io;

/*
use std::io::prelude::*;
*/

fn main()
{
    let user_string: &'static str = "You should type in a string of reasonable size";
    println!("{}", user_string);
    let mut input = String::new();

    /* read_line(): read from shell
     * unwrap() uses the result from the read_line() function
     * expect() would be better practice */
    io::stdin().read_line(&mut input).unwrap();

    println!("{}", input);
}
