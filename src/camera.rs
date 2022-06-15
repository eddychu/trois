use crate::matrix4::Matrix4;
use crate::vector3::Vector3;
use crate::vector4::Vector4;

pub struct Camera {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fov: f64,
    pub aspect_ratio: f64,
    pub near: f64,
    pub far: f64,
}

impl Camera {
    pub fn new(
        position: Vector3,
        target: Vector3,
        up: Vector3,
        fov: f64,
        aspect_ratio: f64,
        near: f64,
        far: f64,
    ) -> Camera {
        Camera {
            position,
            target,
            up,
            fov,
            aspect_ratio,
            near,
            far,
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4 {
        Matrix4::look_at(&self.position, &self.target, &self.up)
    }

    pub fn get_projection_matrix(&self) -> Matrix4 {
        Matrix4::perspective(self.fov, self.aspect_ratio, self.near, self.far)
    }
}
