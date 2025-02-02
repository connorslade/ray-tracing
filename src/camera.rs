use std::f32::consts::{FRAC_PI_2, TAU};

use compute::{
    export::{
        egui::{Context, Key, Ui},
        nalgebra::{Vector2, Vector3},
    },
    interactive::GraphicsCtx,
};
use encase::ShaderType;

use crate::misc::{dragger, vec3_dragger};

#[derive(ShaderType, Clone, PartialEq)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,

    pub fov: f32,
    pub aspect: f32,
}

impl Camera {
    fn direction(&self) -> Vector3<f32> {
        Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }
}

impl Camera {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            vec3_dragger(ui, &mut self.position, |x| x);
            ui.label("Position");
        });
        dragger(ui, "Pitch", &mut self.pitch, |x| x);
        dragger(ui, "Yaw", &mut self.yaw, |x| x);
        dragger(ui, "Fov", &mut self.fov, |x| x.speed(0.01));
    }

    pub fn handle_movement(&mut self, gcx: &GraphicsCtx, ctx: &Context) -> bool {
        let old_camera = self.clone();

        let dragging_viewport = ctx.dragged_id().is_none();
        let delta_time = ctx.input(|x| x.stable_dt);

        let scale_factor = gcx.window.scale_factor() as f32;
        let window = gcx.window.inner_size();
        self.aspect = window.width as f32 / window.height as f32;

        ctx.input(|input| {
            if input.pointer.any_down() && dragging_viewport {
                let pos = input.pointer.latest_pos().unwrap_or_default() * scale_factor * 2.0;
                let delta = input.pointer.delta() * scale_factor;

                if pos.x == window.width as f32 || pos.y == window.height as f32 {
                    return;
                }
                let delta = Vector2::new(delta.x, delta.y);

                self.yaw -= delta.x * 0.002;
                self.pitch -= delta.y * 0.002;

                self.yaw = self.yaw.rem_euclid(TAU);
                self.pitch = self.pitch.clamp(-FRAC_PI_2 + 0.001, FRAC_PI_2 - 0.001);
            }
        });

        let forward = self.direction();
        let right = -Vector3::new(-forward.z, 0.0, forward.x).normalize();
        let up = Vector3::y();

        let (w, a, s, d, space, shift, ctrl) = ctx.input(|x| {
            (
                x.key_down(Key::W),
                x.key_down(Key::A),
                x.key_down(Key::S),
                x.key_down(Key::D),
                x.key_down(Key::Space),
                x.modifiers.shift,
                x.modifiers.ctrl,
            )
        });

        let mut velocity = Vector3::zeros();
        let speed = if ctrl { 4.0 } else { 2.0 } * delta_time;

        for (key, dir) in [
            (w, forward),
            (s, -forward),
            (d, right),
            (a, -right),
            (space, up),
            (shift, -up),
        ] {
            velocity += dir * key as u8 as f32;
        }

        if velocity.norm_squared() > 0.0 {
            self.position += velocity.normalize() * speed;
        }

        self != &old_camera
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector3::zeros(),
            pitch: 0.0,
            yaw: 0.0,

            fov: FRAC_PI_2,
            aspect: 0.0,
        }
    }
}
