use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;
use std::rc::Rc;

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material_reference: Option<Rc<dyn Material>>,
    pub outside_ray: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    // None if not hit
    // normal should face the direction of r
    // if angle(r, normal) = pi/2 then normal points outwards
    // TODO: maybe need to return if they face one direction later
    // (to determine whether ray hits obj from outside)
}

pub fn face_one_direction(r: &Ray, v: &Vec3) -> bool {
    // if pi/2 then false
    dot(r.direction(), *v) > 0.0
}

#[derive(Default)]
pub struct HittableList {
    // might need to change to Vec<Rc<Cell<T>>>
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }
}

// TODO: refactor this spaghetti
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_record = HitRecord {
            t: t_max,
            ..Default::default()
        };
        let mut hit_something = false;
        self.objects
            .iter()
            // might replace t_max with temp_record.t here to remove if condition later,
            // but borrow checker doesn't let me
            .map(|obj| obj.hit(r, t_min, t_max))
            .for_each(|record_opt| {
                if let Some(record) = record_opt {
                    if record.t <= temp_record.t {
                        temp_record = record;
                        hit_something = true;
                    }
                }
            });
        if hit_something {
            return Some(temp_record);
        }
        None
    }
}
