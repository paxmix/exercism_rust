pub fn nth(n: u32) -> u32 {
    let mut prime_vec: Vec<u32> = vec![2, 3];
    let mut num: u32 = 4;

    while prime_vec.len() < n as usize + 1 {
        let sqrt_num = num.isqrt();
        let mut count = 0;

        for p in &prime_vec {
            if p <= &sqrt_num && num.is_multiple_of(*p) {
                count += 1;
                break;
            }
        }
        if count == 0 {
            prime_vec.push(num);
        }
        num += 1;
    }

    prime_vec[n as usize]
}
