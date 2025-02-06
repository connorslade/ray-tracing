use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Instant,
};

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
mod bvh;
mod camera;
mod consts;
mod misc;
mod scene;
mod types;
mod ui;
use app::App;
use consts::{COMPUTE_SOURCE, RENDER_SOURCE};
use scene::Scene;
use types::Uniform;

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let mut scene = Scene::empty();
    scene.load("scenes/mirascope.obj")?;

    let vert = gpu.create_blas(&scene.verts, "vert")?;
    let index = gpu.create_blas(&scene.index, "index")?;
    let acc = gpu.create_acceleration_structure(vert, index, &scene.geometry);

    let sphere_buffer = gpu.create_storage_read(&Vec::new())?;
    let (model_buffer, node_buffer, face_buffer) = scene.create_buffers(&gpu)?;

    let uniform_buffer = gpu.create_uniform(&Uniform::default())?;
    let accumulation_buffer = gpu.create_storage::<Vec<Vector3<f32>>>(&vec![])?;

    let compute_pipeline = gpu
        .compute_pipeline(COMPUTE_SOURCE)
        .bind_buffer(&uniform_buffer)
        .bind_buffer(&accumulation_buffer)
        // .bind_buffer(&sphere_buffer)
        // .bind_buffer(&model_buffer)
        // .bind_buffer(&node_buffer)
        // .bind_buffer(&face_buffer)
        .bind_buffer(&acc)
        .finish();
    let render_pipeline = gpu
        .render_pipeline(RENDER_SOURCE)
        .bind_buffer(&uniform_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&accumulation_buffer, ShaderStages::FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Ray Tracing"),
        App {
            compute_pipeline,
            render_pipeline,
            compute_running: Arc::new(AtomicBool::new(false)),

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
                max_bounces: 10,
                samples: 5,
            },
            spheres: Vec::new(),
            models: scene.models,

            last_frame: Instant::now(),
            last_window: Vector2::zeros(),
            accumulate: true,
            screen_fraction: 16,
        },
    )
    .run()?;

    Ok(())
}
