use std::borrow::Cow;

use compute::export::wgpu::{ShaderModuleDescriptor, ShaderSource};

macro_rules! include_shader {
    ($name:expr) => {
        include_str!(concat!("../shaders/", $name))
    };
}

pub const COMPUTE_SOURCE: ShaderModuleDescriptor = ShaderModuleDescriptor {
    label: None,
    source: ShaderSource::Wgsl(Cow::Borrowed(concat!(
        include_shader!("main.wgsl"),
        include_shader!("types.wgsl"),
        include_shader!("random.wgsl"),
        include_shader!("misc.wgsl"),
        include_shader!("ray.wgsl"),
    ))),
};

pub const RENDER_SOURCE: ShaderModuleDescriptor = ShaderModuleDescriptor {
    label: None,
    source: ShaderSource::Wgsl(Cow::Borrowed(concat!(
        include_shader!("render.wgsl"),
        include_shader!("types.wgsl"),
    ))),
};
