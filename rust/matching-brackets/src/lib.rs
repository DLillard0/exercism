use std::collections::HashMap;

const VALUE: [char; 3] = ['[', '{', '('];
const KEY: [char; 3] = [']', '}', ')'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let map: HashMap<char, char> = KEY.into_iter().zip(VALUE.into_iter()).collect();
    let mut vec: Vec<char> = vec![];

    string.chars().all(|char| {
        match char {
            c if VALUE.contains(&c) => {
                vec.push(c);
                true
            },
            c if KEY.contains(&c) => {
                match vec.pop() {
                    Some(v) => Some(&v) == map.get(&c),
                    None => false
                }
            },
            _ => true
        }
    }) && vec.is_empty()
}
