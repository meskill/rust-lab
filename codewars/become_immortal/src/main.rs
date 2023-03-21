fn show(m: u64, n: u64, l: u64, t: u64) -> u64 {
    let n = n;
    let row_sum = (m - l - 1) * (m - l) / 2;
    let mut vec: Vec<Vec<u64>> = Vec::new();
    let mut s: u64 = 0;

    for x in 0..n {
        let mut row = Vec::new();
        for y in 0..m {
            row.push(x ^ y);
            s += l.max(x ^ y) - l;
        }
        println!("{row:2?}");
        vec.push(row);
    }

    println!("sum: {s}");

    for (i, row) in vec.iter().enumerate() {
        println!(
            "{}, {}",
            row.iter().sum::<u64>(),
            row.iter().skip(i + 1).sum::<u64>()
        );
    }

    (n * row_sum) % t
}
pub fn main() {
    show(256, 257, 0, 100);
}
