use bevy::prelude::*;

pub mod local_player;
pub mod morphing;

pub fn plugin(app: &mut App) {
    app.add_plugins((local_player::plugin, morphing::plugin));
}
