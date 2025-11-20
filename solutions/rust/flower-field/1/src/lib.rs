pub fn annotate(garden: &[&str]) -> Vec<String> {
    let h = garden.len();
    if h == 0 {
        return vec![];
    }
    let w = garden[0].len();

    let mut result = vec![String::new(); h];

    for y in 0..h {
        for x in 0..w {
            let cell = garden[y].as_bytes()[x] as char;

            if cell == '*' {
                result[y].push('*');
            } else {
                let mut count = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }

                        let nx = x as isize + dx;
                        let ny: isize = y as isize + dy;
                        if nx >= 0
                            && nx < w as isize
                            && ny >= 0
                            && ny < h as isize
                            && garden[ny as usize].as_bytes()[nx as usize] as char == '*'
                        {
                            count += 1;
                        }
                    }
                }

                if count == 0 {
                    result[y].push(' ');
                } else {
                    result[y].push(char::from_digit(count, 10).unwrap());
                }
            }
        }
    }

    result
}
