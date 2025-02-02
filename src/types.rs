use std::hash::{Hash, Hasher};

use compute::export::nalgebra::{Vector2, Vector3};
use encase::ShaderType;
use ordered_float::OrderedFloat;

use crate::camera::Camera;

#[derive(Default, ShaderType)]
pub struct Uniform {
    pub window: Vector2<u32>,
    pub camera: Camera,
    pub exposure: f32,
    pub frame: u32,
    pub accumulation_frame: u32,

    pub max_bounces: u32,
    pub samples: u32,
}

#[derive(ShaderType, Default, Clone, Copy, PartialEq)]
pub struct Material {
    pub albedo: Vector3<f32>,
    pub emission: Vector3<f32>,
    pub emission_strength: f32,
    pub roughness: f32,
}

#[derive(ShaderType, Default, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub position: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

#[derive(ShaderType, Default, Copy, Clone, PartialEq)]
pub struct Triangle {
    pub v0: Vector3<f32>,
    pub v1: Vector3<f32>,
    pub v2: Vector3<f32>,

    pub normal: Vector3<f32>,
}

impl Hash for Material {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.albedo.map(OrderedFloat).hash(state);
        self.emission.map(OrderedFloat).hash(state);
        OrderedFloat(self.emission_strength).hash(state);
        OrderedFloat(self.roughness).hash(state);
    }
}

impl Hash for Sphere {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.map(OrderedFloat).hash(state);
        OrderedFloat(self.radius).hash(state);
        self.material.hash(state);
    }
}
