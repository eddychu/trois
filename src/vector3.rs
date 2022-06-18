use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vector3 {
        let length = self.length();
        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn to_u32(&self) -> u32 {
        let r = (self.x * 255.0) as u32;
        let g = (self.y * 255.0) as u32;
        let b = (self.z * 255.0) as u32;
        0xFF000000 | (r << 16) | (g << 8) | b
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
