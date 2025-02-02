use compute::export::nalgebra::{Vector2, Vector3};
use encase::ShaderType;

use crate::camera::Camera;

#[derive(Default, ShaderType)]
pub struct Uniform {
    pub window: Vector2<u32>,
    pub camera: Camera,
    pub frame: u32,
    pub accumulation_frame: u32,

    pub max_bounces: u32,
    pub samples: u32,
}

#[derive(ShaderType, Default, Clone, Copy)]
pub struct Material {
    pub albedo: Vector3<f32>,
    pub emission: Vector3<f32>,
    pub roughness: f32,
}

#[derive(ShaderType, Default, Clone)]
pub struct Sphere {
    pub position: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}
