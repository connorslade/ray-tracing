use std::{fmt::Debug, path::Path};

use anyhow::{Ok, Result};
use compute::{
    bindings::acceleration_structure::{AccelerationStructure, Geometry, GeometryPrimitive},
    export::nalgebra::{Matrix4, Vector3},
    gpu::Gpu,
};
use encase::ShaderType;
use tobj::LoadOptions;

pub struct Scene {
    pub geometry: Vec<Geometry>,
    pub verts: Vec<Vertex>,
    pub index: Vec<u32>,
}

#[derive(ShaderType)]
pub struct Vertex {
    position: Vector3<f32>,
    normal: Vector3<f32>,
}

impl Scene {
    pub fn empty() -> Self {
        Self {
            geometry: Vec::new(),
            verts: Vec::new(),
            index: Vec::new(),
        }
    }

    pub fn finish(&self, gpu: &Gpu) -> Result<AccelerationStructure> {
        let vertex = gpu.create_blas(&self.verts)?;
        let index = gpu.create_blas(&self.index)?;

        let acceleration = gpu.create_acceleration_structure(vertex, index, &self.geometry);

        Ok(acceleration)
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
        for (i, model) in obj.into_iter().skip(3).take(2).enumerate() {
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

            self.geometry.push(Geometry {
                transformation: Matrix4::identity(),
                primitives: vec![GeometryPrimitive {
                    first_vertex: first_vertex as u32,
                    vertex_count: (self.verts.len() - first_vertex) as u32,
                    first_index: first_index as u32,
                    index_count: (self.index.len() - first_index) as u32,
                }],
            });
        }

        Ok(())
    }
}
