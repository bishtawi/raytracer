pub fn float_to_int_truncate(num: f64) -> i32 {
    #[allow(clippy::cast_possible_truncation)] // Truncation is fine
    let integer = num as i32;
    integer
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// Range [0, 1)
pub fn random_float() -> f64 {
    rand::random::<f64>()
}

// Range [min,max)
pub fn random_float_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_float()
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
