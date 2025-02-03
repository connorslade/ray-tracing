use compute::{
    export::{
        egui::{Context, DragValue, Grid, Slider, Ui, Window},
        nalgebra::Vector3,
    },
    interactive::GraphicsCtx,
};

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
            ui.collapsing("Rendering", |ui| {
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut app.uniform.samples, 1..=20));
                    ui.label("Samples");
                });

                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut app.uniform.max_bounces, 1..=100));
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
    let old_spheres = hash(&app.models);
    for (i, model) in app.models.iter_mut().enumerate() {
        let heading = format!("Model #{}", i + 1);
        ui.collapsing(&heading, |ui| {
            material_settings(ui, &mut model.material);
        });
    }

    if hash(&app.spheres) != old_spheres {
        app.invalidate_accumulation();
        app.model_buffer.upload_shrink(&app.models).unwrap();
    }
}

fn material_settings(ui: &mut Ui, material: &mut Material) {
    Grid::new("material_settings")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Roughness");
            ui.add(Slider::new(&mut material.roughness, 0.0..=1.0));
            ui.end_row();

            let albedo = material.albedo;
            let mut color = [albedo.x, albedo.y, albedo.z];
            ui.label("Albedo");
            ui.color_edit_button_rgb(&mut color);
            material.albedo = Vector3::new(color[0], color[1], color[2]);
            ui.end_row();

            let mut color = [
                material.emission_color.x,
                material.emission_color.y,
                material.emission_color.z,
            ];

            ui.label("Emission");
            ui.horizontal(|ui| {
                ui.color_edit_button_rgb(&mut color);
                ui.add(DragValue::new(&mut material.emission_strength));
            });

            material.emission_color = Vector3::new(color[0], color[1], color[2]);
            ui.end_row();
        });
}
