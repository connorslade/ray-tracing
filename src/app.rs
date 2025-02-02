use std::time::Instant;

use compute::{
    buffer::{StorageBuffer, UniformBuffer},
    export::{
        egui::{Context, Slider, Window},
        nalgebra::Vector3,
        wgpu::RenderPass,
        winit::{dpi::PhysicalPosition, window::CursorGrabMode},
    },
    interactive::{GraphicsCtx, Interactive},
    misc::mutability::Immutable,
    pipeline::render::RenderPipeline,
};

use crate::types::{Material, Sphere, Uniform};

pub struct App {
    pub pipeline: RenderPipeline,
    pub uniform_buffer: UniformBuffer<Uniform>,
    pub sphere_buffer: StorageBuffer<Vec<Sphere>, Immutable>,

    pub uniform: Uniform,
    pub start: Instant,
}

impl Interactive for App {
    fn ui(&mut self, gcx: GraphicsCtx, ctx: &Context) {
        let window = gcx.window.inner_size();
        gcx.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
        gcx.window
            .set_cursor_position(PhysicalPosition::new(window.width / 2, window.height / 2))
            .unwrap();
        gcx.window.set_cursor_grab(CursorGrabMode::None).unwrap();

        self.uniform.camera.handle_movement(&gcx, ctx);
        Window::new("Ray Tracing")
            .default_width(0.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut self.uniform.max_bounces, 1..=100));
                    ui.label("Max Bounces");
                });

                ui.separator();

                ui.collapsing("Camera", |ui| {
                    self.uniform.camera.ui(ui);
                });
            });
    }

    fn render(&mut self, _gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        let t = self.start.elapsed().as_secs_f32().sin() + 1.5;
        let spheres = vec![
            Sphere {
                position: Vector3::new(0.0, 0.0, t),
                radius: 0.5,
                material: Material {
                    albedo: Vector3::new(1.0, 1.0, 1.0),
                    emission: Vector3::new(0.0, 0.0, 0.0),
                    roughness: 0.0,
                    metallic: 0.0,
                },
            },
            Sphere {
                position: Vector3::new(0.0, 0.0, -t),
                radius: 0.5,
                material: Material {
                    albedo: Vector3::new(1.0, 0.0, 0.0),
                    emission: Vector3::new(0.0, 0.0, 0.0),
                    roughness: 0.0,
                    metallic: 0.0,
                },
            },
        ];

        self.sphere_buffer.upload_shrink(&spheres);
        self.uniform_buffer.upload(&self.uniform).unwrap();

        self.pipeline.draw_quad(render_pass, 0..1);
    }
}
