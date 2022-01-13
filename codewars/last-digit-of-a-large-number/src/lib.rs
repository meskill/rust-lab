#[test]
fn returns_expected() {
    assert_eq!(last_digit("24525", "0"), 1);
    assert_eq!(last_digit("0", "0"), 1);
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("202", "17"), 2);
    assert_eq!(last_digit("10", "10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(
        last_digit(
            "3715290469715693021198967285016729344580685479654510946723",
            "68819615221552997273737174557165657483427362207517952651"
        ),
        7
    );
}

const ARRAY_LENGTH: usize = 4;

const LAST_DIGIT_BY_POWER: [[u8; ARRAY_LENGTH]; 10] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [6, 2, 4, 8],
    [1, 3, 9, 7],
    [6, 4, 6, 4],
    [5, 5, 5, 5],
    [6, 6, 6, 6],
    [1, 7, 9, 3],
    [6, 8, 4, 2],
    [1, 9, 1, 9],
];

pub fn last_digit(num: &str, power: &str) -> i32 {
    if power == "0" {
        return 1;
    }

    let num_last_digit: usize = num[num.len() - 1..].parse().unwrap();
    let power_last_digits: usize = power[if power.len() > 1 { power.len() - 2 } else { 0 }..]
        .parse()
        .unwrap();

    println!(
        "{} - {}: {}, {}",
        num, power, num_last_digit, power_last_digits
    );

    LAST_DIGIT_BY_POWER[num_last_digit][power_last_digits % ARRAY_LENGTH] as i32
}
