fn is_valid(code: &str) -> bool {
    let code: Vec<_> = code.chars().filter(|c| !c.is_whitespace()).rev().collect();

    if code.len() < 2 {
        return false;
    }

    if code.iter().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    let sum = code.iter().enumerate().fold(0, |acc, (i, c)| {
        let n = c.to_digit(10).unwrap();
        let n = if i % 2 == 0 { n } else { n * 2 };

        acc + (if n > 9 { n - 9 } else { n })
    });

    sum % 10 == 0
}

pub struct Luhn {
    repr: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        is_valid(&self.repr)
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self {
            repr: input.to_string(),
        }
    }
}
