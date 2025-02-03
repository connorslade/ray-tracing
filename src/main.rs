use std::fs::File;

use anyhow::{Ok, Result};
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
mod camera;
mod consts;
mod misc;
mod types;
use app::App;
use consts::{DEFAULT_SPHERES, SHADER_SOURCE};
use stl_io::Vertex;
use types::{Material, Model, Triangle, Uniform};

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let mut mesh_file = File::open("teapot.stl")?;
    let stl = stl_io::read_stl(&mut mesh_file)?;
    let mut triangles = Vec::new();

    for face in stl.faces {
        let map = |x: Vertex| Vector3::new(x[0], x[1], x[2]);
        triangles.push(Triangle {
            v0: map(stl.vertices[face.vertices[0]]),
            v1: map(stl.vertices[face.vertices[1]]),
            v2: map(stl.vertices[face.vertices[2]]),

            n0: map(face.normal),
            n1: map(face.normal),
            n2: map(face.normal),
        });
    }

    let spheres = DEFAULT_SPHERES.to_vec();
    let models = vec![Model {
        material: Material {
            albedo: Vector3::new(1.0, 0.0, 0.0),
            emission: Vector3::new(0.0, 0.0, 0.0),
            emission_strength: 0.0,
            roughness: 1.0,
        },
        face_index: 0,
        face_count: 20,
    }];

    let sphere_buffer = gpu.create_storage_read(&spheres)?;
    let triangle_buffer = gpu.create_storage_read(&triangles)?;
    let models_buffer = gpu.create_storage_read(&models)?;

    let uniform_buffer = gpu.create_uniform(&Uniform::default())?;
    let accumulation_buffer = gpu.create_storage::<Vec<Vector3<f32>>>(&vec![])?;

    let pipeline = gpu
        .render_pipeline(SHADER_SOURCE)
        .bind_buffer(&uniform_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&accumulation_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&models_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&triangle_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&sphere_buffer, ShaderStages::FRAGMENT)
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
                exposure: 1.0,
                frame: 0,
                accumulation_frame: 1,

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
