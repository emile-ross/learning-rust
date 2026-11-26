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
        match state
        {
        }
    }
        
}
