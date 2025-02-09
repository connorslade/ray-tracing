use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Instant,
};

use anyhow::{Ok, Result};
use camera::Camera;
use compute::{
    export::{
        nalgebra::{Vector2, Vector3},
        wgpu::{Features, PowerPreference, ShaderStages},
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
        .power_preference(PowerPreference::HighPerformance)
        .with_features(
            Features::TEXTURE_BINDING_ARRAY
                | Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
        )
        .with_raytracing()
        .build()?;

    let mut scene = Scene::empty();
    scene.load("scenes/lens.obj")?;

    let buffers = scene.finish(&gpu)?;
    let uniform_buffer = gpu.create_uniform(&Uniform::default())?;
    let accumulation_buffer = gpu.create_storage::<Vec<Vector3<f32>>>(&vec![])?;

    let sampler = gpu.create_sampler();
    let compute_pipeline = gpu
        .compute_pipeline(COMPUTE_SOURCE)
        .bind(&uniform_buffer)
        .bind(&accumulation_buffer)
        .bind(&buffers.models)
        .bind(&buffers.acceleration)
        .bind(&buffers.vertex)
        .bind(&buffers.index)
        .bind(&sampler)
        .bind(&buffers.textures)
        .finish();
    let render_pipeline = gpu
        .render_pipeline(RENDER_SOURCE)
        .bind(&uniform_buffer, ShaderStages::FRAGMENT)
        .bind(&accumulation_buffer, ShaderStages::FRAGMENT)
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

                exposure: 1.0,
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
