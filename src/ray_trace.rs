use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;

pub fn hit_sphere(s: &Sphere, r: &Ray) -> Option<f64> {
    let oc = r.origin() - s.center();
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(oc, r.direction());
    let c = dot(oc, oc) - s.radius() * s.radius();
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    Some((-b - discriminant.sqrt()) / (2.0 * a))
}
