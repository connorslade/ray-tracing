use anyhow::{Ok, Result};
use bvh::Bvh;
use camera::Camera;
use compute::{
    export::{
        nalgebra::{Vector2, Vector3},
        wgpu::ShaderStages,
        winit::window::WindowAttributes,
    },
    gpu::Gpu,
};

mod app;
mod bvh;
mod camera;
mod consts;
mod misc;
mod types;
mod ui;
use app::App;
use consts::SHADER_SOURCE;
use tobj::LoadOptions;
use types::{Material, Model, Triangle, Uniform};

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let (obj, materials) = tobj::load_obj(
        "teapot.obj",
        &LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;
    let materials = materials?;

    let mut models = Vec::new();
    let mut faces = Vec::new();
    let mut nodes = Vec::new();

    for model in obj {
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

        let face_offset = faces.len() as u32;
        let node_offset = nodes.len() as u32;
        let bvh = Bvh::from_mesh(&triangles);

        faces.extend(bvh.faces);
        nodes.extend(bvh.nodes);

        let diffuse = material.diffuse.unwrap();
        let shininess = material.shininess.unwrap() / 1000.0;
        let emission = material.unknown_param.get("Ke").unwrap();
        let emission = emission
            .split_ascii_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<_>>();
        let emission = Vector3::new(emission[0], emission[1], emission[2]);

        models.push(Model {
            material: Material {
                albedo: Vector3::new(diffuse[0], diffuse[1], diffuse[2]),
                emission_color: emission.try_normalize(0.0).unwrap_or_default(),
                emission_strength: emission.magnitude(),
                roughness: 1.0 - shininess,
            },
            node_offset,
            face_offset,
        });
    }

    let sphere_buffer = gpu.create_storage_read(&Vec::new())?;

    let model_buffer = gpu.create_storage_read(&models)?;
    let node_buffer = gpu.create_storage_read(&nodes)?;
    let face_buffer = gpu.create_storage_read(&faces)?;

    let uniform_buffer = gpu.create_uniform(&Uniform::default())?;
    let accumulation_buffer = gpu.create_storage::<Vec<Vector3<f32>>>(&vec![])?;

    let pipeline = gpu
        .render_pipeline(SHADER_SOURCE)
        .bind_buffer(&uniform_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&accumulation_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&sphere_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&model_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&node_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&face_buffer, ShaderStages::FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Ray Tracing"),
        App {
            pipeline,
            uniform_buffer,
            accumulation_buffer,

            sphere_buffer,
            model_buffer,

            uniform: Uniform {
                window: Vector2::zeros(),
                camera: Camera::default(),
                frame: 0,
                accumulation_frame: 1,

                environment: 1.0,
                max_bounces: 100,
                samples: 1,
            },
            spheres: Vec::new(),
            models,

            last_window: Vector2::zeros(),
            accumulate: true,
        },
    )
    .run()?;

    Ok(())
}
