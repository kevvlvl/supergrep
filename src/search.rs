use crate::config::config::Config;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.file_to_parse)?;
    let reader = BufReader::new(file);

    let mut total_file_count = 0;

    for (line_num, line_res) in reader.lines().enumerate().map(|(i, l)| (i + 1, l)) {
        let line = line_res?;

        let matches = extract_ctx(&line, &config.pattern);
        let match_count = matches.len();
        total_file_count = total_file_count + match_count;

        if match_count > 0 {
            println!("Found {} matches on line {}", match_count, line_num);

            for m in matches {
                println!("{}", m);
            }
        }
    }

    println!("Total matches found: {}", total_file_count);
    Ok(())
}

fn extract_ctx(line: &str, pattern: &str) -> Vec<String> {
    let mut matches = Vec::new();

    if pattern.len() == 0 {
        return matches;
    }

    let mut start_idx = 0;

    while let Some(relative_idx) = line[start_idx..].find(pattern) {
        let byte_idx = start_idx + relative_idx;

        let before_match = &line[..byte_idx];
        let after_match = &line[byte_idx + pattern.len()..];

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

        let found_str = format!(
            "{}{}[{}]{}{}",
            before_str, left_pad, pattern, right_pad, after_str
        );
        matches.push(found_str);

        start_idx = byte_idx + pattern.len();
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ctx_pattern_in_middle_of_line() {
        let line = "This is a test line with a pattern in the middle.";
        let pattern = "pattern";
        let expected = "line with a [pattern] in the middle.";

        assert_eq!(extract_ctx(line, pattern).get(0).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_pattern_at_start_of_line() {
        let line = "pattern is at the start of the line.";
        let pattern = "pattern";
        let expected = "[pattern] is at the";

        assert_eq!(extract_ctx(line, pattern).get(0).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_pattern_at_end_of_line() {
        let line = "The pattern is at the end of the pattern";
        let pattern = "pattern";
        let expected = vec!["The [pattern] is at the", "end of the [pattern]"];

        let result = extract_ctx(line, pattern);

        assert_eq!(result.len(), 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_ctx_no_whitespace_around_pattern() {
        let line = "Thisisapatternhere";
        let pattern = "pattern";
        let expected = "Thisisa[pattern]here";

        assert_eq!(extract_ctx(line, pattern).get(0).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_line_shorter_than_context() {
        let line = "pattern";
        let pattern = "pattern";
        let expected = "[pattern]";

        assert_eq!(extract_ctx(line, pattern).get(0).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_multiple_patterns_first_match() {
        let line = "pattern one and pattern two";
        let pattern = "pattern";
        let expected = vec!["[pattern] one and pattern", "pattern one and [pattern] two"];

        let result = extract_ctx(line, pattern);

        assert_eq!(result.len(), 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_ctx_pattern_with_special_characters() {
        let line = "This has a p@tt3rn! in it.";
        let pattern = "p@tt3rn!";
        let expected = "This has a [p@tt3rn!] in it.";

        assert_eq!(extract_ctx(line, pattern).get(0).unwrap(), expected);
    }

    #[test]
    fn test_extract_ctx_empty_line_no_pattern() {
        let line = "";
        let pattern = "pattern";
        assert_eq!(extract_ctx(line, pattern).len(), 0);
    }

    #[test]
    fn test_extract_ctx_pattern_not_found() {
        let line = "This line does not contain the word.";
        let pattern = "missing";
        assert_eq!(extract_ctx(line, pattern).len(), 0);
    }

    #[test]
    fn test_extract_ctx_long_line_with_pattern() {
        let line = "This is a very long line with many words before the target pattern that should be truncated to only a few words and also many words after the pattern which should also be truncated to a few words.";
        let pattern = "pattern";
        let expected = vec![
            "before the target [pattern] that should be",
            "words after the [pattern] which should also",
        ];

        let result = extract_ctx(line, pattern);

        assert_eq!(result.len(), 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_ctx_empty_pattern() {
        let line = "some text";
        let pattern = "";

        assert_eq!(extract_ctx(line, pattern).len(), 0)
    }
}
