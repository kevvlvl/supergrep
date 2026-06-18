pub struct Config {
    pub pattern: String,
    pub file_to_parse: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Expected 2 arguments, Usage: supergrep <pattern> file_to_parse.txt");
        }

        let pattern = args[1].clone();
        let file_to_parse = args[2].clone();

        Ok(Config {
            pattern,
            file_to_parse,
        })
    }
}
