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
use app::App;
use consts::{DEFAULT_SPHERES, SHADER_SOURCE};
use tobj::LoadOptions;
use types::{Material, Model, Triangle, Uniform};

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let (obj, _materials) = tobj::load_obj(
        "square.obj",
        &LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;

    let mut models = Vec::new();
    let mut faces = Vec::new();
    let mut nodes = Vec::new();

    for model in obj {
        let mut triangles = Vec::new();

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

        models.push(Model {
            material: Material {
                albedo: Vector3::new(0.5, 1.0, 1.0),
                emission: Vector3::repeat(0.0),
                emission_strength: 0.0,
                roughness: 0.5,
            },
            node_offset,
            face_offset,
        });
    }

    let spheres = DEFAULT_SPHERES.to_vec();

    let sphere_buffer = gpu.create_storage_read(&spheres)?;

    let models_buffer = gpu.create_storage_read(&models)?;
    let node_buffer = gpu.create_storage_read(&nodes)?;
    let face_buffer = gpu.create_storage_read(&faces)?;

    let uniform_buffer = gpu.create_uniform(&Uniform::default())?;
    let accumulation_buffer = gpu.create_storage::<Vec<Vector3<f32>>>(&vec![])?;

    let pipeline = gpu
        .render_pipeline(SHADER_SOURCE)
        .bind_buffer(&uniform_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&accumulation_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&sphere_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&models_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&node_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&face_buffer, ShaderStages::FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Ray Tracing"),
        App {
            pipeline,
            uniform_buffer,
            sphere_buffer,
            accumulation_buffer,

            uniform: Uniform {
                window: Vector2::zeros(),
                camera: Camera::default(),
                frame: 0,
                accumulation_frame: 1,

                environment: 1.0,
                max_bounces: 100,
                samples: 1,
            },
            spheres,

            last_window: Vector2::zeros(),
            accumulate: true,
        },
    )
    .run()?;

    Ok(())
}
