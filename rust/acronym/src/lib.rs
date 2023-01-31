pub fn abbreviate(phrase: &str) -> String {
    let mut res = String::new();
    phrase.chars().fold(' ', |prev, curr| {
        match prev {
            '\'' => (),
            'A'..='Z' => (),
            'a'..='z' => {
                match curr {
                    'A'..='Z' => res.push(curr),
                    _ => ()
                }
            },
            _ if curr.is_alphabetic() => res.push(curr),
            _ => ()
        }
        curr
    });
    res.to_uppercase()
}
