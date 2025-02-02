use compute::{
    buffer::{StorageBuffer, UniformBuffer},
    export::{
        egui::{Context, DragValue, Grid, Slider, Window},
        nalgebra::Vector3,
        wgpu::RenderPass,
        winit::{dpi::PhysicalPosition, window::CursorGrabMode},
    },
    interactive::{GraphicsCtx, Interactive},
    misc::mutability::Immutable,
    pipeline::render::RenderPipeline,
};

use crate::{
    misc::{dragger, vec3_dragger},
    types::{Sphere, Uniform},
};

pub struct App {
    pub pipeline: RenderPipeline,
    pub uniform_buffer: UniformBuffer<Uniform>,
    pub sphere_buffer: StorageBuffer<Vec<Sphere>, Immutable>,

    pub uniform: Uniform,
    pub spheres: Vec<Sphere>,
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
            });
    }

    fn render(&mut self, _gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        self.uniform.frame += 1;
        self.uniform_buffer.upload(&self.uniform).unwrap();
        self.sphere_buffer.upload_shrink(&self.spheres).unwrap(); // todo: only on change

        self.pipeline.draw_quad(render_pass, 0..1);
    }
}
