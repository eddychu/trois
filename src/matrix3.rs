use std::ops::Mul;

use crate::matrix4::Matrix4;
use crate::vector3::Vector3;

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub struct Matrix3 {
    pub m: [f32; 9],
}


impl Matrix3 {
    pub fn new() -> Matrix3 {
        Matrix3 { m: [0.0; 9] }
    }
    pub fn identity() -> Matrix3 {
        Matrix3 {
            m: [
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_mat4(mat: Matrix4) -> Matrix3 {
        Matrix3 {
            m: [
                mat.m[0], mat.m[1], mat.m[2], mat.m[4], mat.m[5], mat.m[6], mat.m[8], mat.m[9], mat.m[10],
            ],
        }
    }

    pub fn row(&self, i : usize) -> Vector3 {
        Vector3 {
            x: self.m[i * 3],
            y: self.m[i * 3 + 1],
            z: self.m[i * 3 + 2],
        }
    }
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.row(0).dot(other),
            self.row(1).dot(other),
            self.row(2).dot(other),
        )
    }
}