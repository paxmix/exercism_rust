pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || len > digits.len() {
        return Vec::new();
    }
    let mut series = Vec::with_capacity(digits.len() - len + 1);

    for i in 0..=digits.len() - len {
        series.push(digits[i..(i + len)].to_string());
    }

    series
}
