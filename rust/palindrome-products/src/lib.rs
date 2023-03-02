/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

fn is_palindrome(value: u64) -> bool {
    value
        .to_string()
        .chars()
        .zip(value.to_string().chars().rev())
        .all(|(a, b)| a == b)
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    match (min, max) {
        (min, max) if min > max => None,
        (min, max) if min == max => Some(( Palindrome::new(min * min)?, Palindrome::new(min * min)? )),
        (min, max) => {
            let mut min_p = None;
            let mut max_p = None;
            (min..=max).for_each(|m| {
                if m % 10 == 0 { return; }
                (m..=max).for_each(|n| {
                    if n % 10 == 0 { return; }
                    let num = m * n;
                    if is_palindrome(num) {
                        match (min_p, max_p) {
                            (None, None) => {
                                min_p = Some(num);
                                max_p = Some(num);
                            }
                            (Some(min), Some(max)) => {
                                if num < min { min_p = Some(num) }
                                if num > max { max_p = Some(num) }
                            }
                            (_, _) => ()
                        }
                    }
                })
            });
            Some(( Palindrome::new(min_p?)?, Palindrome::new(max_p?)? ))
        }
    }

}
