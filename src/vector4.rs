use crate::vector3::Vector3;
use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;
use crate::math::K_EPSILON;
#[derive(Clone, Debug, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn dot(&self, rhs: &Vector4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn to_u32(&self) -> u32 {
        let r = (self.x * 255.0) as u32;
        let g = (self.y * 255.0) as u32;
        let b = (self.z * 255.0) as u32;
        let a = (self.w * 255.0) as u32;
        (r << 16) | (g << 8) | b | (a << 24)
    }

    pub fn xyz(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl Mul for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Vector4 {
        Vector4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Vector4 {
        Vector4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Vector4 {
        Vector4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, other: &Vector4) -> bool {
        (self.x - other.x).abs() < K_EPSILON && (self.y - other.y).abs() < K_EPSILON && (self.z - other.z).abs() < K_EPSILON && (self.w - other.w).abs() < K_EPSILON
    }
}