use std::cmp::Ordering::{Equal, Less, Greater};

pub fn find<T: Ord, R: AsRef<[T]>>(array: R, key: T) -> Option<usize> {
    let arr = array.as_ref();
    if arr.len() == 0 {
        return None;
    }
    let mut start = 0;
    let mut end = arr.len();
    while end > start {
        let mid = start + (end - start) / 2;
        match arr[mid].cmp(&key) {
            Equal => return Some(mid),
            Greater => end = mid,
            Less => start = mid + 1
        }
    }
    None
}
