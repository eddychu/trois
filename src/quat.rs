use std::f64::EPSILON;
use std::ops::{Add, Sub, Mul, Neg};
use std::cmp::{PartialEq};
use crate::vector3::Vector3;
use crate::matrix4::Matrix4;
use crate::math::K_EPSILON;

#[derive(Clone, Debug, Copy)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quat {
        Quat { x, y, z, w }
    }

    pub fn identity() -> Quat {
        Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn angle_axis(angle: f32, axis: &Vector3) -> Quat {
        let axis = axis.normalize();
        let s = (angle * 0.5).sin();
        Quat {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: (angle * 0.5).cos(),
        }
    }

    pub fn from_to(from: &Vector3, to: &Vector3) -> Quat {
        let f = from.normalize();
        let t = to.normalize();
        if f == t {
            return Quat::identity();
        } else if f == t * (-1.0) {
            let mut ortho = Vector3::new(1.0, 0.0, 0.0);
            if f.y.abs() < f.x.abs() {
                ortho = Vector3::new(0.0, 1.0, 0.0);
            } 
            if f.z.abs() < f.y.abs() && f.z.abs() < f.x.abs() {
                ortho = Vector3::new(0.0, 0.0, 1.0);
            }
            let axis = f.cross(&ortho).normalize();
            return Quat::new(axis.x, axis.y, axis.z, 0.0);
        }
        let half = (f + t).normalize();
        let axis = f.cross(&half);
        return Quat::new(axis.x, axis.y, axis.z, f.dot(&half));
    }

    pub fn look_dir(dir: &Vector3, up: &Vector3) -> Quat {
        let dir = dir.normalize();
        let up = up.normalize();
        let right = dir.cross(&up).normalize();
        let up = dir.cross(&right);
        let from = Vector3::new(0.0, 0.0, 1.0);
        let f2d = Quat::from_to(&from, &dir);
        let object_up = f2d * Vector3::new(0.0, 1.0, 0.0);
        let u2u = Quat::from_to(&object_up, &up);
        let result = f2d * u2u;
        return result.normalize();
    }

    pub fn normalize(&self) -> Quat {
        let len_sq = self.length_squared();
        if len_sq < K_EPSILON {
            return Quat::identity();
        }
        let i_len = 1.0 / len_sq.sqrt();
        Quat {
            x: self.x * i_len,
            y: self.y * i_len,
            z: self.z * i_len,
            w: self.w * i_len,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn get_axis(&self) -> Vector3 {
       Vector3::new(self.x, self.y, self.z).normalize()
    }

    pub fn get_angle(&self) -> f32 {
        2.0 * self.w.acos()
    }

    pub fn to_mat4(&self) -> Matrix4 {
        let r = self.mul(Vector3::new(1.0, 0.0, 0.0));
        let u = self.mul(Vector3::new(0.0, 1.0, 0.0));
        let f = self.mul(Vector3::new(0.0, 0.0, 1.0));
        Matrix4::from_array([
            r.x, r.y, r.z, 0.0,
            u.x, u.y, u.z, 0.0,
            f.x, f.y, f.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

}

impl Add for Quat {
    type Output = Quat;

    fn add(self, other: Quat) -> Quat {
        Quat {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Quat {
    type Output = Quat;

    fn sub(self, other: Quat) -> Quat {
        Quat {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul for Quat {
    type Output = Quat;

    fn mul(self, other: Quat) -> Quat {
        Quat {
            x: other.x * self.w + other.y * self.z - other.z * self.y + other.w * self.x,
		    y: -other.x * self.z + other.y * self.w + other.z * self.x + other.w * self.y,
            z : other.x * self.y - other.y * self.x + other.z * self.w + other.w * self.z,
            w : -other.x * self.x - other.y * self.y - other.z * self.z + other.w * self.w
        }
    }
}

impl Mul<f32> for Quat {
    type Output = Quat;

    fn mul(self, other: f32) -> Quat {
        Quat {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<Vector3> for Quat {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        let vector = Vector3::new(self.x, self.y, self.z);
        vector * 2.0 * vector.dot(&other) + other * (self.w * self.w - vector.dot(&vector)) + vector.cross(&other) * 2.0 * self.w
    }
}

impl Neg for Quat {
    type Output = Quat;

    fn neg(self) -> Quat {
        Quat {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl PartialEq for Quat {
    fn eq(&self, other: &Quat) -> bool {
        (self.x - other.x).abs() < K_EPSILON && (self.y - other.y).abs() < K_EPSILON && (self.z - other.z).abs() < K_EPSILON && (self.w - other.w).abs() < K_EPSILON
    }
}