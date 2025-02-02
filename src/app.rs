use std::time::Instant;

use compute::{
    buffer::{StorageBuffer, UniformBuffer},
    export::{
        egui::{Context, Window},
        nalgebra::Vector3,
        wgpu::RenderPass,
        winit::{dpi::PhysicalPosition, window::CursorGrabMode},
    },
    interactive::{GraphicsCtx, Interactive},
    misc::mutability::Immutable,
    pipeline::render::RenderPipeline,
};

use crate::{
    camera::Camera,
    types::{Material, Sphere, Uniform},
};

pub struct App {
    pub pipeline: RenderPipeline,
    pub uniform: UniformBuffer<Uniform>,
    pub spheres: StorageBuffer<Vec<Sphere>, Immutable>,

    pub camera: Camera,

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

        self.camera.handle_movement(&gcx, ctx);
        Window::new("Ray Tracing").show(ctx, |ui| {
            ui.collapsing("Camera", |ui| {
                self.camera.ui(ui);
            });
        });
    }

    fn render(&mut self, _gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        let material = Material {
            albedo: Vector3::new(1.0, 1.0, 1.0),
            emission: Vector3::new(0.0, 0.0, 0.0),
            roughness: 0.0,
            metallic: 1.0,
        };

        let t = self.start.elapsed().as_secs_f32().sin() + 1.5;
        let spheres = vec![
            Sphere {
                position: Vector3::new(0.0, 0.0, t),
                radius: 0.5,
                material,
            },
            Sphere {
                position: Vector3::new(0.0, 0.0, -t),
                radius: 0.5,
                material,
            },
        ];
        self.spheres.upload_shrink(&spheres);

        self.uniform
            .upload(&Uniform {
                camera: self.camera.clone(),
                light_dir: Vector3::new(1.0, -1.0, 1.0).normalize(),
            })
            .unwrap();

        self.pipeline.draw_quad(render_pass, 0..1);
    }
}
