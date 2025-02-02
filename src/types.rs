use compute::export::nalgebra::Vector3;
use encase::ShaderType;

use crate::camera::Camera;

#[derive(Default, ShaderType)]
pub struct Uniform {
    pub camera: Camera,
    pub light_dir: Vector3<f32>,
}

#[derive(ShaderType, Clone, Copy)]
pub struct Material {
    pub albedo: Vector3<f32>,
    pub emission: Vector3<f32>,
    pub roughness: f32,
    pub metallic: f32,
}

#[derive(ShaderType)]
pub struct Sphere {
    pub position: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}
