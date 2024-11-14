use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Match {
    // not having pub prevents external users from constructing the Match struct
    line_number: usize,
    line_content: String,
}

impl Match {
    #[allow(dead_code)]
    pub fn line_number(&self) -> usize {
        self.line_number
    }

    #[allow(dead_code)]
    pub fn line_content(&self) -> &String {
        &self.line_content
    }

    fn new(line_number: usize, line_content: &str) -> Match {
        Match {
            line_number,
            line_content: line_content.to_string(),
        }
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.line_number, self.line_content)
    }
}

fn matches_pattern(line: &str, pattern: &str) -> bool {
    line.contains(pattern)
}

pub fn parse_matches<'a>(contents: &'a str, pattern: &'a str) -> impl 'a + Iterator<Item = Match> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| matches_pattern(line, pattern))
        .map(|(line_number, line)| Match::new(line_number + 1, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_matches() {
        let results: Vec<Match> =
            parse_matches("Hello there\nYou are a bold one", "Kenobi").collect();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_single_line_match() {
        let results: Vec<Match> = parse_matches("Hello there! General Kenobi", "Kenobi").collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], Match::new(1, "Hello there! General Kenobi"));
    }

    const TEST_DATA: &str = "I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

    #[test]
    fn test_multi_line_match() {
        let results: Vec<Match> = parse_matches(TEST_DATA, "body").collect();
        let expected_results = vec![
            Match::new(1, "I'm nobody! Who are you?"),
            Match::new(2, "Are you nobody, too?"),
            Match::new(6, "How dreary to be somebody!"),
        ];
        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_display() {
        let result = format!("{}", Match::new(5, "hello world"));
        assert_eq!(result, "5: hello world");
    }
}
