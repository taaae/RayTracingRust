#![warn(rust_2018_idioms)]

/*
Constants to increase/decrease quality:
image_width_pixels
samples_per_pixel (in cam)
max_depth
*/

use crate::ray::*;
use crate::ray_trace::*;
use crate::vec3::*;
use camera::*;
use generate_scene::generate_scene;
use number_stuff::*;

pub mod camera;
pub mod generate_scene;
pub mod material;
pub mod number_stuff;
pub mod ray;
pub mod ray_trace;
pub mod sphere;
pub mod vec3;

fn ray_color<T: Hittable>(ray: &mut Ray, world: &T, depth: u16) -> Color {
    if depth == 0 {
        return Color::default();
    }
    match world.hit(ray, 0.00001, f64::MAX) {
        Some(record) => {
            // TODO: add class for reflection method
            // let reflect_point = record.point + record.normal + random_unit_vector();
            let color = record
                .material_reference
                .clone()
                .expect("no idea why empty world works too")
                .project_ray(ray, &record);
            color * ray_color(ray, world, depth - 1)
        }
        None => {
            let unit_direction = unit_vector(ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let mut to_write = String::new();

    let samples_per_pixel = 500;
    let max_depth = 50;
    let image_width_pixels = 1200;

    // Image
    let cam = Camera::new(
        30.0,
        3.0 / 2.0,
        1.0,
        Point3::new(13.0, 2.0, 3.0),
        samples_per_pixel,
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.1,
        10.0,
    );

    let image_height_pixels = (f64::from(image_width_pixels) / cam.aspect_ratio()).round() as i32;

    // World
    let world = generate_scene();

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
                let mut r = cam.get_ray(u, v);
                pixel_color += ray_color(&mut r, &world, max_depth);
            }
            to_write.push_str(&generate_color(pixel_color, cam.samples_per_pixel()));
        }
    }
    println!("{}", to_write);
    eprintln!("Done!");
}
