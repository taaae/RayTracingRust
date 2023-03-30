use crate::number_stuff::degrees_to_radians;
use crate::number_stuff::random_in_unit_disk;
use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    vertical_fov_degrees: f64,
    aspect_ratio: f64,
    viewpoint_height: f64,
    viewpoint_width: f64,
    focal_length: f64,
    look_from: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    samples_per_pixel: u32,
    look_at: Point3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        vertical_fov_degrees: f64,
        aspect_ratio: f64,
        focal_length: f64,
        look_from: Point3,
        samples_per_pixel: u32,
        look_at: Point3,
        vec_up: Vec3,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov_degrees);
        let h = (theta / 2.0).tan();
        let viewpoint_height = 2.0 * h;
        let viewpoint_width = viewpoint_height * aspect_ratio;
        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(vec_up, w));
        let v = cross(w, u);
        let horizontal = viewpoint_width * u * focus_dist;
        let vertical = viewpoint_height * v * focus_dist;

        Self {
            vertical_fov_degrees,
            aspect_ratio,
            viewpoint_height,
            viewpoint_width,
            focal_length,
            look_from,
            samples_per_pixel,
            horizontal,
            vertical,
            lower_left_corner: look_from - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
            look_at,
            lens_radius: aperture / 2.0,
            u,
            v,
        }
    }

    /// returns a ray that goes from camera origin to (u , v) point. When (u, v) = (0.0, 0.0) end of the ray
    /// will be lower_left_corner. When (u, v) = (1.0, 1.0), end of the ray will be higher  up corner.
    pub fn get_ray(&self, s: f64, x: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.look_from + offset,
            self.lower_left_corner + s * self.horizontal + x * self.vertical
                - self.look_from
                - offset,
        )
    }

    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }
}
