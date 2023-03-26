use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;

pub mod ray;
pub mod ray_trace;
pub mod sphere;
pub mod vec3;

fn ray_color(r: &Ray) -> Color {
    let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    match hit_sphere(&s, r) {
        Some(t) => {
            let normal = unit_vector(r.at(t) - s.center());
            0.5 * (normal + Color::new(1.0, 1.0, 1.0))
        }
        None => {
            let unit_direction = unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width_pixels = 400;
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
            write_color(&mut std::io::stdout(), ray_color(&r));
        }
    }
    eprintln!("Done!");
}
