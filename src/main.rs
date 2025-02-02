use std::time::Instant;

use anyhow::{Ok, Result};
use camera::Camera;
use compute::{
    export::{wgpu::ShaderStages, winit::window::WindowAttributes},
    gpu::Gpu,
};

mod app;
mod camera;
mod consts;
mod misc;
mod types;
use app::App;
use consts::SHADER_SOURCE;
use types::Uniform;

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let uniform = gpu.create_uniform(&Uniform::default())?;
    let spheres = gpu.create_storage_read(&vec![])?;

    let pipeline = gpu
        .render_pipeline(SHADER_SOURCE)
        .bind_buffer(&uniform, ShaderStages::FRAGMENT)
        .bind_buffer(&spheres, ShaderStages::FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Ray Tracing"),
        App {
            pipeline,
            uniform_buffer: uniform,
            sphere_buffer: spheres,

            uniform: Uniform {
                camera: Camera::default(),
                frame: 0,

                max_bounces: 100,
                samples: 10,
            },
            start: Instant::now(),
        },
    )
    .run()?;

    Ok(())
}
