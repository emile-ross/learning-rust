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

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let mut state : ParsingStates =  ParsingStates::UndefinedSearch;

    let data_ch: char = '\0';

    for (i, arg) in argv.iter().enumerate()
    {
        /* parsing arguments (-s -a -i -x) */
        if arg == "-s" || arg == "--strict"
        {
            state = ParsingStates::LetterSearch;
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
        /* if argument is unknown */
        /* 
        if state_int != ParsingStates::UndefinedSearch as i32
            println!("Unknown argument: {}", arg);
        */

        /* 
        match state { 
            ParsingStates::UndefinedSearch => println!("Is undefined"), 
            _=> println!("A"),
        }
        println!("Unknown State");
        */

        if state as i32 != ParsingStates::UndefinedSearch as i32
        {
            /*  TODO Add a switch for states 
            match state {
            }
            */
        }
        println!("Arg: {} at {}", arg, i);
    }
}
