pub fn nth(n: u32) -> u32 {
    let mut i = 0;
    let mut r = 2;
    let mut v = 2;
    while i != n {
        v += 1;
        let limit = (v as f32).sqrt().floor() as u32;
        if limit > 1 && (2 .. limit+1).any(|x| v % x == 0) {
            continue;
        } else {
            i += 1;
            r = v;
        }
    }
    r
}
