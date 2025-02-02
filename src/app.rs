use compute::{
    buffer::{StorageBuffer, UniformBuffer},
    export::{
        egui::{Context, DragValue, Grid, Slider, Window},
        nalgebra::{Vector2, Vector3},
        wgpu::RenderPass,
        winit::{dpi::PhysicalPosition, window::CursorGrabMode},
    },
    interactive::{GraphicsCtx, Interactive},
    misc::mutability::{Immutable, Mutable},
    pipeline::render::RenderPipeline,
};

use crate::{
    misc::vec3_dragger,
    types::{Sphere, Uniform},
};

pub struct App {
    pub pipeline: RenderPipeline,
    pub uniform_buffer: UniformBuffer<Uniform>,
    pub sphere_buffer: StorageBuffer<Vec<Sphere>, Immutable>,
    pub accumulation_buffer: StorageBuffer<Vec<Vector3<f32>>, Mutable>,

    pub uniform: Uniform,
    pub spheres: Vec<Sphere>,

    pub last_window: Vector2<u32>,
    pub accumulate: bool,
}

impl Interactive for App {
    fn ui(&mut self, gcx: GraphicsCtx, ctx: &Context) {
        let window = gcx.window.inner_size();
        gcx.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
        gcx.window
            .set_cursor_position(PhysicalPosition::new(window.width / 2, window.height / 2))
            .unwrap();
        gcx.window.set_cursor_grab(CursorGrabMode::None).unwrap();

        if self.uniform.camera.handle_movement(&gcx, ctx) {
            self.uniform.accumulation_frame = 1;
        }

        Window::new("Ray Tracing")
            .default_width(0.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut self.uniform.samples, 1..=20));
                    ui.label("Samples");
                });

                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut self.uniform.max_bounces, 1..=100));
                    ui.label("Max Bounces");
                });

                ui.separator();

                ui.collapsing("Spheres", |ui| {
                    for (i, sphere) in self.spheres.iter_mut().enumerate() {
                        let heading = format!("Sphere #{}", i + 1);
                        ui.collapsing(&heading, |ui| {
                            Grid::new(&heading).num_columns(2).show(ui, |ui| {
                                ui.label("Position");
                                vec3_dragger(ui, &mut sphere.position, |x| x.speed(0.01));
                                ui.end_row();

                                ui.label("Radius");
                                ui.add(DragValue::new(&mut sphere.radius).speed(0.01));
                                ui.end_row();

                                ui.label("Roughness");
                                ui.add(Slider::new(&mut sphere.material.roughness, 0.0..=1.0));
                                ui.end_row();

                                let albedo = sphere.material.albedo;
                                let mut color = [albedo.x, albedo.y, albedo.z];
                                ui.label("Albedo");
                                ui.color_edit_button_rgb(&mut color);
                                sphere.material.albedo = Vector3::new(color[0], color[1], color[2]);
                                ui.end_row();

                                let emission = sphere.material.emission;
                                let mut color = [emission.x, emission.y, emission.z];
                                ui.label("Emission");
                                ui.color_edit_button_rgb(&mut color);
                                sphere.material.emission =
                                    Vector3::new(color[0], color[1], color[2]);
                                ui.end_row();
                            });
                        });
                    }

                    if ui.button("New").clicked() {
                        self.spheres.push(Sphere::default());
                    }
                });

                ui.collapsing("Camera", |ui| {
                    self.uniform.camera.ui(ui);
                });

                ui.separator();

                ui.checkbox(&mut self.accumulate, "Accumulate");
            });
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
        self.sphere_buffer.upload_shrink(&self.spheres).unwrap(); // todo: only on change

        self.pipeline.draw_quad(render_pass, 0..1);
    }
}
