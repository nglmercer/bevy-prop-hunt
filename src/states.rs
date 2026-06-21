use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
