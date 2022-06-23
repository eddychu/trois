use crate::vector3::Vector3;

pub struct Material {
    pub basecolor_map : u32,
    pub metallic_map : u32,
    pub emission_map : u32,
    pub roughness_map : u32,
    pub normal_map : u32,
    pub occlusion_map : u32,
}