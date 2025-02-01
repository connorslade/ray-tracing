use encase::ShaderType;

use crate::camera::Camera;

#[derive(Default, ShaderType)]
pub struct Uniform {
    pub camera: Camera,
}
