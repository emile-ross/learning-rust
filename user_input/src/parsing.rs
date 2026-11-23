pub mod parsing {
    pub fn get_integer(string: String)
    {
        let value: i32 = string.parse::<i32>().expect("Invalid number\n");

        println!("{}", string);
        println!("{}", value);
    }

    /*
    match trimmed_string.parse::<i32>() {
        Ok(i) => println!("string : {}\n", string);
        Err(..) => println!("string : {}\n", string);
    }
    */
}
