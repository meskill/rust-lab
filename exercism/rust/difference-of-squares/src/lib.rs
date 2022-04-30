pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).into_iter().sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).into_iter().map(|x| x * x).sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
