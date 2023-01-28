
pub fn factors(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut num = n;
    while num != 1 {
        let prime = (2 .. num + 1).find(|v| num % v == 0).unwrap();
        res.push(prime);
        num /= prime; 
    }
    res
}
