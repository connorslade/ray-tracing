use std::{fmt::Debug, path::Path};

use anyhow::{Ok, Result};
use compute::{
    bindings::{
        acceleration_structure::{AccelerationStructure, Geometry, GeometryPrimitive},
        BlasBuffer,
    },
    export::nalgebra::{Matrix4, Vector3},
    gpu::Gpu,
};
use tobj::LoadOptions;

use crate::{
    misc::next_id,
    types::{Material, Model, ModelBuffer, Vertex},
};

pub struct Scene {
    pub geometry: Vec<Geometry>,
    pub models: Vec<Model>,

    pub verts: Vec<Vertex>,
    pub index: Vec<u32>,
}

impl Scene {
    pub fn empty() -> Self {
        Self {
            geometry: Vec::new(),
            models: Vec::new(),

            verts: Vec::new(),
            index: Vec::new(),
        }
    }

    pub fn finish(
        &self,
        gpu: &Gpu,
    ) -> Result<(
        ModelBuffer,
        BlasBuffer<Vertex>,
        BlasBuffer<u32>,
        AccelerationStructure,
    )> {
        let vertex = gpu.create_blas(&self.verts)?;
        let index = gpu.create_blas(&self.index)?;
        let acceleration = gpu.create_acceleration_structure(&vertex, &index, &self.geometry);

        let models = self.models.iter().map(|x| x.to_gpu()).collect::<Vec<_>>();
        let models = gpu.create_storage_read(&models)?;

        Ok((models, vertex, index, acceleration))
    }

    pub fn load(&mut self, path: impl AsRef<Path> + Debug) -> Result<()> {
        println!("[*] Loading {path:?}");

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
        let mut primitives = Vec::new();
        for (i, model) in obj.into_iter().enumerate() {
            println!(
                " {} Loading `{}`",
                if i + 1 == object_count { "\\" } else { "|" },
                model.name
            );

            let (first_index, first_vertex) = (self.index.len(), self.verts.len());
            let mesh = &model.mesh;

            self.verts.extend(
                mesh.positions
                    .chunks_exact(3)
                    .zip(mesh.normals.chunks_exact(3))
                    .map(|(pos, normal)| Vertex {
                        position: Vector3::new(pos[0], pos[1], pos[2]),
                        normal: Vector3::new(normal[0], normal[1], normal[2]),
                    }),
            );
            self.index.extend_from_slice(&mesh.indices);

            primitives.push(GeometryPrimitive {
                first_vertex: first_vertex as u32,
                vertex_count: (self.verts.len() - first_vertex) as u32,
                first_index: first_index as u32,
                index_count: (self.index.len() - first_index) as u32,
            });

            let material = &materials[model.mesh.material_id.unwrap()];
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
                id: next_id(),

                material: Material {
                    diffuse_color: Vector3::new(diffuse[0], diffuse[1], diffuse[2]),
                    specular_color: Vector3::new(specular[0], specular[1], specular[2]),

                    specular_probability: shininess,
                    roughness: 1.0,

                    emission_color: emission.try_normalize(0.0).unwrap_or_default(),
                    emission_strength: emission.magnitude(),
                },
                vertex_start: first_vertex as u32,
                index_start: first_index as u32,

                position: Vector3::zeros(),
                scale: Vector3::repeat(1.0),
            });
        }

        self.geometry.push(Geometry {
            transformation: Matrix4::identity(),
            primitives,
        });

        Ok(())
    }
}
