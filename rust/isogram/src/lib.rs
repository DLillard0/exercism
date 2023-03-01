use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    if candidate.len() == 0 {
        return true;
    }
    let mut set = HashSet::new();
    candidate.to_ascii_lowercase().chars().all(|c| match c {
        'a'..='z' if set.get(&c) != None => false,
        'a'..='z' => {
            set.insert(c);
            true
        }
        _ => true,
    })
}
