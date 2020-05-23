#[derive(Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
pub struct Helices {
    v1: Vec3,
    v2: Vec3,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn norm(self) -> Self {
        let mag = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

use std::iter::Sum;
use std::ops::{Add, Div, Sub};

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Self {
        iter.fold(Vec3::new(0.0, 0.0, 0.0), |acc, vec| acc + vec)
    }
}

impl Helices {
    pub fn new(v1: Vec3, v2: Vec3) -> Self {
        Self {
            v1: v1.norm(),
            v2: v2.norm(),
        }
    }

    pub fn dot(self) -> f64 {
        (self.v1.x * self.v2.x + self.v1.y * self.v2.y + self.v1.z * self.v2.z)
            .acos()
            .to_degrees()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{Helices, Vec3};
    #[test]
    fn test_vec3_new() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3 {
                x: 1f64,
                y: 2f64,
                z: 3f64
            }
        )
    }
    #[test]
    fn test_vec3_subtraction() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) - Vec3::new(0.5, 0.7, 1.2),
            Vec3 {
                x: 0.5,
                y: 1.3,
                z: 1.8
            }
        )
    }

    #[test]
    fn test_vec3_norm() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 12.0).norm(),
            Vec3 {
                x: 3.0 / 13.0,
                y: 4.0 / 13.0,
                z: 12.0 / 13.0
            }
        )
    }

    #[test]
    fn test_hlx_new() {
        let helices = Helices::new(
            Vec3::new(1.0, 2.0, 3.0).norm(),
            Vec3::new(2.0, 3.0, 4.0).norm(),
        );
        assert!((helices.dot() - 6.982_497_287_918_67).abs() < 0.000_000_1)
    }
}
