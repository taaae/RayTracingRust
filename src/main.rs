#![warn(rust_2018_idioms)]

/*
Constants to increase/decrease quality:
image_width_pixels
samples_per_pixel (in cam)
max_depth
*/

use crate::ray::*;
use crate::ray_trace::*;
use crate::sphere::*;
use crate::vec3::*;
use camera::*;
use number_stuff::*;
use std::rc::Rc;

pub mod camera;
pub mod number_stuff;
pub mod ray;
pub mod ray_trace;
pub mod sphere;
pub mod vec3;

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: u16) -> Color {
    if depth == 0 {
        return Color::default();
    }
    match world.hit(r, 0.00001, f64::MAX) {
        Some(record) => {
            let reflect_point = record.point + record.normal + random_unit_vector();
            0.5 * ray_color(
                &Ray::new(record.point, reflect_point - record.point),
                world,
                depth - 1,
            )
        }
        None => {
            let unit_direction = unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let mut to_write = String::new();
    // Image

    let cam = Camera::new(16.0 / 9.0, 2.0, 1.0, Point3::new(0.0, 0.0, 0.0), 100);

    let max_depth = 50;
    let image_width_pixels = 400;
    let image_height_pixels = (f64::from(image_width_pixels) / cam.aspect_ratio()).round() as i32;

    // World
    let mut world = HittableList::new(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    to_write.push_str(&format!(
        "P3\n{} {}\n255\n",
        image_width_pixels, image_height_pixels
    ));
    for i in (0..image_height_pixels).rev() {
        eprintln!("Lines left: {}", i + 1);
        for j in 0..image_width_pixels {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..cam.samples_per_pixel() {
                let u = (j as f64 + random_f64()) / (image_width_pixels as f64 - 1.0);
                let v = (i as f64 + random_f64()) / (image_height_pixels as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            to_write.push_str(&generate_color(pixel_color, cam.samples_per_pixel()));
        }
    }
    println!("{}", to_write);
    eprintln!("Done!");
}
