#[cfg(test)]
mod tests {
    use super::patterns::*;
    use super::*;

    #[test]
    fn index_from_char_tests() {
        assert_eq!(index_from_char('A'), 0);
        assert_eq!(index_from_char('B'), 1);
        assert_eq!(index_from_char('F'), 5);
        assert_eq!(index_from_char('H'), 7);
    }

    #[test]
    fn index_to_point_tests() {
        assert_eq!(index_to_point(0), 0b1);
        assert_eq!(index_to_point(1), 0b10);
        assert_eq!(index_to_point(3), 0b1000);
        assert_eq!(index_to_point(5), 0b100000);
    }

    #[test]
    fn basic_tests() {
        assert_eq!(count_patterns('A', 0), 0);
        assert_eq!(count_patterns('A', 10), 0);
        assert_eq!(count_patterns('B', 1), 1);
        assert_eq!(count_patterns('C', 2), 5);
        assert_eq!(count_patterns('D', 3), 37);
        assert_eq!(count_patterns('E', 4), 256);
        assert_eq!(count_patterns('E', 8), 23280);
    }
}

mod patterns {
    pub fn index_from_char(point: char) -> u8 {
        point.to_digit(20).unwrap() as u8 - 10
    }

    pub fn index_to_point(index: u8) -> u16 {
        1u16 << index
    }

    const NEXT_POINT_MAPPING: [[u16; 9]; 9] = [
        [0, 0, 0b10, 0, 0, 0, 0b1000, 0, 0b10000],
        [0, 0, 0, 0, 0, 0, 0, 0b10000, 0],
        [0b10, 0, 0, 0, 0, 0, 0b10000, 0, 0b100000],
        [0, 0, 0, 0, 0, 0b10000, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0b10000, 0, 0, 0, 0, 0],
        [0b1000, 0, 0b10000, 0, 0, 0, 0, 0, 0b10000000],
        [0, 0b10000, 0, 0, 0, 0, 0, 0, 0],
        [0b10000, 0, 0b100000, 0, 0, 0, 0b10000000, 0, 0],
    ];

    #[derive(Debug)]
    pub struct Pattern {
        occupied: u16,
        current: u8,
    }

    impl Pattern {
        pub fn new(start: char) -> Self {
            let current = index_from_char(start);
            let occupied = index_to_point(current);

            Self { current, occupied }
        }

        pub fn next_patterns(self) -> impl Iterator<Item = Self> {
            let current = self.current;
            let occupied = self.occupied;
            let next_occupied = occupied;

            (0u8..9)
                .map(|i| (i, index_to_point(i)))
                .filter(move |&(i, point)| {
                    let mapping = NEXT_POINT_MAPPING[current as usize][i as usize];

                    i != current
                        && occupied & point == 0
                        && (mapping == 0 || mapping & occupied > 0)
                })
                .map(move |(i, point)| Self {
                    current: i,
                    occupied: next_occupied | point,
                })
        }
    }
}

use patterns::Pattern;

pub fn count_patterns(from: char, length: u8) -> u64 {
    if length == 0 {
        return 0;
    }

    let current_pattern = Pattern::new(from);

    let mut patterns: Box<dyn Iterator<Item = Pattern>> =
        Box::new(vec![current_pattern].into_iter());

    for _ in 1..length {
        patterns = Box::new(patterns.flat_map(|p| p.next_patterns()));
    }

    patterns.count() as u64
}
