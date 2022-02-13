pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let chars = message.chars();

    let has_alphabetic = chars.clone().any(|c| c.is_ascii_alphabetic());
    let is_all_capital = has_alphabetic
        && chars
            .clone()
            .all(|c| !c.is_alphabetic() || c.is_ascii_uppercase());
    let is_question = message.ends_with("?");

    match message {
        "" => "Fine. Be that way!",
        _ if is_all_capital && is_question => "Calm down, I know what I'm doing!",
        _ if is_all_capital => "Whoa, chill out!",
        _ if is_question => "Sure.",
        _ => "Whatever.",
    }
}
