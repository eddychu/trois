use crate::{transform::Transform, vector3::Vector3};

#[derive(Clone, Debug, Copy)]
pub struct Light {
    pub ambient: Vector3,
    pub intensity: Vector3,
    pub transform: Transform,
} 

impl Light {
    pub fn new(ambient: Vector3, intensity: Vector3, transform: Transform) -> Light {
        Light {
            ambient,
            intensity,
            transform,
        }
    }
}