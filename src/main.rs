use std::time::Instant;

use anyhow::{Ok, Result};
use camera::Camera;
use compute::{
    export::{
        nalgebra::Vector3,
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
use types::{Material, Sphere, Uniform};

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let material = Material {
        albedo: Vector3::new(1.0, 1.0, 1.0),
        emission: Vector3::new(0.0, 0.0, 0.0),
        roughness: 0.0,
        metallic: 1.0,
    };
    let spheres = vec![
        Sphere {
            position: Vector3::new(0.0, 0.0, -2.0),
            radius: 0.5,
            material,
        },
        Sphere {
            position: Vector3::new(0.0, 0.0, 2.0),
            radius: 0.5,
            material,
        },
    ];

    let uniform = gpu.create_uniform(&Uniform::default())?;
    let spheres = gpu.create_storage_read(&spheres)?;

    let pipeline = gpu
        .render_pipeline(include_wgsl!("shaders/render.wgsl"))
        .bind_buffer(&uniform, ShaderStages::FRAGMENT)
        .bind_buffer(&spheres, ShaderStages::FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Ray Tracing"),
        App {
            pipeline,
            uniform,
            spheres,

            camera: Camera::default(),

            start: Instant::now(),
        },
    )
    .run()?;

    Ok(())
}
