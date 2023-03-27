use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
// use std::io::Write;
use num::clamp;
use std::ops::{Div, DivAssign, Mul, MulAssign};

// Note: intentionally used Self and not &Self in operations (cuz i'm too lazy)
#[derive(Add, Sub, Neg, AddAssign, SubAssign, Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

// fn cross(u: Vec3, v: Vec3) -> Vec3 {
//     Vec3 {
//         x: u.y() * v.z() - u.z() * v.y(),
//         y: u.z() * v.x() - u.x() * v.z(),
//         z: u.x() * v.y() - u.y() * v.x(),
//     }
// }

pub fn unit_vector(u: Vec3) -> Vec3 {
    u / u.length()
}

// Note: considered to ignore [] and * (for two vectors) operators

pub type Point3 = Vec3;
pub type Color = Vec3;

// Note: idk how streams work in rust so just implemented simple print

// pub fn write_color(stdout: &mut std::io::Stdout, pixel_color: Color) {
//     let r = (255.999 * pixel_color.x()) as i32;
//     let g = (255.999 * pixel_color.y()) as i32;
//     let b = (255.999 * pixel_color.z()) as i32;
//     writeln!(stdout, "{} {} {}", r, g, b).unwrap();
// }

pub fn generate_color(pixel_color: Color, samples_per_pixel: u32) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let final_r = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let final_g = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let final_b = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    format!("{} {} {}\n", final_r, final_g, final_b)
}

//////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let u = Point3::new(1.0, 2.0, 3.0);
        let v = Point3::new(3.0, 6.0, 9.0);
        assert_eq!(u + v, Point3::new(4.0, 8.0, 12.0));
        assert_eq!(v - u, Point3::new(2.0, 4.0, 6.0));
        assert_eq!(u * 10.0, Point3::new(10.0, 20.0, 30.0));
        assert_eq!(v / 3.0, Point3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn assign() {
        let mut u = Point3::new(1.0, 2.0, 3.0);
        let v = Point3::new(1.0, 1.0, 1.0);
        u += v;
        assert_eq!(u, Point3::new(2.0, 3.0, 4.0));
        u -= v * 2.0;
        assert_eq!(u, Point3::new(0.0, 1.0, 2.0));
        u *= 6.0;
        assert_eq!(u, Point3::new(0.0, 6.0, 12.0));
        u /= 3.0;
        assert_eq!(u, Point3::new(0.0, 2.0, 4.0));
    }

    #[test]
    fn cross_dot_unit_length() {
        let u = Point3::new(1.0, 2.0, 3.0);
        let v = Point3::new(-1.0, 3.0, 6.0);
        // assert_eq!(cross(u, v), Point3::new(3.0, -9.0, 5.0));
        assert_eq!(dot(u, v), 23.0);
        assert_eq!(
            unit_vector(u),
            Vec3::new(
                1.0 / (14.0_f64.sqrt()),
                (2.0_f64 / 7.0).sqrt(),
                3.0 / 14_f64.sqrt()
            )
        );
        assert_eq!(u.length(), 14.0_f64.sqrt());
    }
}
