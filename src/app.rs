use compute::{
    buffer::UniformBuffer,
    export::{
        egui::{Context, Window},
        wgpu::RenderPass,
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
            })
            .unwrap();

        self.pipeline.draw_quad(render_pass, 0..1);
    }
}
