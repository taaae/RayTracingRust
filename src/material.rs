use crate::{
    number_stuff::{random_f64, random_in_unit_sphere, random_unit_vector},
    vec3::{dot, reflect, refract, unit_vector},
    Color, HitRecord, Ray,
};

pub trait Material {
    fn project_ray(&self, ray: &mut Ray, record: &HitRecord) -> Color;
}

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
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
            panic!(
                "fuzziness should be in [0.0, 1.0] range but was given: {}",
                fuzziness
            );
        }
        Self { color, fuzziness }
    }
}

impl Material for Metal {
    fn project_ray(&self, ray: &mut Ray, record: &HitRecord) -> Color {
        *ray = Ray::new(
            record.point,
            reflect(&ray.direction(), &record.normal) + self.fuzziness * random_in_unit_sphere(),
        );
        self.color
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn project_ray(&self, ray: &mut Ray, record: &HitRecord) -> Color {
        let refraction_ratio = match record.outside_ray {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };
        let unit_direction = unit_vector(ray.direction());
        let cos_theta = dot(-unit_direction, record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let schlick_approximation =
            Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64();
        let direction = match cannot_refract || schlick_approximation {
            true => reflect(&unit_direction, &record.normal),
            false => refract(&unit_direction, &record.normal, refraction_ratio),
        };
        *ray = Ray::new(record.point, direction);
        Color::new(1.0, 1.0, 1.0) // white
    }
}
