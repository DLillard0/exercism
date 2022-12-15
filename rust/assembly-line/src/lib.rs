// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const ONE_SPEED_PRODUCTION: u32 = 221;

pub fn get_success_rate(speed: u8) -> f32 {
    match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9 | 10 => 0.77,
        _ => 0.0,
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (speed as f64) * (ONE_SPEED_PRODUCTION as f64) * (get_success_rate(speed) as f64)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let one_hour_production = production_rate_per_hour(speed);
    (one_hour_production as u32) / 60
}
