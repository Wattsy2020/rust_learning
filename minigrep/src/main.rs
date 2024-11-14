use std::io::ErrorKind;
use crate::argparse::Arguments;

mod argparse;
mod minigrep;

fn read_file(arguments: &Arguments) -> String {
    let filename = arguments.filename();
    match std::fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => panic!("File {} not found", filename),
            ErrorKind::PermissionDenied => panic!("Permission denied for file {}", filename),
            err => panic!("Failed to parse file, received error: {}", err)
        }
    }
}

fn main() {
    let arguments = argparse::read_arguments();
    println!("Searching {} for pattern: {}", arguments.filename(), arguments.pattern());
    let file_contents = read_file(&arguments);
    let matches = minigrep::parse_matches(&file_contents, arguments.pattern());
    matches.for_each(|_match| println!("{}", _match));
}
