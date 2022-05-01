pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![String::new(); digits.len() + 1];
    }

    let digits: Vec<_> = digits.chars().collect();

    digits
        .windows(len)
        .map(|slice| slice.iter().collect())
        .collect()
}
