use std::borrow::Cow;

use compute::export::{
    nalgebra::Vector3,
    wgpu::{ShaderModuleDescriptor, ShaderSource},
};

use crate::types::{Material, Model, Sphere, Triangle};

macro_rules! include_shader {
    ($name:expr) => {
        include_str!(concat!("../shaders/", $name))
    };
}

pub const SHADER_SOURCE: ShaderModuleDescriptor = ShaderModuleDescriptor {
    label: None,
    source: ShaderSource::Wgsl(Cow::Borrowed(concat!(
        include_shader!("main.wgsl"),
        include_shader!("types.wgsl"),
        include_shader!("vertex.wgsl"),
        include_shader!("random.wgsl"),
        include_shader!("misc.wgsl"),
        include_shader!("ray.wgsl"),
    ))),
};

pub const DEFAULT_SPHERES: [Sphere; 3] = [
    Sphere {
        position: Vector3::new(0.0, -0.5, 1.0),
        radius: 0.5,
        material: Material {
            albedo: Vector3::new(1.0, 1.0, 1.0),
            emission: Vector3::new(0.0, 0.0, 0.0),
            emission_strength: 0.0,
            roughness: 0.0,
        },
    },
    Sphere {
        position: Vector3::new(0.0, -0.5, -1.0),
        radius: 0.5,
        material: Material {
            albedo: Vector3::new(1.0, 0.8, 0.8),
            emission: Vector3::new(0.0, 0.0, 0.0),
            emission_strength: 0.0,
            roughness: 0.1,
        },
    },
    Sphere {
        position: Vector3::new(0.0, -1001.0, 0.0),
        radius: 1000.0,
        material: Material {
            albedo: Vector3::new(0.8, 0.8, 1.0),
            emission: Vector3::new(0.0, 0.0, 0.0),
            emission_strength: 0.0,
            roughness: 1.0,
        },
    },
];

pub const DEFAULT_TRIANGLES: [Triangle; 1] = [Triangle {
    v0: Vector3::new(0.0, 1.0, 0.0),
    v1: Vector3::new(0.0, 0.0, -1.0),
    v2: Vector3::new(0.0, 0.0, 1.0),

    n0: Vector3::new(-0.5, 0.5, 0.0),
    n1: Vector3::new(-1.0, 0.0, 0.0),
    n2: Vector3::new(-1.0, 0.0, 0.0),
}];

pub const DEFAULT_MODELS: [Model; 1] = [Model {
    material: Material {
        albedo: Vector3::new(0.8, 0.8, 1.0),
        emission: Vector3::new(0.0, 0.0, 0.0),
        emission_strength: 0.0,
        roughness: 1.0,
    },
    vertex: 0,
    vertex_count: 1,
}];
