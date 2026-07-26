use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Deserialize, Serialize, Clone)]
pub struct MagicTrailParticles {
    pub from: Transform,
    pub following: Entity,
}
