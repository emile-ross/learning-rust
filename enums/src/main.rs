#[derive(Copy, Clone)]

enum Flag
{
    Strict,
    Excludes,
    Includes,
    Absent,
    Unknown
}

enum ParsingStates 
{
    UndefinedSearch,
    FlagSearch,
    LetterSearch,
    IndexSearch,
    FilenameSearch,
    WordSearch,
    ModeSearch,
    ListSearch
}

fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    let mut state : ParsingStates =  ParsingStates::UndefinedSearch;
    let mut prev_flag_type : Flag =  ParsingStates::UndefinedSearch;

    let data_ch: char = '\0';
    let mut data_idx: usize = 0;

    for (i, arg) in argv.iter().enumerate()
    { 
        match state
        {
            ParsingStates::FlagSearch || ParsingStates::ModeSearch => {
                if arg == "-i" || arg == "--includes"
                {
                    println!("Includes flag");
                }
                else if arg == "-x" || arg == "--excludes"
                {
                    println!("Excludes flag");
                    prev_flag_type = Flag::Excludes;
                }
                else if arg == "-f" || arg == "--file"
                {
                    state = ParsingStates::FilenameSearch;
                }
                else if arg == "-w" || arg == "--word-list"
                {
                    state = ParsingStates::ListSearch;
                }
                else if arg == "-v" || arg == "--validate"
                {
                    state = ParsingStates::WordSearch;
                }
                else if arg == "--release" || arg == "--version"
                {
                    /* return the version number */
                    state = ParsingStates::UndefinedSearch;
                }
            },
            ParsingStates::LetterSearch => {
                get_letter(&data_ch, arg.clone())
            }, 
            ParsingStates::IndexSearch => data_idx = arg.parse::<usize>().expect("Invalid integer"),
            ParsingStates::FlagSearch => println!("Broken logic (state is FlagSearch)\n"), 
        
            ParsingStates::ListSearch => {
                println!("This is the word list: {}", arg);
            },
            ParsingStates::ModeSearch => {
            },
            _=> println!("AA"),
        }
        println!("Arg: {} at {}", arg, i);
    }
}


fn get_letter(data_ch: &char, argument: String)
{
    if argument.len() > 1 {
        println!("Argument is of size greater than 0");
    } else {
        println!("You're fine");
    }
    // TODO set the data_ch to the first letter of the argument
    // *data_ch = argument[0];
}

/* 
fn get_index(mut data_idx: &usize, argument: &String)
{
    data_idx = argument.parse::<usize>().expect("Bruh");
}
*/
