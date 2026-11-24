fn main() {
    let names: Vec<&str> = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter()
    {
        println!("{}", name);
    }

    let values: Vec<i32> = vec![4, 5, 1, 2];

    for val in values.iter()
    {
        println!("{}", val);
    }
}

/* same thing but in C : 

char *array[3] = { "Bob", "Frank", "Ferris" };
for (int i = 0; i < 3; i++)
{
    printf("%s\n", array[i]);
}

same thing but in java

String[] names = { "Bob", "Frank", "Ferris" };
for (int i = 0; i < names.length; i++)
{
    System.out.println(names[i]);
}

 */
