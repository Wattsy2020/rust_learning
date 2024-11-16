use crate::argparse::ArgumentError::{NotEnoughArguments, TooManyArguments};

#[derive(Debug, PartialEq)]
pub enum ArgumentError {
    NotEnoughArguments(u8),
    TooManyArguments,
}

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

    fn build_from_args(mut args: impl Iterator<Item=String>) -> Result<Arguments, ArgumentError> {
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
    
    /// Parse the CLI arguments
    pub fn build() -> Result<Arguments, ArgumentError> {
        Self::build_from_args(std::env::args())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_from_args(vec: Vec<&str>) -> Result<Arguments, ArgumentError> {
        Arguments::build_from_args(vec.iter().map(|str| str.to_string()))
    }

    #[test]
    fn test_read_failure_too_few_arguments() {
        assert_eq!(
            build_from_args(vec!["binary_name"]),
            Err(NotEnoughArguments(0))
        );
        assert_eq!(
            build_from_args(vec!["binary_name", "file"]),
            Err(NotEnoughArguments(1))
        );
    }

    #[test]
    fn test_read_failure_too_many_arguments() {
        assert_eq!(
            build_from_args(vec!["binary_name", "file", "pattern", "extra"]),
            Err(TooManyArguments)
        );
    }

    #[test]
    fn test_successful_read() {
        let result_arguments = build_from_args(vec!["binary_name", "file", "pattern"]);
        let expected_arguments = Arguments {
            filename: "file".to_string(),
            pattern: "pattern".to_string(),
        };
        assert_eq!(result_arguments, Ok(expected_arguments));
    }
}
