use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref PRIMES: Mutex<Vec<u32>> = Mutex::new(vec![2, 3, 5]);
}

pub fn nth(n: u32) -> u32 {
    let mut primes = PRIMES.lock().unwrap();

    for i in primes.len()..=n as usize {
        let mut val = primes[i - 1]; // next odd

        'outer: loop {
            val += 2;

            for x in primes.iter() {
                if val % x == 0 {
                    continue 'outer;
                }
            }

            primes.push(val);
            break;
        }
    }

    primes[n as usize]
}
