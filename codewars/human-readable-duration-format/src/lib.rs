#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(0), "now");
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
        assert_eq!(
            format_duration(132_030_240),
            "4 years, 68 days, 3 hours and 4 minutes"
        );
    }
}

const DIVIDERS: [u16; 5] = [60, 60, 24, 365, u16::MAX];
const NOUNS: [&str; 5] = ["second", "minute", "hour", "day", "year"];
const SEPARATOR: [&str; 5] = ["", " and ", ", ", ", ", ", "];

pub fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return String::from("now");
    }

    DIVIDERS
        .iter()
        .scan(seconds, |acc, &val| {
            let result = *acc % val as u64;

            *acc = *acc / val as u64;

            Some(result)
        })
        .zip(NOUNS)
        .filter(|x| x.0 > 0)
        .map(|x| {
            if x.0 > 1 {
                format!("{} {}s", x.0, x.1)
            } else {
                format!("{} {}", x.0, x.1)
            }
        })
        .zip(SEPARATOR)
        .fold(String::new(), |acc, x| format!("{}{}{}", x.0, x.1, acc))
}
