use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};

use compute::{
    buffer::{StorageBuffer, UniformBuffer},
    export::{
        egui::Context,
        nalgebra::{Vector2, Vector3},
        wgpu::RenderPass,
    },
    interactive::{GraphicsCtx, Interactive},
    misc::mutability::Mutable,
    pipeline::{compute::ComputePipeline, render::RenderPipeline},
};

use crate::{
    types::{Model, ModelBuffer, Sphere, SphereBuffer, Uniform},
    ui::ui,
};

pub struct App {
    pub compute_pipeline: ComputePipeline,
    pub render_pipeline: RenderPipeline,
    pub compute_running: Arc<AtomicBool>,

    pub uniform_buffer: UniformBuffer<Uniform>,
    pub accumulation_buffer: StorageBuffer<Vec<Vector3<f32>>, Mutable>,

    pub sphere_buffer: SphereBuffer,
    pub model_buffer: ModelBuffer,

    pub uniform: Uniform,
    pub spheres: Vec<Sphere>,
    pub models: Vec<Model>,

    pub last_frame: Instant,
    pub last_window: Vector2<u32>,
    pub accumulate: bool,
    pub screen_fraction: u8,
}

impl App {
    pub fn invalidate_accumulation(&mut self) {
        self.uniform.accumulation_frame = 1;
    }

    pub fn upload_models(&self) {
        let gpu_models = self.models.iter().map(|x| x.to_gpu()).collect::<Vec<_>>();
        self.model_buffer.upload_shrink(&gpu_models).unwrap();
    }
}

impl Interactive for App {
    fn init(&mut self, _gcx: GraphicsCtx) {
        self.upload_models();
    }

    fn ui(&mut self, gcx: GraphicsCtx, ctx: &Context) {
        ui(self, gcx, ctx);
    }

    fn render(&mut self, gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        if !self.compute_running.swap(true, Ordering::Relaxed) {
            self.uniform.accumulation_frame += 1;
            if !self.accumulate {
                self.uniform.accumulation_frame = 1;
            }

            let window = gcx.window.inner_size();
            let window = Vector2::new(window.width, window.height) / self.screen_fraction as u32;

            let compute_running = self.compute_running.clone();
            self.compute_pipeline.dispatch_callback(
                Vector3::new(window.x.div_ceil(8), window.y.div_ceil(8), 1),
                move || compute_running.store(false, Ordering::Relaxed),
            );

            if self.last_window != window {
                self.uniform.accumulation_frame = 1;
                self.last_window = window;
                self.accumulation_buffer
                    .upload_shrink(&vec![Vector3::zeros(); (window.x * window.y) as usize])
                    .unwrap();
            }

            self.uniform.window = window;
            self.uniform.frame += 1;
            self.uniform_buffer.upload(&self.uniform).unwrap();
        }

        self.render_pipeline.draw_quad(render_pass, 0..1);
    }
}
