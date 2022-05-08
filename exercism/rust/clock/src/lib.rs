use std::fmt::Display;

const MINUTES_IN_DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: u32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = hours % 24;
        let minutes = hours * 60 + minutes;
        let minutes = minutes.rem_euclid(MINUTES_IN_DAY) as u32;

        Self { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: (self.minutes as i32 + minutes).rem_euclid(MINUTES_IN_DAY) as u32,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.minutes / 60, self.minutes % 60)
    }
}
