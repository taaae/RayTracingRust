use crate::vec3::*;
use rand::Rng;
use std::f64::consts::PI;

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

// TODO: Implement this with rand crate traits
pub fn random_vec() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
}

pub fn random_vec_in_range(min: f64, max: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_sphere() -> Point3 {
    loop {
        let v = random_vec_in_range(-1.0, 1.0);
        if v.length_squared() >= 1.0 {
            continue;
        }
        break v;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}
