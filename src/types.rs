use compute::export::nalgebra::Vector3;
use encase::ShaderType;

use crate::camera::Camera;

#[derive(Default, ShaderType)]
pub struct Uniform {
    pub camera: Camera,
    pub light_dir: Vector3<f32>,
}
