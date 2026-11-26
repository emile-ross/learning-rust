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

    for (i, arg) in argv.iter().enumerate()
    {
        /* parsing arguments (-s -a -i -x) */
        if arg == "-w" || arg == "--word-list"

        if arg == "-w" || arg == "--word-list"
        {
            state = ParsingStates::ListSearch;
        }
        else if arg == "-v" || arg == "--validate"
        {
            state = ParsingStates::ListSearch;
        }
        else if arg == "--release" || arg == "--version"
        {
            /* return the version number */
            state = ParsingStates::UndefinedSearch;
        }
        match state
        {
        }
        println!("{} at {}", arg, i)
    }
        
}
