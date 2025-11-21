pub fn reply(message: &str) -> &str {
    let trim_msg = message.trim();

    match (
        is_shouting(trim_msg),
        trim_msg.ends_with("?"),
        trim_msg.is_empty(),
    ) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Whoa, chill out!",
        (false, true, _) => "Sure.",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

fn is_shouting(s: &str) -> bool {
    let mut has_alpha = false;
    let mut all_upper = true;

    for c in s.chars() {
        if c.is_alphabetic() {
            has_alpha = true;
            if !c.is_uppercase() {
                all_upper = false;
                break;
            }
        }
    }

    has_alpha && all_upper
}
