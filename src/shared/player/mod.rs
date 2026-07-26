use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Deserialize, Serialize, Default, Clone, Copy)]
pub struct Player;

#[derive(Component, Default, Clone, Copy)]
pub struct LocalPlayer;
