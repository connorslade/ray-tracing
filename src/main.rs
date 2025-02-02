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
use consts::{DEFAULT_SPHERES, SHADER_SOURCE};
use types::Uniform;

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let spheres = DEFAULT_SPHERES.to_vec();

    let uniform_buffer = gpu.create_uniform(&Uniform::default())?;
    let sphere_buffer = gpu.create_storage_read(&spheres)?;

    let pipeline = gpu
        .render_pipeline(SHADER_SOURCE)
        .bind_buffer(&uniform_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&sphere_buffer, ShaderStages::FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Ray Tracing"),
        App {
            pipeline,
            uniform_buffer,
            sphere_buffer,

            uniform: Uniform {
                camera: Camera::default(),
                frame: 0,

                max_bounces: 100,
                samples: 10,
            },
            spheres,
        },
    )
    .run()?;

    Ok(())
}
