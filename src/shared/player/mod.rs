use bevy::prelude::*;
use lightyear::prelude::PeerId;
use serde::{Deserialize, Serialize};

#[derive(Component, Deserialize, Serialize, Clone, Copy)]
pub struct Player(pub PeerId);

#[derive(Component, Default, Clone, Copy)]
pub struct LocalPlayer;

#[derive(Component, Default, Clone, Copy)]
pub struct RemotePlayer;
