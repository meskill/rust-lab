pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();

    while n % 2 == 0 {
        result.push(2);
        n /= 2;
    }

    while n > 1 {
        for div in (3..).step_by(2) {
            if n % div == 0 {
                result.push(div);
                n /= div;
                break;
            }
        }
    }

    result
}
