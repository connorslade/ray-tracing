use std::{fmt::Debug, path::Path};

use anyhow::{Ok, Result};
use compute::{export::nalgebra::Vector3, gpu::Gpu};
use tobj::LoadOptions;

use crate::{
    bvh::{Bvh, BvhNode},
    types::{FaceBuffer, Material, Model, ModelBuffer, NodeBuffer, Triangle},
};

pub struct Scene {
    pub models: Vec<Model>,
    pub faces: Vec<Triangle>,
    pub nodes: Vec<BvhNode>,
}

impl Scene {
    pub fn empty() -> Self {
        Self {
            models: Vec::new(),
            faces: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn create_buffers(&self, gpu: &Gpu) -> Result<(ModelBuffer, NodeBuffer, FaceBuffer)> {
        let models = self.models.iter().map(|x| x.to_gpu()).collect();
        Ok((
            gpu.create_storage_read(&models)?,
            gpu.create_storage_read(&self.nodes)?,
            gpu.create_storage_read(&self.faces)?,
        ))
    }

    pub fn load(&mut self, path: impl AsRef<Path> + Debug) -> Result<()> {
        println!("[*] Loading `{path:?}`");

        let (obj, materials) = tobj::load_obj(
            path,
            &LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )?;
        let materials = materials?;

        let object_count = obj.len();
        for (i, model) in obj.into_iter().enumerate() {
            println!(
                " {} Loading `{}`",
                if i + 1 == object_count { "\\" } else { "|" },
                model.name
            );

            let mut triangles = Vec::new();
            let material = &materials[model.mesh.material_id.unwrap()];

            for face in model.mesh.indices.chunks_exact(3) {
                let vertex = |idx: u32| {
                    let start = idx as usize * 3;
                    let positions = &model.mesh.positions;
                    Vector3::new(positions[start], positions[start + 1], positions[start + 2])
                };

                let normal = |idx: u32| {
                    let start = idx as usize * 3;
                    let normals = &model.mesh.normals;
                    Vector3::new(normals[start], normals[start + 1], normals[start + 2])
                };

                triangles.push(Triangle {
                    vertices: [vertex(face[0]), vertex(face[1]), vertex(face[2])],
                    normals: [normal(face[0]), normal(face[1]), normal(face[2])],
                });
            }

            let face_offset = self.faces.len() as u32;
            let node_offset = self.nodes.len() as u32;
            let bvh = Bvh::from_mesh(&triangles);

            self.faces.extend(bvh.faces);
            self.nodes.extend(bvh.nodes);

            let diffuse = material.diffuse.unwrap();
            let specular = material.specular.unwrap();
            let shininess = material.shininess.unwrap() / 1000.0;
            let emission = material.unknown_param.get("Ke").unwrap();
            let emission = emission
                .split_ascii_whitespace()
                .map(|x| x.parse::<f32>().unwrap())
                .collect::<Vec<_>>();
            let emission = Vector3::new(emission[0], emission[1], emission[2]);

            self.models.push(Model {
                name: model.name,
                material: Material {
                    diffuse_color: Vector3::new(diffuse[0], diffuse[1], diffuse[2]),
                    specular_color: Vector3::new(specular[0], specular[1], specular[2]),

                    specular_probability: shininess,
                    roughness: 1.0,

                    emission_color: emission.try_normalize(0.0).unwrap_or_default(),
                    emission_strength: emission.magnitude(),
                },
                node_offset,
                face_offset,

                position: Vector3::zeros(),
                scale: Vector3::repeat(1.0),
            });
        }

        Ok(())
    }
}
