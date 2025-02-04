use std::{fs::File, time::Instant};

use compute::{
    export::{
        egui::{Context, DragValue, Grid, Slider, Ui, Window},
        nalgebra::Vector3,
    },
    interactive::GraphicsCtx,
};
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};

use crate::{
    app::App,
    misc::{hash, vec3_dragger},
    types::{Material, Sphere},
};

pub fn ui(app: &mut App, gcx: GraphicsCtx, ctx: &Context) {
    let old_camera = hash(&app.uniform.camera);
    app.uniform.camera.handle_movement(&gcx, ctx);

    Window::new("Ray Tracing")
        .default_width(0.0)
        .show(ctx, |ui| {
            ui.label(format!(
                "FPS: {}",
                app.last_frame.elapsed().as_secs_f32().recip()
            ));
            app.last_frame = Instant::now();
            ui.separator();

            ui.collapsing("Rendering", |ui| {
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut app.uniform.samples, 1..=20));
                    ui.label("Samples");
                });

                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut app.uniform.max_bounces, 2..=100));
                    ui.label("Max Bounces");
                });

                ui.checkbox(&mut app.accumulate, "Accumulate");

                ui.separator();

                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut app.uniform.environment, 0.0..=1.0));
                    ui.label("Environment");
                });
            });

            ui.collapsing("Spheres", |ui| sphere_settings(app, ui));
            ui.collapsing("Models", |ui| model_settings(app, ui));
            ui.collapsing("Camera", |ui| app.uniform.camera.ui(ui));

            ui.separator();

            if ui.button("Capture").clicked() {
                let window = gcx.window.inner_size();
                app.accumulation_buffer.download_async(move |data| {
                    let encoder = PngEncoder::new(File::create("out.png").unwrap());
                    let data = data
                        .iter()
                        .map(|x| (x * 255.0).map(|x| x as u8))
                        .flat_map(|x| [x.x, x.y, x.z])
                        .collect::<Vec<_>>();

                    encoder
                        .write_image(&data, window.width, window.height, ExtendedColorType::Rgb8)
                        .unwrap();
                });
            }
        });

    if hash(&app.uniform.camera) != old_camera {
        app.invalidate_accumulation();
    }
}

fn sphere_settings(app: &mut App, ui: &mut Ui) {
    let old_spheres = hash(&app.spheres);
    let mut delete = None;

    for (i, sphere) in app.spheres.iter_mut().enumerate() {
        let heading = format!("Sphere #{}", i + 1);
        ui.collapsing(&heading, |ui| {
            Grid::new(&heading).num_columns(2).show(ui, |ui| {
                ui.label("Position");
                vec3_dragger(ui, &mut sphere.position, |x| x.speed(0.01));
                ui.end_row();

                ui.label("Radius");
                ui.add(DragValue::new(&mut sphere.radius).speed(0.01));
                ui.end_row();
            });

            ui.separator();
            material_settings(ui, &mut sphere.material);

            ui.separator();
            if ui.button("Delete").clicked() {
                delete = Some(i);
            }
        });
    }

    if let Some(delete) = delete {
        app.spheres.remove(delete);
    }

    if ui.button("New").clicked() {
        app.spheres.push(Sphere::default());
    }

    if hash(&app.spheres) != old_spheres {
        app.invalidate_accumulation();
        app.sphere_buffer.upload_shrink(&app.spheres).unwrap();
    }
}

fn model_settings(app: &mut App, ui: &mut Ui) {
    let old_models = hash(&app.models);
    for model in app.models.iter_mut() {
        ui.collapsing(&model.name, |ui| {
            Grid::new(&model.name).num_columns(2).show(ui, |ui| {
                ui.label("Position");
                vec3_dragger(ui, &mut model.position, |x| x.speed(0.01));
                ui.end_row();

                ui.label("Scale");
                vec3_dragger(ui, &mut model.scale, |x| x.speed(0.01));
                ui.end_row();
            });

            ui.separator();

            material_settings(ui, &mut model.material);
        });
    }

    if hash(&app.models) != old_models {
        app.invalidate_accumulation();
        app.upload_models();
    }
}

fn material_settings(ui: &mut Ui, material: &mut Material) {
    Grid::new("material_settings")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Roughness");
            ui.add(Slider::new(&mut material.roughness, 0.0..=1.0));
            ui.end_row();

            ui.label("Specular Probability");
            ui.add(Slider::new(&mut material.specular_probability, 0.0..=1.0));
            ui.end_row();

            ui.label("Diffuse Color");
            let diffuse_color = material.diffuse_color;
            let mut color = [diffuse_color.x, diffuse_color.y, diffuse_color.z];
            ui.color_edit_button_rgb(&mut color);
            material.diffuse_color = Vector3::new(color[0], color[1], color[2]);
            ui.end_row();

            ui.label("Specular Color");
            let specular_color = material.specular_color;
            let mut color = [specular_color.x, specular_color.y, specular_color.z];
            ui.color_edit_button_rgb(&mut color);
            material.specular_color = Vector3::new(color[0], color[1], color[2]);
            ui.end_row();

            let emission_color = material.emission_color;
            let mut color = [emission_color.x, emission_color.y, emission_color.z];

            ui.label("Emission");
            ui.horizontal(|ui| {
                ui.color_edit_button_rgb(&mut color);
                ui.add(DragValue::new(&mut material.emission_strength));
            });

            material.emission_color = Vector3::new(color[0], color[1], color[2]);
            ui.end_row();
        });
}
