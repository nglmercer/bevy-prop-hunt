use bevy::prelude::*;
use leafwing_input_manager::{Actionlike, InputControlKind};
use lightyear::prelude::PeerId;
use serde::{Deserialize, Serialize};

pub mod movement;

#[derive(Component, Deserialize, Serialize, Clone, Copy)]
pub struct Player(pub PeerId);

#[derive(Component, Default, Clone, Copy)]
pub struct LocalPlayer;

#[derive(Component, Default, Clone, Copy)]
pub struct RemotePlayer;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect, Serialize, Deserialize)]
pub enum PlayerAction {
    Move,
    Jump,
    Morph,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
            Self::Jump => InputControlKind::Button,
            Self::Morph => InputControlKind::Button,
        }
    }
}
