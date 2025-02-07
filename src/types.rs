use std::hash::{Hash, Hasher};

use bitflags::bitflags;
use compute::{
    bindings::{BlasBuffer, StorageBuffer},
    export::nalgebra::{Matrix4x3, Vector2, Vector3},
    misc::mutability::Immutable,
};
use encase::ShaderType;
use ordered_float::OrderedFloat;

use crate::camera::Camera;

pub type ModelBuffer = StorageBuffer<Vec<GpuModel>, Immutable>;
pub type TransformBuffer = BlasBuffer<Matrix4x3<f32>>;

#[derive(Default, ShaderType)]
pub struct Uniform {
    pub window: Vector2<u32>,
    pub camera: Camera,
    pub frame: u32,
    pub accumulation_frame: u32,
    pub flags: u32,

    pub environment: f32,
    pub max_bounces: u32,
    pub samples: u32,
}

bitflags! {
    pub struct Flags: u32 {
        const CULL_BACKFACES = 1;
    }
}

#[derive(ShaderType, Debug, Default, Clone, Copy, PartialEq)]
pub struct Material {
    pub diffuse_color: Vector3<f32>,
    pub specular_color: Vector3<f32>,

    pub specular_probability: f32,
    pub roughness: f32,

    pub emission_color: Vector3<f32>,
    pub emission_strength: f32,
}

#[derive(ShaderType, Default, Clone, Copy, PartialEq)]
pub struct GpuModel {
    material: Material,
    vertex_start: u32,
    index_start: u32,
}

pub struct Model {
    pub name: String,
    pub id: u32,

    pub material: Material,
    pub vertex_start: u32,
    pub index_start: u32,

    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

#[derive(ShaderType)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl Model {
    pub fn to_gpu(&self) -> GpuModel {
        GpuModel {
            material: self.material,
            vertex_start: self.vertex_start,
            index_start: self.index_start,
        }
    }
}

impl Hash for Uniform {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.camera.hash(state);
        state.write_u32(self.flags);
        OrderedFloat(self.environment).hash(state);
        state.write_u32(self.max_bounces);
        state.write_u32(self.samples);
    }
}

impl Hash for Material {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.diffuse_color.map(OrderedFloat).hash(state);
        self.specular_color.map(OrderedFloat).hash(state);
        self.emission_color.map(OrderedFloat).hash(state);
        OrderedFloat(self.emission_strength).hash(state);
        OrderedFloat(self.roughness).hash(state);
        OrderedFloat(self.specular_probability).hash(state);
    }
}

impl Hash for Model {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.material.hash(state);
        self.position.map(OrderedFloat).hash(state);
        self.scale.map(OrderedFloat).hash(state);
        self.rotation.map(OrderedFloat).hash(state);
    }
}
