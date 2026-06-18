use std::env;
use std::process;

mod config;
mod errors;
mod search;
use crate::config::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(errors::ERR_INVALID_ARGS);
    });

    println!("Matching for '{}'", config.pattern);
    println!("File to search: '{}'", config.file_to_parse);

    if let Err(read_error) = search::read(config) {
        eprintln!(
            "Problem parsing config and/or reading the config: {}",
            read_error
        );
        process::exit(errors::ERR_READING_FILE);
    }
}
