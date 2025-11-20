/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if let Some(digits) = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
    {
        if digits.len() <= 1 {
            return false;
        }

        let checksum: u32 = digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &n)| {
                if i % 2 != 0 {
                    if n * 2 >= 10 { n * 2 - 9 } else { n * 2 }
                } else {
                    n
                }
            })
            .sum();

        checksum.is_multiple_of(10)
    } else {
        false
    }
}
