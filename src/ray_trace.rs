use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;

pub fn hit_sphere(s: &Sphere, r: &Ray) -> Option<f64> {
    let oc = r.origin() - s.center();
    let a = dot(r.direction(), r.direction());
    let half_b = dot(oc, r.direction());
    let c = dot(oc, oc) - s.radius() * s.radius();
    let discriminant_quarter = half_b * half_b - a * c;
    if discriminant_quarter < 0.0 {
        return None;
    }
    Some((-half_b - discriminant_quarter.sqrt()) / a)
}
