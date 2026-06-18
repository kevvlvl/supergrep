#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_build_success() {
        let args = vec![
            String::from("program_name"),
            String::from("test_pattern"),
            String::from("test_file.txt"),
        ];
        let config = Config::build(&args).unwrap();

        assert_eq!(config.pattern, "test_pattern");
        assert_eq!(config.file_to_parse, "test_file.txt");
    }

    #[test]
    fn test_build_error_not_enough_args() {
        let args = vec![String::from("program_name"), String::from("test_pattern")];
        let config_result = Config::build(&args);

        assert!(config_result.is_err());
        assert_eq!(
            config_result.unwrap_err(),
            "Expected 2 arguments, Usage: supergrep <pattern> file_to_parse.txt"
        );
    }

    #[test]
    fn test_build_error_too_many_args() {
        let args = vec![
            String::from("program_name"),
            String::from("test_pattern"),
            String::from("test_file.txt"),
            String::from("extra_arg"),
        ];
        let config_result = Config::build(&args);

        assert!(config_result.is_err());
        assert_eq!(
            config_result.unwrap_err(),
            "Expected 2 arguments, Usage: supergrep <pattern> file_to_parse.txt"
        );
    }
}
