use crate::ray::*;
use crate::ray_trace::*;
use crate::vec3::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        assert!(t_max >= t_min);
        let oc = r.origin() - self.center();
        let a = dot(r.direction(), r.direction());
        let half_b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius() * self.radius();
        let discriminant_quarter = half_b * half_b - a * c;

        if discriminant_quarter < 0.0 {
            return None;
        }
        let sqrtd = discriminant_quarter.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        //let root2 = (-half_b + sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        Some(HitRecord {
            point: r.at(root),
            normal: (r.at(root) - self.center()) / self.radius(),
            t: root,
        })
    }
}

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
