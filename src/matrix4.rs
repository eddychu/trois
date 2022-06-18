use crate::vector3::Vector3;
use crate::vector4::Vector4;
use std::ops::Mul;
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub struct Matrix4 {
    m: [f32; 16],
}

impl Matrix4 {
    pub fn new() -> Matrix4 {
        Matrix4 { m: [0.0; 16] }
    }

    pub fn identity() -> Matrix4 {
        Matrix4 {
            m: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_array(array: [f32; 16]) -> Matrix4 {
        Matrix4 { m: array.clone() }
    }

    pub fn from_columns(c1: &Vector4, c2: &Vector4, c3: &Vector4, c4: &Vector4) -> Matrix4 {
        Matrix4 {
            m: [
                c1.x, c1.y, c1.z, c1.w, c2.x, c2.y, c2.z, c2.w, c3.x, c3.y, c3.z, c3.w, c4.x, c4.y,
                c4.z, c4.w,
            ],
        }
    }

    pub fn from_rows(r1: &Vector4, r2: &Vector4, r3: &Vector4, r4: &Vector4) -> Matrix4 {
        Matrix4 {
            m: [
                r1.x, r2.x, r3.x, r4.x, r1.y, r2.y, r3.y, r4.y, r1.z, r2.z, r3.z, r4.z, r1.w, r2.w,
                r3.w, r4.w,
            ],
        }
    }

    pub fn from_translation(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4 {
            m: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
            ],
        }
    }

    pub fn from_scaling(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4 {
            m: [
                x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_rotation(x: f32, y: f32, z: f32) -> Matrix4 {
        let x_rad = x.to_radians();
        let y_rad = y.to_radians();
        let z_rad = z.to_radians();
        let sx = x_rad.sin();
        let sy = y_rad.sin();
        let sz = z_rad.sin();
        let cx = x_rad.cos();
        let cy = y_rad.cos();
        let cz = z_rad.cos();
        Matrix4 {
            m: [
                cy * cz,
                sx * sy * cz + cx * sz,
                -cx * sy * cz + sx * sz,
                0.0,
                -cy * sz,
                -sx * sy * sz + cx * cz,
                cx * sy * sz + sx * cz,
                0.0,
                sy,
                -sx * cy,
                cx * cy,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        }
    }

    pub fn look_at(eye: &Vector3, target: &Vector3, up: &Vector3) -> Matrix4 {
        let f = (target.clone() - eye.clone()).normalize() * -1.0f32;
        let r = (*up).cross(&f).normalize();
        let u = f.cross(&r).normalize();
        let t = Vector3::new(-r.dot(&*eye), -u.dot(&*eye), -f.dot(&*eye));
        Matrix4 {
            m: [
                r.x, u.x, f.x, 0.0, r.y, u.y, f.y, 0.0, r.z, u.z, f.z, 0.0, t.x, t.y, t.z, 1.0,
            ],
        }
    }

    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4 {
        Matrix4 {
            m: [
                2.0 * near / (right - left),
                0.0,
                0.0,
                0.0,
                0.0,
                2.0 * near / (top - bottom),
                0.0,
                0.0,
                (right + left) / (right - left),
                (top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                -1.0,
                0.0,
                0.0,
                -2.0 * far * near / (far - near),
                0.0,
            ],
        }
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
        let ymax = near * f32::tan(fov * 0.5);
        let xmax = ymax * aspect;
        return Matrix4::frustum(-xmax, xmax, -ymax, ymax, near, far);
    }

    pub fn transpose(&self) -> Matrix4 {
        Matrix4::from_rows(
            &Vector4::new(self.m[0], self.m[1], self.m[2], self.m[3]),
            &Vector4::new(self.m[4], self.m[5], self.m[6], self.m[7]),
            &Vector4::new(self.m[8], self.m[9], self.m[10], self.m[11]),
            &Vector4::new(self.m[12], self.m[13], self.m[14], self.m[15]),
        )
    }

    pub fn row(&self, i: usize) -> Vector4 {
        Vector4::new(
            self.m[0 * 4 + i],
            self.m[1 * 4 + i],
            self.m[2 * 4 + i],
            self.m[3 * 4 + i],
        )
    }

    pub fn col(&self, i: usize) -> Vector4 {
        Vector4::new(
            self.m[i * 4],
            self.m[i * 4 + 1],
            self.m[i * 4 + 2],
            self.m[i * 4 + 3],
        )
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut result = Matrix4::new();
        Matrix4 {
            m: [
                self.row(0).dot(&other.col(0)),
                self.row(1).dot(&other.col(0)),
                self.row(2).dot(&other.col(0)),
                self.row(3).dot(&other.col(0)),
                self.row(0).dot(&other.col(1)),
                self.row(1).dot(&other.col(1)),
                self.row(2).dot(&other.col(1)),
                self.row(3).dot(&other.col(1)),
                self.row(0).dot(&other.col(2)),
                self.row(1).dot(&other.col(2)),
                self.row(2).dot(&other.col(2)),
                self.row(3).dot(&other.col(2)),
                self.row(0).dot(&other.col(3)),
                self.row(1).dot(&other.col(3)),
                self.row(2).dot(&other.col(3)),
                self.row(3).dot(&other.col(3)),
            ],
        }
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;
    fn mul(self, other: Vector4) -> Vector4 {
        Vector4::new(
            self.row(0).dot(&other),
            self.row(1).dot(&other),
            self.row(2).dot(&other),
            self.row(3).dot(&other),
        )
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_vector() {
        let m = Matrix4::from_translation(1.0, 2.0, 3.0);
        let v = Vector4::new(1.0, 2.0, 3.0, 1.0);
        let v2 = m * v;
        assert_eq!(v2, Vector4::new(2.0, 4.0, 6.0, 1.0));
    }
}
