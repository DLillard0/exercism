pub fn encode(source: &str) -> String {
    let mut num = 0;
    let mut char = '0';
    source
        .chars()
        .chain("9".chars())
        .map(|c| {
            if c == char || char == '0' {
                num += 1;
                char = c;
                String::new()
            } else {
                let str = if num == 1 {
                    String::from(char)
                } else {
                    String::from(format!("{}{}", num, char))
                };
                num = 1;
                char = c;
                str
            }
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    let mut num = String::new();
    source
        .chars()
        .map(|c| match c {
            '0'..='9' => {
                num.push(c);
                String::new()
            }
            c => {
                if num.is_empty() {
                    c.to_string()
                } else {
                    let str: String = std::iter::repeat(c)
                        .take(num.parse::<usize>().unwrap())
                        .collect();
                    num.clear();
                    str
                }
            }
        })
        .collect()
}
