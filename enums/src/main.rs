#[derive(Copy, Clone)]

enum ParsingStates 
{
    UndefinedSearch,
    FlagSearch,
    LetterSearch,
    IndexSearch,
    FilenameSearch,
    WordSearch,
    ListSearch,
    ModeSearch
}

fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    let mut state : ParsingStates =  ParsingStates::UndefinedSearch;

    let data_ch: char = '\0';

    for (i, arg) in argv.iter().enumerate()
    {
        if state as i32 == ParsingStates::FlagSearch as i32
        {
            /* parsing arguments (-s -a -i -x) */
            if arg == "-s" || arg == "--strict"
            {
                state = ParsingStates::LetterSearch;
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
        }
        else 
        {
            match state {
                ParsingStates::LetterSearch => getLetter(&data_ch, arg.clone()), 
                ParsingStates::IndexSearch => getLetter(&data_ch, arg.clone()), 
                ParsingStates::FlagSearch => println!("Broken logic (state is FlagSearch)\n"), 
                _=> todo!()
            }
        }
        println!("Arg: {} at {}", arg, i);
    }
}



fn getLetter(data_ch: &char, argument: String)
{
    if argument.len() > 1
    {
        println!("Argument is of size greater than 0");
    } else {
        println!("You're fine");
    }
    // *data_ch = argument[0];
}
