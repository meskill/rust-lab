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

        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "test");
        assert_eq!(config.filename, "poem.txt");
        assert_eq!(config.case_sensitive, false);
    }

    #[test]
    fn no_query() {
        let args: Vec<String> = vec!["name".to_string()];

        if let Err(s) = Config::new(args) {
            assert_eq!(s, "Please specify query");
        } else {
            panic!("Didn't throw");
        }
    }

    #[test]
    fn no_filename() {
        let args: Vec<String> = vec!["name".to_string(), "test".to_string()];

        if let Err(s) = Config::new(args) {
            assert_eq!(s, "Please specify filename");
        } else {
            panic!("Didn't throw");
        }
    }
}

impl Config {
    pub fn new(args: impl IntoIterator<Item = String>) -> Result<Self, &'static str> {
        let mut iter = args.into_iter();

        iter.next();

        let query = match iter.next() {
            Some(s) => s,
            None => return Err("Please specify query"),
        };
        let filename = match iter.next() {
            Some(s) => s,
            None => return Err("Please specify filename"),
        };

        let case_sensitive = env::var("MINIGREP_CASE_SENSITIVE").is_ok();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
