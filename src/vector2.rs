use std::ops::{Add, Mul};
use std::cmp::{PartialEq};
use crate::math::K_EPSILON;
#[derive(Clone, Debug, Copy)]
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


impl PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        (self.x - other.x).abs() < K_EPSILON && (self.y - other.y).abs() < K_EPSILON
    }
}