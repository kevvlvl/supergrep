use crate::config::config::Config;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.file_to_parse)?;
    let reader = BufReader::new(file);

    let mut matches_count = 0;

    for (line_num, line_res) in reader.lines().enumerate().map(|(i, l)| (i + 1, l)) {
        let line = line_res?;

        if line.contains(&config.pattern) {
            matches_count += 1;

            if let Some(ctx) = extract_ctx(&line, &config.pattern) {
                println!("{}: ... {}", line_num, ctx);
            }
        }
    }

    println!("Total matches found: {}", matches_count);
    Ok(())
}

fn extract_ctx(line: &str, pattern: &str) -> Option<String> {
    let byte_index = line.find(pattern)?;

    let before_match = &line[..byte_index];
    let after_match = &line[byte_index + pattern.len()..];

    let words_before: Vec<&str> = before_match
        .split_whitespace()
        .rev()
        .take(3)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect();

    let words_after: Vec<&str> = after_match.split_whitespace().take(3).collect();

    let before_str = words_before.join(" ");
    let after_str = words_after.join(" ");

    let left_pad = if before_match.ends_with(char::is_whitespace) || before_match.is_empty() {
        if before_str.is_empty() { "" } else { " " }
    } else {
        ""
    };

    let right_pad = if after_match.starts_with(char::is_whitespace) || after_match.is_empty() {
        if after_match.is_empty() { "" } else { " " }
    } else {
        ""
    };

    Some(format!(
        "{}{}[{}]{}{}",
        before_str, left_pad, pattern, right_pad, after_str
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ctx_pattern_in_middle_of_line() {
        let line = "This is a test line with a pattern in the middle.";
        let pattern = "pattern";
        let expected = "line with a [pattern] in the middle.";
        assert_eq!(extract_ctx(line, pattern).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_pattern_at_start_of_line() {
        let line = "pattern is at the start of the line.";
        let pattern = "pattern";
        let expected = "[pattern] is at the";
        assert_eq!(extract_ctx(line, pattern).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_pattern_at_end_of_line() {
        let line = "The pattern is at the end of the pattern";
        let pattern = "pattern";
        let expected = "The [pattern] is at the";
        assert_eq!(extract_ctx(line, pattern).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_no_whitespace_around_pattern() {
        let line = "Thisisapatternhere";
        let pattern = "pattern";
        let expected = "Thisisa[pattern]here";
        assert_eq!(extract_ctx(line, pattern).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_line_shorter_than_context() {
        let line = "pattern";
        let pattern = "pattern";
        let expected = "[pattern]";
        assert_eq!(extract_ctx(line, pattern).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_multiple_patterns_first_match() {
        let line = "pattern one and pattern two";
        let pattern = "pattern";
        let expected = "[pattern] one and pattern";
        assert_eq!(extract_ctx(line, pattern).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_pattern_with_special_characters() {
        let line = "This has a p@tt3rn! in it.";
        let pattern = "p@tt3rn!";
        let expected = "This has a [p@tt3rn!] in it.";
        assert_eq!(extract_ctx(line, pattern).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_empty_line_no_pattern() {
        let line = "";
        let pattern = "pattern";
        assert!(extract_ctx(line, pattern).is_none());
    }

    #[test]
    fn test_extract_ctx_pattern_not_found() {
        let line = "This line does not contain the word.";
        let pattern = "missing";
        assert!(extract_ctx(line, pattern).is_none());
    }

    #[test]
    fn test_extract_ctx_long_line_with_pattern() {
        let line = "This is a very long line with many words before the target pattern that should be truncated to only a few words and also many words after the pattern which should also be truncated to a few words.";
        let pattern = "pattern";
        let expected = "before the target [pattern] that should be";
        assert_eq!(extract_ctx(line, pattern).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_empty_pattern() {
        let line = "some text";
        let pattern = "";
        assert!(extract_ctx(line, pattern).is_some());
    }
}
