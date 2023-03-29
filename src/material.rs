use crate::{Ray, HitRecord, Color, number_stuff::{random_unit_vector, random_in_unit_sphere}, vec3::reflect};

pub trait Material {
    fn project_ray(&self, ray: &mut Ray, record: &HitRecord) -> Color;
}

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {Self{color}}
}

impl Material for Lambertian {
    fn project_ray<'a>(&self, ray: &mut Ray, record: &HitRecord) -> Color {
        let mut projected_direction = record.normal + random_unit_vector();
        if projected_direction.really_small() {
            projected_direction = record.normal;
        }
        *ray = Ray::new(record.point, projected_direction);
        self.color
    }
}


pub struct Metal {
    color: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(color: Color, fuzziness: f64) -> Self {
        if !(0.0..=1.0).contains(&fuzziness) {
            panic!("fuzziness should be in [0.0, 1.0] range but was given: {}", fuzziness);
        }
        Self{color, fuzziness}} 
}

impl Material for Metal {
    fn project_ray(&self, ray: &mut Ray, record: &HitRecord) -> Color {
        *ray = Ray::new(record.point, reflect(&ray.direction(), &record.normal) + self.fuzziness * random_in_unit_sphere());
        self.color
    }
}
