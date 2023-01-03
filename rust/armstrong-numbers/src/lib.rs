pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let len = str.len();
    num as u64 == str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(len as u32) as u64)
        .sum()
}
