pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::from("");
    }

    let proverb: Vec<_> = list
        .iter()
        .zip(list.iter().skip(1))
        .map(|(&s1, &s2)| format!("For want of a {} the {} was lost.\n", s1, s2))
        .collect();

    format!("{}And all for the want of a {}.", proverb.join(""), list[0])
}
