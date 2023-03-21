fn elder_age(m: u64, n: u64, l: u64, t: u64) -> u64 {
    macro_rules! rem {
        ($expr: expr) => {
            (($expr) % t)
        };
    }

    let (m, n) = (n.min(m), n.max(m));

    if n <= 1 {
        return 0;
    }

    let mut p = n.next_power_of_two();

    if p > n {
        p >>= 1;
    }

    let n_next = n.min(p);
    let m_next = m.min(p);

    let mut sum = 0;

    if p > l {
        let row_sum = if l % 2 == 0 {
            rem!((p - l) >> 1) * rem!(p - l - 1)
        } else {
            rem!((p - l - 1) >> 1) * rem!(p - l)
        };
        sum += rem!(row_sum) * rem!(m_next);
        sum %= t;
    }

    if n > p && m > p {
        sum += elder_age(n - p, m - p, l, t);
        sum %= t;
    }

    let lp = p.min(l);
    let l = l - lp;

    if n > p {
        let n = n - p;
        let p = rem!(p - lp);
        let e = rem!(rem!(n) * rem!(m_next));

        sum += rem!(e * p) + elder_age(n, m_next, l, t);
        sum %= t;
    }

    if m > p {
        let m = m - p;
        let p = rem!(p - lp);
        let e = rem!(rem!(n_next) * rem!(m));

        sum += rem!(e * p) + elder_age(n_next, m, l, t);
        sum %= t;
    }

    sum % t
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(m: u64, n: u64, l: u64, t: u64) -> u64 {
        assert!(m < 10000, "Naive solution is too damn slow");
        assert!(n < 10000, "Naive solution is too damn slow");

        let mut s: u64 = 0;

        for x in 0..n {
            for y in 0..m {
                s += l.max(x ^ y) - l;
            }

            s %= t;
        }

        s % t
    }

    macro_rules! assert_naive {
        ($m: expr, $n: expr, $l: expr, $t: expr) => {
            assert_eq!(elder_age($m, $n, $l, $t), naive($m, $n, $l, $t))
        };
    }

    #[test]
    fn test_with_naive_power_of_two() {
        assert_naive!(8, 5, 1, 100);
        assert_naive!(8, 8, 0, 100007);
        assert_naive!(1, 1, 1, 10);
        assert_naive!(2, 2, 0, 10);
        assert_naive!(4, 4, 2, 6);
        assert_naive!(16, 12, 5, 356);
        assert_naive!(1024, 768, 13, 322);
    }

    #[test]
    fn test_with_naive() {
        assert_naive!(1, 5, 3, 10);
        assert_naive!(3, 3, 0, 2);
        assert_naive!(5, 3, 0, 2);
        assert_naive!(6, 7, 0, 104);
        assert_naive!(6, 7, 2, 104);
        assert_naive!(12, 3, 0, 100);
        assert_naive!(14, 5, 3, 100);
        assert_naive!(28, 36, 6, 36);
        assert_naive!(12, 3, 34, 5);
        assert_naive!(25, 31, 1, 100);
        assert_naive!(440, 445, 17, 12935);
        assert_naive!(256, 257, 258, 5);
    }

    #[test]
    fn example_tests() {
        assert_eq!(elder_age(8, 5, 1, 100), 5);
        assert_eq!(elder_age(8, 8, 0, 100007), 224);
        assert_eq!(elder_age(25, 31, 0, 100007), 11925);
        assert_eq!(elder_age(5, 45, 3, 1000007), 4323);
        assert_eq!(elder_age(31, 39, 7, 2345), 1586);
        assert_eq!(elder_age(545, 435, 342, 1000007), 808451);

        // You need to run this test very quickly before attempting the actual tests :)
        assert_eq!(
            elder_age(28827050410, 35165045587, 7109602, 13719506),
            5456283
        );
    }
}
