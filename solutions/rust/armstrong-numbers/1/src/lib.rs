pub fn is_armstrong_number(num: u32) -> bool {
        let digits = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let armstrong: u32 = digits.iter().map(|&n| n.pow(digits.len() as u32)).sum();

    num == armstrong

}
