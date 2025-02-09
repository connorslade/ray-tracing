use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};

use compute::{
    bindings::{acceleration_structure::AccelerationStructure, StorageBuffer, UniformBuffer},
    export::{
        egui::Context,
        nalgebra::{Matrix4, Vector2, Vector3},
        wgpu::RenderPass,
    },
    interactive::{GraphicsCtx, Interactive},
    misc::mutability::Mutable,
    pipeline::{compute::ComputePipeline, render::RenderPipeline},
};

use crate::{
    types::{Model, ModelBuffer, TransformBuffer, Uniform, Vertex},
    ui::ui,
};

pub struct App {
    pub compute_pipeline: ComputePipeline,
    pub render_pipeline: RenderPipeline,
    pub compute_running: Arc<AtomicBool>,
    pub accumulation_buffer: StorageBuffer<Vec<Vector3<f32>>, Mutable>,

    pub uniform: Uniform,
    pub uniform_buffer: UniformBuffer<Uniform>,

    pub models: Vec<Model>,
    pub acceleration_structure: AccelerationStructure<Vertex>,
    pub model_buffer: ModelBuffer,
    pub transform_buffer: TransformBuffer,

    pub last_frame: Instant,
    pub last_window: Vector2<u32>,
    pub accumulate: bool,
    pub screen_fraction: u8,
}

impl App {
    pub fn invalidate_accumulation(&mut self) {
        self.uniform.accumulation_frame = 0;
    }

    pub fn upload_models(&self) {
        let gpu_models = self.models.iter().map(|x| x.to_gpu()).collect::<Vec<_>>();
        self.model_buffer.upload_shrink(&gpu_models).unwrap();

        let transformations = self
            .models
            .iter()
            .map(|model| {
                let transformation = Matrix4::new_nonuniform_scaling(&model.scale)
                    * Matrix4::new_rotation(model.rotation)
                    * Matrix4::new_translation(&model.position);
                transformation.remove_row(3).transpose()
            })
            .collect::<Vec<_>>();
        self.transform_buffer.upload(&transformations).unwrap();
        self.acceleration_structure.update();
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
                self.uniform.accumulation_frame = 0;
            }

            let window = gcx.window.inner_size();
            let window = Vector2::new(window.width, window.height) / self.screen_fraction as u32;

            if self.last_window != window {
                self.uniform.accumulation_frame = 0;
                self.last_window = window;
                self.accumulation_buffer
                    .upload_shrink(&vec![Vector3::zeros(); (window.x * window.y) as usize])
                    .unwrap();
            }

            self.uniform.window = window;
            self.uniform.frame += 1;
            self.uniform_buffer.upload(&self.uniform).unwrap();

            let compute_running = self.compute_running.clone();
            self.compute_pipeline.queue_dispatch_callback(
                Vector3::new(window.x.div_ceil(8), window.y.div_ceil(8), 1),
                move || compute_running.store(false, Ordering::Relaxed),
            );
        }

        self.render_pipeline.draw_quad(render_pass, 0..1);
    }
}
