use bevy::prelude::*;

pub mod local_player;
pub mod morphing;

pub fn plugins(app: &mut App) {
    app.add_plugins((local_player::plugin, morphing::plugin));
}

#[derive(Component, Default, Clone, Copy)]
pub struct Player;

#[derive(Component, Default, Clone, Copy)]
pub struct LocalPlayer;

#[derive(Component, Default, Clone, Copy)]
pub struct PeerPlayer;
