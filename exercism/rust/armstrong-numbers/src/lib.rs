pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();

    num == num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(len as u32))
        .sum()
}
