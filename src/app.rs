use compute::{
    buffer::{StorageBuffer, UniformBuffer},
    export::{
        egui::Context,
        nalgebra::{Vector2, Vector3},
        wgpu::RenderPass,
    },
    interactive::{GraphicsCtx, Interactive},
    misc::mutability::{Immutable, Mutable},
    pipeline::render::RenderPipeline,
};

use crate::{
    types::{Model, Sphere, Uniform},
    ui::ui,
};

pub struct App {
    pub pipeline: RenderPipeline,
    pub uniform_buffer: UniformBuffer<Uniform>,
    pub accumulation_buffer: StorageBuffer<Vec<Vector3<f32>>, Mutable>,

    pub sphere_buffer: StorageBuffer<Vec<Sphere>, Immutable>,
    pub model_buffer: StorageBuffer<Vec<Model>, Immutable>,

    pub uniform: Uniform,
    pub spheres: Vec<Sphere>,
    pub models: Vec<Model>,

    pub last_window: Vector2<u32>,
    pub accumulate: bool,
}

impl App {
    pub fn invalidate_accumulation(&mut self) {
        self.uniform.accumulation_frame = 1;
    }
}

impl Interactive for App {
    fn ui(&mut self, gcx: GraphicsCtx, ctx: &Context) {
        ui(self, gcx, ctx);
    }

    fn render(&mut self, gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        self.uniform.accumulation_frame += 1;
        if !self.accumulate {
            self.uniform.accumulation_frame = 1;
        }

        let window = gcx.window.inner_size();
        let window = Vector2::new(window.width, window.height);

        self.uniform.window = window;
        if self.last_window != window {
            self.uniform.accumulation_frame = 1;
            self.last_window = window;
            self.accumulation_buffer
                .upload_shrink(&vec![Vector3::zeros(); (window.x * window.y) as usize])
                .unwrap();
        }

        self.uniform.frame += 1;
        self.uniform_buffer.upload(&self.uniform).unwrap();

        self.pipeline.draw_quad(render_pass, 0..1);
    }
}
