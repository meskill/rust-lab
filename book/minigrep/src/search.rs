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
            search(query, contents, true),
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

        assert_eq!(search(query, contents, false), vec!["Rust:", "Trust me."])
    }
}

pub fn search<'a>(query: &str, text: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let lowercased_query: String;

    let normalized_query = if case_sensitive {
        query
    } else {
        lowercased_query = query.to_lowercase();
        &lowercased_query
    };

    for line in text.lines() {
        let lowercased_line: String;

        let normalized_line = if case_sensitive {
            line
        } else {
            lowercased_line = line.to_lowercase();
            &lowercased_line
        };

        if normalized_line.contains(normalized_query) {
            result.push(line);
        }
    }

    result
}
