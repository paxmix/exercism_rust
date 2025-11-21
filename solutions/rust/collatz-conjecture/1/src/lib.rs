pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut count = 0;
    while n != 1 {
        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        count += 1
    }

    Some(count)
}