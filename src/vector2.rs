use std::ops::{Add, Mul};
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn dot(&self, other: Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn cross(&self, other: Vector2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl Mul for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
