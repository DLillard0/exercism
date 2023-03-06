use  std::mem::swap;

pub fn translate(input: &str) -> String {
    input.split_whitespace().map(|str| {
        let mut str1 = String::new();
        let mut str2 = String::new();
        let mut flag = false;
        let mut iter = str.chars();
        match iter.next() {
            None => (),
            Some(c) => {
                match c {
                    c if is_vowel(c) => {
                        str2.push_str(input);
                    },
                    c => {
                        str1.push(c);
                        iter.for_each(|i| {
                            if flag {
                                str2.push(i);
                            } else if is_vowel(i) || i == 'y' {
                                str2.push(i);
                                flag = true;
                            } else {
                                str1.push(i);
                            }
                        });
                    }
                }
            }
        }
        println!("{}, {}", str1, str2);
        if str1.ends_with("q") && str2.starts_with("u") {
            str1.push(str2.remove(0));
        }
        if str1.starts_with("xr") || str1.starts_with("yt") {
            swap(&mut str1, &mut str2);
        }
        str2.push_str(str1.as_str());
        str2.push_str("ay");
        str2
    }).collect::<Vec<String>>().join(" ")
}

fn is_vowel(char: char) -> bool {
    match char {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}
