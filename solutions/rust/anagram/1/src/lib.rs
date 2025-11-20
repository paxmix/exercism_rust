use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_sorted = sort_string(&word);

    possible_anagrams
        .iter()
        .filter(|&anagram|{
            let anagram = anagram.to_lowercase();
            let anagram_sorted = sort_string(&anagram);

            anagram != word && anagram_sorted == word_sorted
        })
        .cloned()
        .collect()
}

fn sort_string(s:&str)->String{
    let mut chars:Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
