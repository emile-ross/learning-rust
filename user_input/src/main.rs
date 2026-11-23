use std::io;

fn main()
{
    let user_string: &'static str = "You should type in a string of reasonable size";
    println!("{}", user_string);
    let mut input = String::new();

    /* read_line(): read from shell
     * unwrap() uses the result from the read_line() function
     * expect() would be better practice */
    io::stdin().read_line(&mut input).unwrap();

    /* input.trim() could've been used in order to remove 
     * newline characters 
     * trim_end() removes newlines from the end of a string */
    let trimmed_string = input.trim_end();

    println!("{}", trimmed_string);     /* does not display a newline */

    println!("{}", input.trim());   /* also doesn't display any newline */

    println!("{}", input);


    match trimmed_string.parse::<i32>() {
        Ok(i) =>  println!("Here's your integer {}", i),
        Err(..) => println!("Not an integer \'{}\'", input.trim()),
    }

    /* we could've used unwrap() instead of expect() */
    let br: i32 = trimmed_string.parse::<i32>().expect("Invalid number\n");
    println!("Int: {}", br);
}
