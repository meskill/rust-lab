fn bottle_call(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    }
}

fn down_call(n: u32) -> String {
    match n {
        1 => "it down".to_string(),
        _ => "one down".to_string(),
    }
}

pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }

    format!("{0} of beer on the wall, {0} of beer.\nTake {1} and pass it around, {2} of beer on the wall.\n", bottle_call(n), down_call(n), bottle_call(n-1))
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .into_iter()
        .rev()
        .map(|x| verse(x))
        .collect::<Vec<_>>()
        .join("\n")
}
