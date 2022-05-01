const LIMIT: u64 = u64::MAX / 3;

pub fn collatz(mut n: u64) -> Option<u64> {
    let mut step_num = 0;

    if n == 0 {
        return None;
    }

    loop {
        if n == 1 {
            return Some(step_num);
        }

        if n % 2 == 0 {
            n /= 2;
        } else {
            if n >= LIMIT {
                return None;
            }

            n = 3 * n + 1;
        }

        step_num += 1;
    }
}
