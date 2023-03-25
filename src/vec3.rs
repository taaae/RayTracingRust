use std::ops::{Mul, MulAssign, Div, DivAssign};
use derive_more::{Add, Neg, Sub, AddAssign, SubAssign};

#[derive(Add, Sub, Neg, AddAssign, SubAssign, Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
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

    fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    fn length(&self) -> f64 {
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

fn dot(u: Vec3, v: Vec3) -> f64 {
      u.x() * v.x()
    + u.y() * v.y()
    + u.z() * v.z()
}

fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.y() * v.z() - u.z() * v.y(),
        y: u.z() * v.x() - u.x() * v.z(),
        z: u.x() * v.y() - u.y() * v.x(),
    }
}

fn unit_vector(u: Vec3) -> Vec3 {
    u / u.length()
}

// Note: considered to ignore [] and * (for two vectors) operators

pub type Point3 = Vec3;
pub type Color = Vec3;


// Note: idk what to do with streams so just implemented simple print

pub fn write_color(c: Color) {
    println!("{} {} {}", (255.999 * c.x()).round(), (255.999 * c.y()).round(), (255.99 * c.z()).round());
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
        assert_eq!(cross(u, v), Point3::new(3.0, -9.0, 5.0));
        assert_eq!(dot(u, v), 23.0);
        assert_eq!(unit_vector(u), Vec3::new(1.0/(14.0_f64.sqrt()), (2.0_f64 / 7.0).sqrt(), 3.0 / 14_f64.sqrt()));
        assert_eq!(u.length(), 14.0_f64.sqrt());
    }
}
