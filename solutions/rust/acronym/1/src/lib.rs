pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = Vec::new();

    let mut initial_here = true;
    let mut last_char_uppercase = false;

    for char in phrase.chars() {
        if char == ' ' || char == '-' || char == '_' {
            initial_here = true;
            continue;
        }
        match (last_char_uppercase, initial_here, char.is_uppercase()) {
            (_, true, _) => {
                acronym.push(char.to_uppercase().to_string());
                last_char_uppercase = true;
                initial_here = false;
            }
            (true, false, true) => continue,
            (false, false, true) => {
                acronym.push(char.to_string());
            }
            (_, _, _) => last_char_uppercase = false,
        }
    }

    acronym.join("")
}
