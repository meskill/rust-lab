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

        let result: Vec<&str> = search(query, contents, true).collect();

        assert_eq!(result, vec!["safe, fast, productive."])
    }

    #[test]
    fn two_results_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let result: Vec<&str> = search(query, contents, false).collect();

        assert_eq!(result, vec!["Rust:", "Trust me."])
    }
}

pub fn search<'a>(
    query: &'a str,
    text: &'a str,
    case_sensitive: bool,
) -> impl Iterator<Item = &'a str> {
    let f: Box<dyn Fn(&str) -> bool> = if case_sensitive {
        Box::new(move |line| line.contains(query))
    } else {
        let lowercased_query = query.to_lowercase();

        Box::new(move |line| line.to_lowercase().contains(&lowercased_query))
    };

    text.lines().filter(move |&line| f(line))
}
