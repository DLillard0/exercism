#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match (1..=num / 2).filter(|&n| num % n == 0).sum::<u64>() {
        v if v == num => Some(Classification::Perfect),
        v if v > num => Some(Classification::Abundant),
        v if v < num => Some(Classification::Deficient),
        _ => panic!()
    }
}
