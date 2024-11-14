use crate::argparse::ArgumentError::{NotEnoughArguments, TooManyArguments};

#[derive(Debug)]
pub struct Arguments {
    filename: String,
    pattern: String,
}

impl Arguments {
    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }
}

#[derive(Debug)]
enum ArgumentError {
    NotEnoughArguments(u8),
    TooManyArguments,
}

fn try_read_arguments() -> Result<Arguments, ArgumentError> {
    let mut args = std::env::args();
    let _ = args
        .next()
        .expect("Rust should always provide the filename as the first argument");
    let filename = args.next().ok_or(NotEnoughArguments(0))?;
    let pattern = args.next().ok_or(NotEnoughArguments(1))?;
    match args.next() {
        Some(_) => Err(TooManyArguments),
        None => Ok(Arguments { filename, pattern }),
    }
}

const CORRECT_USAGE: &str = "correct usage: `cargo run -- filename pattern`";

/// Read arguments and panic on incorrect user input
pub fn read_arguments() -> Arguments {
    match try_read_arguments() {
        Err(TooManyArguments) => panic!("Too many arguments provided!\nExpected two\n{}", CORRECT_USAGE),
        Err(NotEnoughArguments(num_args)) => panic!("Not enough arguments provided!\nExpected 2, received {}\n{}", num_args, CORRECT_USAGE),
        Ok(arguments) => arguments
    }
}
