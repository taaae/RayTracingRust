use crate::number_stuff::degrees_to_radians;
use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    vertical_fov_degrees: f64,
    aspect_ratio: f64,
    viewpoint_height: f64,
    viewpoint_width: f64,
    focal_length: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    samples_per_pixel: u32,
}

impl Camera {
    pub fn new(
        vertical_fov_degrees: f64,
        aspect_ratio: f64,
        focal_length: f64,
        origin: Point3,
        samples_per_pixel: u32,
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov_degrees);
        let h = (theta/2.0).tan();
        let viewpoint_height = 2.0 * h;
        Self {
            vertical_fov_degrees,
            aspect_ratio,
            viewpoint_height,
            viewpoint_width: viewpoint_height * aspect_ratio,
            focal_length,
            origin,
            horizontal: Vec3::new(viewpoint_height * aspect_ratio, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewpoint_height, 0.0),
            lower_left_corner: origin
                - Vec3::new(viewpoint_height * aspect_ratio, 0.0, 0.0) / 2.0
                - Vec3::new(0.0, viewpoint_height, 0.0) / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
            samples_per_pixel,
        }
    }

    /// returns a ray that goes from camera origin to (u , v) point. When (u, v) = (0.0, 0.0) end of the ray
    /// will be lower_left_corner. When (u, v) = (1.0, 1.0), end of the ray will be higher  up corner.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }
}
