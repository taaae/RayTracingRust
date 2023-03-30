use crate::{
    material::{Dielectric, Lambertian, Metal},
    number_stuff::{random_f64, random_f64_in_range, random_vec, random_vec_in_range},
    ray_trace::HittableList,
    sphere::Sphere,
    vec3::{Color, Point3},
};
use std::rc::Rc;

pub fn generate_scene() -> HittableList {
    let mut world = HittableList::default();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a_int in -11..11 {
        for b_int in -11..11 {
            let a = a_int as f64;
            let b = b_int as f64;
            let choose_mat = random_f64();
            let center = Point3::new(a + 0.9 * random_f64(), 0.2, b + 0.9 * random_f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // shared_ptr<material> sphere_material;
                if choose_mat < 0.8 {
                    // diffuse
                    let color = random_vec() * random_vec();
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(color)),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let color = random_vec_in_range(0.5, 1.0);
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(color, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
