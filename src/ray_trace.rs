use crate::ray::*;
use crate::vec3::*;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    // None if not hit
    // normal should face the direction of r
    // if angle(r, normal) = pi/2 then normal points outwards
}

pub fn face_one_direction(r: &Ray, v: &Vec3) -> bool {
    // if pi/2 then false
    dot(r.direction(), *v) > 0.0
}
