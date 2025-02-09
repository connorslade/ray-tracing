use std::{fs::File, io::BufReader, mem, path::Path};

use anyhow::{Ok, Result};
use compute::{
    bindings::{
        acceleration_structure::{AccelerationStructure, Geometry, GeometryPrimitive},
        BlasBuffer, TextureCollection,
    },
    export::nalgebra::{Matrix4, Matrix4x3, Vector2, Vector3},
    gpu::Gpu,
};
use image::{imageops, ImageFormat, RgbaImage};
use tobj::LoadOptions;

use crate::{
    misc::{next_id, GetUnknownMaterialParam},
    types::{Material, MetalMaterial, Model, ModelBuffer, Vertex},
};

pub struct Scene {
    pub primitives: Vec<GeometryPrimitive>,
    pub models: Vec<Model>,
    pub textures: Vec<RgbaImage>,

    pub verts: Vec<Vertex>,
    pub index: Vec<u32>,
}

pub struct SceneBuffers {
    pub models: ModelBuffer,
    pub vertex: BlasBuffer<Vertex>,
    pub index: BlasBuffer<u32>,
    pub transformation: BlasBuffer<Matrix4x3<f32>>,
    pub acceleration: AccelerationStructure<Vertex>,
    pub textures: TextureCollection,
}

impl Scene {
    pub fn empty() -> Self {
        Self {
            primitives: Vec::new(),
            models: Vec::new(),
            textures: Vec::new(),

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

        let textures = self
            .textures
            .iter()
            .map(|image| {
                let size = Vector2::new(image.width(), image.height());
                let texture = gpu.create_texture_2d(size);
                texture.upload(size, &imageops::flip_vertical(image));
                texture
            })
            .collect::<Vec<_>>();
        let textures = gpu.create_texture_collection(&textures);

        Ok(SceneBuffers {
            models,
            vertex,
            index,
            transformation,
            acceleration,
            textures,
        })
    }

    pub fn load(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let dir = path.parent().unwrap();
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

            let verts = mesh
                .positions
                .chunks_exact(3)
                .zip(mesh.normals.chunks_exact(3))
                .enumerate()
                .map(|(idx, (pos, normal))| {
                    let texcoords = &mesh
                        .texcoords
                        .get(idx * 2..idx * 2 + 2)
                        .unwrap_or(&[0.0, 0.0]);

                    Vertex {
                        position: Vector3::new(pos[0], pos[1], pos[2]),
                        normal: Vector3::new(normal[0], normal[1], normal[2]),
                        uv: Vector2::new(texcoords[0], texcoords[1]),
                    }
                });
            self.verts.extend(verts);
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

            let mut load_texture = |path: &Option<String>| {
                if let Some(file) = path {
                    let path = dir.join(strip_flags(file));
                    let file = BufReader::new(File::open(&path).unwrap());
                    let format = ImageFormat::from_path(path).unwrap();

                    let image = image::load(file, format).unwrap().into_rgba8();
                    self.textures.push(image);
                    self.textures.len() as u32
                } else {
                    0
                }
            };

            let diffuse_texture = load_texture(&material.diffuse_texture);

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

                    diffuse_texture,
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

fn strip_flags(path: &str) -> &str {
    let mut i = 0;

    while path[i..].starts_with('-') {
        for _ in 0..2 {
            while !path[i..].starts_with(' ') {
                i += 1;
            }
            i += 1;
        }
    }

    &path[i..]
}
