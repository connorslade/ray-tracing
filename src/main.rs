use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Instant,
};

use anyhow::{Ok, Result};
use camera::Camera;
use compute::{
    export::{
        nalgebra::{Vector2, Vector3},
        wgpu::{PowerPreference, ShaderStages},
        winit::window::WindowAttributes,
    },
    gpu::Gpu,
};

mod app;
mod camera;
mod consts;
mod misc;
mod scene;
mod types;
mod ui;
use app::App;
use consts::{COMPUTE_SOURCE, RENDER_SOURCE};
use scene::Scene;
use types::{Flags, Uniform};

fn main() -> Result<()> {
    let gpu = Gpu::builder()
        .with_raytracing()
        .power_preference(PowerPreference::HighPerformance)
        .build()?;

    let mut scene = Scene::empty();
    scene.load("scenes/lens.obj")?;

    let buffers = scene.finish(&gpu)?;
    let uniform_buffer = gpu.create_uniform(&Uniform::default())?;
    let accumulation_buffer = gpu.create_storage::<Vec<Vector3<f32>>>(&vec![])?;

    let compute_pipeline = gpu
        .compute_pipeline(COMPUTE_SOURCE)
        .bind_buffer(&uniform_buffer)
        .bind_buffer(&accumulation_buffer)
        .bind_buffer(&buffers.models)
        .bind_buffer(&buffers.acceleration)
        .bind_buffer(&buffers.vertex)
        .bind_buffer(&buffers.index)
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

            model_buffer: buffers.models,
            acceleration_structure: buffers.acceleration,
            transform_buffer: buffers.transformation,
            uniform: Uniform {
                window: Vector2::zeros(),
                camera: Camera::default(),
                frame: 0,
                accumulation_frame: 1,
                flags: Flags::empty().bits(),

                environment: 1.0,
                max_bounces: 10,
                samples: 5,
            },

            models: scene.models,
            last_frame: Instant::now(),
            last_window: Vector2::zeros(),
            accumulate: true,
            screen_fraction: 2,
        },
    )
    .run()?;

    Ok(())
}
