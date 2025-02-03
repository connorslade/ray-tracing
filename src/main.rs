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
use consts::{DEFAULT_MODELS, DEFAULT_SPHERES, DEFAULT_TRIANGLES, SHADER_SOURCE};
use types::Uniform;

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let spheres = DEFAULT_SPHERES.to_vec();
    let triangles = DEFAULT_TRIANGLES.to_vec();
    let models = DEFAULT_MODELS.to_vec();

    let sphere_buffer = gpu.create_storage_read(&spheres)?;
    let triangle_buffer = gpu.create_storage_read(&triangles)?;
    let models_buffer = gpu.create_storage_read(&models)?;

    let uniform_buffer = gpu.create_uniform(&Uniform::default())?;
    let accumulation_buffer = gpu.create_storage::<Vec<Vector3<f32>>>(&vec![])?;

    let pipeline = gpu
        .render_pipeline(SHADER_SOURCE)
        .bind_buffer(&uniform_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&sphere_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&triangle_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&models_buffer, ShaderStages::FRAGMENT)
        .bind_buffer(&accumulation_buffer, ShaderStages::FRAGMENT)
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
