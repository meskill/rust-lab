pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets_stack: Vec<char> = Vec::new();

    for c in string.chars() {
        let last = *brackets_stack.last().or(Some(&' ')).unwrap();
        let closed_bracket = matching_bracket(last);

        match c {
            '(' | '[' | '{' => brackets_stack.push(c),
            ')' | ']' | '}' => {
                if c != closed_bracket {
                    return false;
                }

                brackets_stack.pop();
            }
            _ => (),
        }
    }

    brackets_stack.is_empty()
}

fn matching_bracket(open_bracket: char) -> char {
    match open_bracket {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => ' ',
    }
}
