mod config;
mod search;

use config::Config;
use std::error::Error;
use std::fs;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let config = Config::new(args)?;

    let contents = fs::read_to_string(&config.filename)?;
    let found = search::search(&config.query, &contents, config.case_sensitive);

    for line in found {
        println!("{}", line);
    }

    Ok(())
}
