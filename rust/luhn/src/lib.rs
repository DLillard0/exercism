/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut even = false;
    let mut count = 0;
    let flag = code.chars().rev().all(|v| {
        match v {
            '0'..='9' => {
                let mut num = v.to_digit(10).unwrap();
                if even {
                    num *= 2;
                    if num > 9 {
                        num -= 9;
                    }
                }
                even = !even;
                count += 1;
                sum += num;
                true
            },
            ' ' => true,
            _ => false,
        }
    });
    flag && count > 1 && sum % 10 == 0
}
