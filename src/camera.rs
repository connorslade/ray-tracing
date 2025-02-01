use std::f32::consts::FRAC_PI_2;

use compute::{
    export::{
        egui::{Context, Key, Ui},
        nalgebra::{Vector2, Vector3},
    },
    interactive::GraphicsCtx,
};
use encase::ShaderType;

use crate::misc::{dragger, vec3_dragger};

#[derive(Clone, ShaderType)]
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
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
        )
    }
}

impl Camera {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Position");
            vec3_dragger(ui, &mut self.position, |x| x);
        });
        dragger(ui, "Pitch", &mut self.pitch, |x| x);
        dragger(ui, "Yaw", &mut self.yaw, |x| x);
        dragger(ui, "Fov", &mut self.fov, |x| x.speed(0.01));
    }

    pub fn handle_movement(&mut self, gcx: &GraphicsCtx, ctx: &Context) {
        let scale_factor = gcx.window.scale_factor() as f32;
        let window = gcx.window.inner_size();
        self.aspect = window.width as f32 / window.height as f32;

        let dragging_viewport = ctx.dragged_id().is_none();

        ctx.input(|input| {
            if input.pointer.any_down() && dragging_viewport {
                let delta = input.pointer.delta() * scale_factor;
                let delta = Vector2::new(delta.x, delta.y);

                self.pitch -= delta.y * 0.01;
                self.yaw -= delta.x * 0.01;

                const EPSILON: f32 = 0.0001;
                self.pitch = self.pitch.clamp(-FRAC_PI_2 + EPSILON, FRAC_PI_2 - EPSILON);
            }
        });

        let direction = self.direction();
        let (w, a, s, d, space, shift, crtl) = ctx.input(|x| {
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

        let speed = if crtl { 0.4 } else { 0.2 };

        self.position += direction * speed * (w as i32 - s as i32) as f32;
        self.position += direction.cross(&Vector3::z()) * speed * (d as i32 - a as i32) as f32;
        self.position += Vector3::z() * speed * (space as i32 - shift as i32) as f32;
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
