#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct";

        assert_eq!(
            search_case_sensitive(query, contents),
            vec!["safe, fast, productive."]
        )
    }

    #[test]
    fn two_results_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            search_case_insensitive(query, contents),
            vec!["Rust:", "Trust me."]
        )
    }
}

pub fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let query = query.to_lowercase();

    for line in text.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_sensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in text.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}
