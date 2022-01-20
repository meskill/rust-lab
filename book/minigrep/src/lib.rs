mod config;
mod search;

use config::Config;
use std::error::Error;
use std::fs;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let config = Config::new(args)?;

    let contents = fs::read_to_string(&config.filename)?;
    let found = if config.case_sensitive {
        search::search_case_sensitive(&config.query, &contents)
    } else {
        search::search_case_insensitive(&config.query, &contents)
    };

    for line in found {
        println!("{}", line);
    }

    Ok(())
}
