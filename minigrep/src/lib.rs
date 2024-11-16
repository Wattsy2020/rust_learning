mod argparse;
mod match_parsing;

pub use argparse::{Arguments, ArgumentError, ArgumentError::*};
pub use match_parsing::Match;

pub fn run(arguments: &Arguments) -> Result<Vec<Match>, std::io::Error> {
    let file_contents = std::fs::read_to_string(arguments.filename())?;
    let matches = Match::parse_matches(&file_contents, arguments.pattern());
    Ok(matches.collect())
}