use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// Not my answer, i copied this from iamhere2 and tried to understand the algorithm

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Filter all leading letter to laters ensure its factor != 0
    let first = input
        .split(&['+', '='])
        .filter_map(|s| s.trim().chars().next())
        .collect::<HashSet<_>>();

    let (letters, factors) = calc_factors(input);

    // Loop all possible permutations after having extracted all letters
    for perm in (0..=9).permutations(letters.len()) {
        // use the permutation vec, apply the factors accordingly to the
        // letters to calculate sum
        let sum = perm
            .iter()
            .enumerate()
            .map(|(i, v)| v * factors.get(i).unwrap())
            .sum::<i64>();
        // if the sum == 0 (satisfied the linear equation) and the value of
        // all leading letters != 0 , return the perm as a HashMap
        if sum == 0
            && !perm.iter().enumerate().any(|(i, v)| {
                *v == 0 && first.contains(letters.get(i).unwrap())
            })
        {
            return Some(HashMap::from_iter(
                perm.iter()
                    .enumerate()
                    .map(|(i, v)| (*letters.get(i).unwrap(), *v as u8)),
            ));
        }
    }

    // If all permutations failed return None
    None
}

// consider the input into linear equation: SEND + MORE - MONEY = 0
// simplify it into factors * letters:
// -9000*M + 1000*S - 900*O + 91*E - 90*N + 10*R - 1*Y + 1*D = 0
fn calc_factors(input: &str) -> (Vec<char>, Vec<i64>) {
    let mut factors = HashMap::new();
    let mut sign = -1;
    let mut pos = 0;
    // iterate in reverse to easily count factors of each position:
    // SEND = D*10^0 + N*10^1 + E*10^2 + S*10^3
    // so sign need to begin as negative, turn to + after passing "="
    // position count reset after the "+"
    for c in input.chars().filter(|c| !c.is_whitespace()).rev() {
        match c {
            '=' => {
                sign = 1;
                pos = 0
            }
            '+' => pos = 0,
            _ => {
                *factors.entry(c).or_insert(0) += sign * 10_i64.pow(pos);
                pos += 1
            }
        }
    }

    // return 2 vec, 1 vec of letters and 1 vec of their factors with the
    // accordingly index
    factors.into_iter().sorted_by_key(|(_, v)| -v.abs()).unzip()
    // According to the author, sorting by letter by their factors absolute
    // coefficient do affect performance for the permutation loop above
    // I imagine the reason is that the letter has biggest factors
    // would usually have a very low value, so the loop usually won't lead to
    // M in the example to go way up to 8-9, while the smallest like Y and D easily can
}
