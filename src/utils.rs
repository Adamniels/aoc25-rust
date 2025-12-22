// Utility functions that can be reused across multiple days

/// Parse input into a vector of lines
pub fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Parse input into a vector of numbers
#[allow(unused)]
pub fn parse_numbers<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| line.parse::<T>().expect("Failed to parse number"))
        .collect()
}

/// Split input by blank lines (useful for grouped data)
#[allow(unused)]
pub fn split_by_blank_lines(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "line1\nline2\nline3";
        assert_eq!(parse_lines(input), vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_parse_numbers() {
        let input = "1\n2\n3";
        assert_eq!(parse_numbers::<i32>(input), vec![1, 2, 3]);
    }
}
