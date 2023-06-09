use crate::material::Material;
use crate::ray::*;
use crate::ray_trace::*;
use crate::vec3::*;
use std::rc::Rc;

pub struct Sphere<M: Material + 'static> {
    center: Point3,
    radius: f64,
    material: Rc<M>,
}

impl<M: Material + 'static> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: Rc<M>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl<M: Material + 'static> Hittable for Sphere<M> {
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
        // outward normal here
        let mut normal = (r.at(root) - self.center()) / self.radius();
        let mut outside_ray = true;
        if face_one_direction(r, &normal) {
            normal = -normal;
            outside_ray = false;
        }
        Some(HitRecord {
            point: r.at(root),
            normal,
            t: root,
            material_reference: Some(self.material.clone()),
            outside_ray,
        })
    }
}
