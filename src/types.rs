use std::hash::{Hash, Hasher};

use compute::{
    buffer::StorageBuffer,
    export::nalgebra::{Matrix4, Vector2, Vector3},
    misc::mutability::Immutable,
};
use encase::ShaderType;
use ordered_float::OrderedFloat;

use crate::{bvh::BvhNode, camera::Camera};

pub type ModelBuffer = StorageBuffer<Vec<GpuModel>, Immutable>;
pub type NodeBuffer = StorageBuffer<Vec<BvhNode>, Immutable>;
pub type FaceBuffer = StorageBuffer<Vec<Triangle>, Immutable>;
pub type SphereBuffer = StorageBuffer<Vec<Sphere>, Immutable>;

#[derive(Default, ShaderType)]
pub struct Uniform {
    pub window: Vector2<u32>,
    pub camera: Camera,
    pub frame: u32,
    pub accumulation_frame: u32,

    pub environment: f32,
    pub max_bounces: u32,
    pub samples: u32,
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
    pub material: Material,
    pub node_offset: u32,
    pub face_offset: u32,

    pub transformation: Matrix4<f32>,
    pub inv_transformation: Matrix4<f32>,
}

pub struct Model {
    pub name: String,
    pub id: u32,

    pub material: Material,
    pub node_offset: u32,
    pub face_offset: u32,

    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
}

#[derive(ShaderType, Default, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub position: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

#[derive(ShaderType, Debug, Default, Copy, Clone, PartialEq)]
pub struct Triangle {
    pub vertices: [Vector3<f32>; 3],
    pub normals: [Vector3<f32>; 3],
}

impl Triangle {
    pub fn center(&self) -> Vector3<f32> {
        self.vertices.iter().sum::<Vector3<_>>() / 2.0
    }
}

impl Model {
    pub fn to_gpu(&self) -> GpuModel {
        let transformation =
            Matrix4::new_nonuniform_scaling(&self.scale) * Matrix4::new_translation(&self.position);

        GpuModel {
            material: self.material,
            node_offset: self.node_offset,
            face_offset: self.face_offset,
            transformation,
            inv_transformation: transformation.try_inverse().unwrap(),
        }
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
        state.write_u32(self.node_offset);
        state.write_u32(self.face_offset);

        self.position.map(OrderedFloat).hash(state);
        self.scale.map(OrderedFloat).hash(state);
    }
}

impl Hash for Sphere {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.map(OrderedFloat).hash(state);
        OrderedFloat(self.radius).hash(state);
        self.material.hash(state);
    }
}
