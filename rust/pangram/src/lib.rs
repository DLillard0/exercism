use std::collections::hash_map::RandomState;
use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set: HashSet<char, RandomState> = HashSet::from_iter('a'..='z');
    sentence.to_ascii_lowercase().chars().for_each(|c| {
        set.remove(&c);
    });
    set.is_empty()
}
