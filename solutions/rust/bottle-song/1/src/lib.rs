pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut recite_vec: Vec<String> = Vec::new();

    let number = [
        "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    for i in 0..take_down {
        recite_vec.push(format!(
            "{} green {} hanging on the wall,\n{} green {} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.\n",
            number[(start_bottles - i) as usize],
            bottle_from_num(start_bottles - i),
            number[( start_bottles-i ) as usize],
            bottle_from_num(start_bottles - i),
            number[(start_bottles - i - 1) as usize].to_lowercase(),
            bottle_from_num(start_bottles - i - 1)
        ));
    }

    recite_vec.join("\n")
}

fn bottle_from_num(num: u32) -> String {
    if num == 1 {
        "bottle".to_string()
    } else {
        "bottles".to_string()
    }
}
