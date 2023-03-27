#![warn(rust_2018_idioms)]

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

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    match world.hit(r, 0.0, f64::MAX) {
        Some(record) => 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0)),
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

    // Camera
    /*let viewpoint_height = 2.0;
    let viewpoint_width = aspect_ratio * viewpoint_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewpoint_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewpoint_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);*/

    let cam = Camera::new(16.0 / 9.0, 2.0, 1.0, Point3::new(0.0, 0.0, 0.0), 30);

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
        eprintln!("Lines left: {}", i);
        for j in 0..image_width_pixels {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..cam.samples_per_pixel() {
                let u = (j as f64 + random_f64()) / (image_width_pixels as f64 - 1.0);
                let v = (i as f64 + random_f64()) / (image_height_pixels as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            to_write.push_str(&generate_color(pixel_color, cam.samples_per_pixel()));
        }
    }
    println!("{}", to_write);
    eprintln!("Done!");
}
