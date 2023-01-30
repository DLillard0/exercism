pub fn reply(message: &str) -> &str {
    match message.trim() {
        s if s.len() == 0 => "Fine. Be that way!",
        s if is_question(s) && is_yelling(s) => "Calm down, I know what I'm doing!",
        s if is_question(s) => "Sure.",
        s if is_yelling(s) => "Whoa, chill out!",
        _ => "Whatever."
    }
}

fn has_letter(str: &str) -> bool {
    str.chars().any(|v| v.is_alphabetic())
}

fn is_question(str: &str) -> bool {
    str.ends_with('?')
}

fn is_yelling(str: &str) -> bool {
    has_letter(str) && str == str.to_uppercase()
}
