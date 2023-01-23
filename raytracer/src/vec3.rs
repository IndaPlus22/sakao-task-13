// use rand::prelude::*;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            x, y, z
        }
    }

    pub fn scale(vec: Vec3, scalar: f64) -> Vec3 {
        PVector::new(vec.x * scalar, vec.y * scalar, vec.z * scalar)
    }

    pub fn div(vec: PVector, scalar: f64) -> PVector {
        PVector::new(vec.x / scalar, vec.y / scalar, vec.z / scalar)
    }

    pub fn magnitude(&mut self) -> f64 {
        f64::sqrt(self.x*self.x + self.y*self.y + self.z*self.z)
    }

    pub fn normalize(&mut self) -> PVector {
        Self::div(PVector::new(self.x, self.y, self.z), self.magnitude())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}