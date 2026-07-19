use bevy::prelude::*;

mod control;
mod freecam;
pub mod tween;

pub use freecam::RADIANS_PER_DOT;

pub fn plugins(app: &mut App) {
    app.add_plugins((freecam::plugin, tween::plugin, control::plugin));
}

#[derive(Component, Clone, Copy, FromTemplate)]
pub struct PlayerCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub rot: Quat,
    pub target_pos: Vec3,
    pub player_distance: f32,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraMode {
    Playing,
    Freecam,
}

#[derive(Component, Default, Clone, Copy)]
pub struct FreeCamera;

#[derive(Component, Default, Clone, Copy)]
pub struct CurrentCamera;
