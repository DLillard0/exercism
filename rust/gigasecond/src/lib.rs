use time::PrimitiveDateTime as DateTime;
use core::time::Duration;

const GIGASECOND:u32 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let duration = Duration::new(GIGASECOND as u64, 0);
    start + duration
}
