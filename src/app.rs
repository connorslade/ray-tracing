use compute::{
    buffer::UniformBuffer,
    export::{
        egui::{Context, Window},
        nalgebra::Vector3,
        wgpu::RenderPass,
        winit::{dpi::PhysicalPosition, window::CursorGrabMode},
    },
    interactive::{GraphicsCtx, Interactive},
    pipeline::render::RenderPipeline,
};

use crate::{camera::Camera, types::Uniform};

pub struct App {
    pub pipeline: RenderPipeline,
    pub uniform: UniformBuffer<Uniform>,

    pub camera: Camera,
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
        self.uniform
            .upload(&Uniform {
                camera: self.camera.clone(),
                light_dir: Vector3::new(1.0, -1.0, 1.0).normalize(),
            })
            .unwrap();

        self.pipeline.draw_quad(render_pass, 0..1);
    }
}
