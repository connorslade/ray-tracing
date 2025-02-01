use anyhow::{Ok, Result};
use camera::Camera;
use compute::{
    export::{
        wgpu::{include_wgsl, ShaderStages},
        winit::window::WindowAttributes,
    },
    gpu::Gpu,
};

mod app;
mod camera;
mod misc;
mod types;
use app::App;
use types::Uniform;

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let uniform = gpu.create_uniform(&Uniform::default())?;
    let pipeline = gpu
        .render_pipeline(include_wgsl!("shaders/render.wgsl"))
        .bind_buffer(&uniform, ShaderStages::VERTEX_FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Ray Tracing"),
        App {
            pipeline,
            uniform,
            camera: Camera::default(),
        },
    )
    .run()?;

    Ok(())
}
