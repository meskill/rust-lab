#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        let tests = vec![
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6),
        ];

        for test in tests {
            assert_eq!(last_digit(&test.0), test.1);
        }
    }
}

// all thanks to https://stackoverflow.com/questions/51304865/last-digit-of-power-list
pub fn last_digit(lst: &[u64]) -> u64 {
    lst.iter().rfold(1, |acc, &x| {
        u64::pow(
            if x < 20 { x } else { x % 20 + 20 },
            if acc < 4 { acc } else { acc % 4 + 4 } as u32,
        )
    }) % 10
}
