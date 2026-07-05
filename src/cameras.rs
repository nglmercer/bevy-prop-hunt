use bevy::prelude::*;

mod control;
mod freecam;
mod tween;

pub fn plugins(app: &mut App) {
    app.add_plugins((freecam::plugin, tween::plugin, control::plugin));
}

#[derive(Component, Clone, Copy, FromTemplate)]
pub struct PlayerCamera;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraMode {
    Playing,
    Freecam,
}

#[derive(Component, Default, Clone, Copy)]
pub struct FreeCamera;

#[derive(Component, Default, Clone, Copy)]
pub struct CurrentCamera;
