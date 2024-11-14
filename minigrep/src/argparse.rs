use crate::argparse::ArgumentError::{NotEnoughArguments, TooManyArguments};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
enum ArgumentError {
    NotEnoughArguments(u8),
    TooManyArguments,
}

fn try_read_arguments(mut args: impl Iterator<Item = String>) -> Result<Arguments, ArgumentError> {
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
    match try_read_arguments(std::env::args()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn call_try_read_arguments(vec: Vec<&str>) -> Result<Arguments, ArgumentError> {
        try_read_arguments(vec.iter().map(|str| str.to_string()))
    }

    #[test]
    fn test_read_failure_too_few_arguments() {
        assert_eq!(
            call_try_read_arguments(vec!["binary_name"]),
            Err(NotEnoughArguments(0))
        );
        assert_eq!(
            call_try_read_arguments(vec!["binary_name", "file"]),
            Err(NotEnoughArguments(1))
        );
    }

    #[test]
    fn test_read_failure_too_many_arguments() {
        assert_eq!(
            call_try_read_arguments(vec!["binary_name", "file", "pattern", "extra"]),
            Err(TooManyArguments)
        );
    }

    #[test]
    fn test_successful_read() {
        let result_arguments = call_try_read_arguments(vec!["binary_name", "file", "pattern"]);
        let expected_arguments = Arguments {
            filename: "file".to_string(),
            pattern: "pattern".to_string(),
        };
        assert_eq!(result_arguments, Ok(expected_arguments));
    }
}
