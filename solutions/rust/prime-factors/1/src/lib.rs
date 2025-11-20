pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut factor = 2;

    while num > 1 {
        while n.is_multiple_of(factor) {
            factors.push(factor);
            num /= factor;
        }
        factor += 1;
        if factor.pow(2) > n && n > 1 {
            factors.push(factor);
            break;
        }
    }

    factors
}
