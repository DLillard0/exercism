/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut vec = Vec::new();
    isbn.chars().for_each(|c| {
        match c {
            '0'..='9' => vec.insert(0, c.to_digit(10).unwrap()),
            'X' if vec.len() == 9 => vec.insert(0, 10),
            _ => ()
        };
    });
    if vec.len() != 10 {
        false
    } else {
        vec.into_iter()
            .enumerate()
            .map(|(i, num)| {
                num * (i as u32 + 1)
            })
            .sum::<u32>()
            % 11 == 0
    }
}
