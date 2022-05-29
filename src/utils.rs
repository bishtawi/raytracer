pub fn float_to_int_truncate(num: f64) -> i32 {
    #[allow(clippy::cast_possible_truncation)] // Truncation is fine
    let integer = num as i32;
    integer
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
