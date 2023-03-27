use std::f64::consts::PI;
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// TODO: optimize it to not call thread_rng every time (pass a reference to a local thread rng)
pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
