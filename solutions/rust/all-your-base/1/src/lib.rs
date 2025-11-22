///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    if let Some(d) = number.iter().find(|&&d| d >= from_base) {
        return Err(Error::InvalidDigit(*d));
    }

    if number.iter().all(|&n| n == 0) {
        return Ok(vec![0]);
    }

    Ok(from_decimal(to_decimal(number, from_base), to_base))
}

fn to_decimal(number: &[u32], from_base: u32) -> u32 {
    let mut decimal = 0;

    for digit in number {
        decimal = decimal * from_base + digit;
    }

    decimal
}

fn from_decimal(mut number: u32, to_base: u32) -> Vec<u32> {
    let mut result = Vec::new();

    while number > 0 {
        result.push(number % to_base);
        number /= to_base;
    }

    result.reverse();
    result
}
