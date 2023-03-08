pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut vec: Vec<bool> = std::iter::repeat(false)
        .take(upper_bound as usize)
        .collect();
    (2..=upper_bound / 2).for_each(|n| {
        if vec[n as usize - 1] {
            return;
        }
        let mut m = n;
        m += n;
        while m <= upper_bound {
            vec[m as usize - 1] = true;
            m += n;
        }
    });
    vec.into_iter()
        .enumerate()
        .filter(|(i, b)| *i != 0 && !b)
        .map(|(i, _)| i as u64 + 1)
        .collect()
}
