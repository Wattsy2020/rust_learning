use std::io::ErrorKind;

// refer to the lib crate by the name of the package
use minigrep_learning_2048::{run, Arguments, ArgumentError::*};

const CORRECT_USAGE: &str = "correct usage: `cargo run -- filename pattern`";

/// Read arguments and panic on incorrect user input
fn read_arguments() -> Arguments {
    match Arguments::build() {
        Err(TooManyArguments) => panic!(
            "Too many arguments provided!\nExpected two\n{}",
            CORRECT_USAGE
        ),
        Err(NotEnoughArguments(num_args)) => panic!(
            "Not enough arguments provided!\nExpected 2, received {}\n{}",
            num_args, CORRECT_USAGE
        ),
        Ok(arguments) => arguments,
    }
}

fn main() {
    let arguments = read_arguments();
    println!(
        "Searching {} for pattern: {}",
        arguments.filename(),
        arguments.pattern()
    );

    let matches = match run(&arguments) {
        Ok(matches) => matches,
        Err(file_error) => match file_error.kind() {
            ErrorKind::NotFound => panic!("File {} not found", arguments.filename()),
            ErrorKind::PermissionDenied => panic!("Permission denied for file {}", arguments.filename()),
            err => panic!("Failed to parse file, received error: {}", err),
        }
    };

    matches.iter().for_each(|_match| println!("{}", _match));
}
