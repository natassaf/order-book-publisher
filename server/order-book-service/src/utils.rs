pub fn round_to(num: f64, num_digits: i32) -> f64 {
    let multiplier: f64 = (10 as i64).pow(num_digits.try_into().unwrap()) as f64;
    ((num * multiplier).round() / multiplier) as f64
}
