#![warn(rust_2018_idioms)]

use crate::ray::*;
use crate::ray_trace::*;
use crate::sphere::*;
use crate::vec3::*;
use std::rc::Rc;

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
    let aspect_ratio = 16.0 / 9.0;
    let image_width_pixels = 10000;
    let image_height_pixels = (f64::from(image_width_pixels) / aspect_ratio).round() as i32;

    // Camera
    let viewpoint_height = 2.0;
    let viewpoint_width = aspect_ratio * viewpoint_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewpoint_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewpoint_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // World
    let mut world = HittableList::new(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    println!("P3\n{} {}\n255", image_width_pixels, image_height_pixels);
    for i in (0..image_height_pixels).rev() {
        eprintln!("Lines left: {}", i);
        for j in 0..image_width_pixels {
            let u = j as f64 / (image_width_pixels as f64 - 1.0);
            let v = i as f64 / (image_height_pixels as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            to_write.push_str(&generate_color(ray_color(&r, &world)));
            //write_color(&mut std::io::stdout(), ray_color(&r, &world));
        }
    }
    println!("{}", to_write);
    eprintln!("Done!");
}
