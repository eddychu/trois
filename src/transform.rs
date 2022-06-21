
use crate::vector3::Vector3;
use crate::matrix4::Matrix4;
use crate::quat::Quat;
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quat,
    pub scale: Vector3,
}

impl Transform {
    pub fn new(position: Vector3, rotation: Quat, scale: Vector3) -> Transform {
        Transform {
            position,
            rotation,
            scale,
        }
    }

    pub fn identity() -> Transform {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quat::identity(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn transform_point(&self, point: Vector3) -> Vector3 {
        self.position + self.rotation * (self.scale * point)
    }

    pub fn transform_vector(&self, vector: Vector3) -> Vector3 {
        self.rotation * (self.scale * vector)
    }

    pub fn to_mat4(&self) -> Matrix4 {
        let mut x = self.rotation * Vector3::new(1.0, 0.0, 0.0);
        let mut y = self.rotation * Vector3::new(0.0, 1.0, 0.0);
        let mut z = self.rotation * Vector3::new(0.0, 0.0, 1.0);
        x = self.scale * x;
        y = self.scale * y;
        z = self.scale * z;

        let p = self.position;
        Matrix4::from_array([
            x.x, x.y, x.z, 0.0,
            y.x, y.y, y.z, 0.0,
            z.x, z.y, z.z, 0.0,
            p.x, p.y, p.z, 1.0,
        ])
    }

    
}