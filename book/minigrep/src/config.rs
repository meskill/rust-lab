use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_config() {
        let args = vec![
            "minigrep".to_string(),
            "test".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, "test");
        assert_eq!(config.filename, "poem.txt");
        assert_eq!(config.case_sensitive, false);
    }

    #[test]
    fn not_enough_arguments() {
        let args: Vec<String> = vec![];

        if let Err(s) = Config::new(&args) {
            assert_eq!(s, "Not enough arguments");
        } else {
            panic!("Didn't throw");
        }
    }
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("MINIGREP_CASE_SENSITIVE").is_ok();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
