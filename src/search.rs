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
