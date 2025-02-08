use std::{fmt::Debug, mem, path::Path};

use anyhow::{Ok, Result};
use compute::{
    bindings::{
        acceleration_structure::{AccelerationStructure, Geometry, GeometryPrimitive},
        BlasBuffer,
    },
    export::nalgebra::{Matrix4, Matrix4x3, Vector3},
    gpu::Gpu,
};
use tobj::LoadOptions;

use crate::{
    misc::{next_id, GetUnknownMaterialParam},
    types::{Material, MetalMaterial, Model, ModelBuffer, Vertex},
};

pub struct Scene {
    pub primitives: Vec<GeometryPrimitive>,
    pub models: Vec<Model>,

    pub verts: Vec<Vertex>,
    pub index: Vec<u32>,
}

pub struct SceneBuffers {
    pub models: ModelBuffer,
    pub vertex: BlasBuffer<Vertex>,
    pub index: BlasBuffer<u32>,
    pub transformation: BlasBuffer<Matrix4x3<f32>>,
    pub acceleration: AccelerationStructure<Vertex>,
}

impl Scene {
    pub fn empty() -> Self {
        Self {
            primitives: Vec::new(),
            models: Vec::new(),

            verts: Vec::new(),
            index: Vec::new(),
        }
    }

    pub fn finish(&mut self, gpu: &Gpu) -> Result<SceneBuffers> {
        let vertex = gpu.create_blas(&self.verts)?;
        let index = gpu.create_blas(&self.index)?;
        let transformation =
            gpu.create_blas(&vec![Matrix4x3::identity(); self.primitives.len()])?;

        let acceleration = gpu.create_acceleration_structure(
            vertex.clone(),
            index.clone(),
            transformation.clone(),
            vec![Geometry {
                transformation: Matrix4::identity(),
                primitives: mem::take(&mut self.primitives),
            }],
        );

        let models = self.models.iter().map(|x| x.to_gpu()).collect::<Vec<_>>();
        let models = gpu.create_storage_read(&models)?;

        Ok(SceneBuffers {
            models,
            vertex,
            index,
            transformation,
            acceleration,
        })
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

            self.primitives.push(GeometryPrimitive {
                first_vertex: first_vertex as u32,
                vertex_count: (self.verts.len() - first_vertex) as u32,
                first_index: first_index as u32,
                index_count: (self.index.len() - first_index) as u32,
                transformation_offset: self.primitives.len() as u64,
            });

            let material = &materials[model.mesh.material_id.unwrap()];

            let diffuse = material.diffuse.unwrap_or_default();
            let specular = material.specular.unwrap_or_default();
            let specular_probability = material.get_unknown("Pm");
            let roughness = material.get_unknown("Pr");
            let emission: Vector3<_> = material.get_unknown("Ke");

            self.models.push(Model {
                name: model.name,
                id: next_id(),

                material: Material::metal(MetalMaterial {
                    diffuse_color: Vector3::from_row_slice(&diffuse),
                    specular_color: Vector3::from_row_slice(&specular),

                    specular_probability,
                    roughness,

                    emission_color: emission.try_normalize(0.0).unwrap_or_default(),
                    emission_strength: emission.magnitude(),
                }),
                vertex_start: first_vertex as u32,
                index_start: first_index as u32,

                position: Vector3::zeros(),
                scale: Vector3::repeat(1.0),
                rotation: Vector3::repeat(0.0),
            });
        }

        Ok(())
    }
}
