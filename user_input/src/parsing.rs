pub mod parsing {
    pub mod get_integer(string: String)
    {
        let value: i32 = string.parse::<i32>().expect("Invalid number\n");
        println!("{}", string);
        return value;
    }
}
