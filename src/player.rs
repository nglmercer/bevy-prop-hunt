use bevy::prelude::*;

pub mod local_player;

#[derive(Component, Default, Clone, Copy)]
pub struct Player;

#[derive(Component, Default, Clone, Copy)]
pub struct LocalPlayer;

#[derive(Component, Default, Clone, Copy)]
pub struct PeerPlayer;
