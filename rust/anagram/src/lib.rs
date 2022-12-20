use std::collections::{HashSet, HashMap};

pub fn is_same (word: &str, anagram: &str) -> bool {
    let l_word = word.to_lowercase();
    let l_anagram = anagram.to_lowercase();

    if l_word == l_anagram || l_word.len() != l_anagram.len() {
        return false;
    }

    let mut map: HashMap<_, i32> = HashMap::new();

    l_word
        .chars()
        .for_each(|i| {
            map.entry(i).and_modify(|v| *v += 1).or_insert(1);
        });

    !l_anagram
        .chars()
        .any(|i| {
            let val = map.entry(i).or_insert(0);
            *val -= 1;
            *val < 0
        })
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|x| is_same(word, x))
        .cloned()
        .collect()
}
