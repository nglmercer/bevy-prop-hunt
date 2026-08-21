use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::prelude::*;
use lightyear::input::config::InputConfig;
use lightyear::prelude::input::leafwing;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::shared::player::{Player, PlayerAction};

pub struct MorphChannel;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct MorphRequest {
    pub target: Entity,
}

impl MapEntities for MorphRequest {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.target = entity_mapper.get_mapped(self.target);
    }
}

pub fn plugin(app: &mut App) {
    app.component::<Player>().replicate_once();

    app.add_plugins(leafwing::InputPlugin::<PlayerAction> {
        config: InputConfig::<PlayerAction> {
            rebroadcast_inputs: false,
            ..default()
        },
    });

    app.add_channel::<MorphChannel>(ChannelSettings {
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
        ..default()
    })
    .add_direction(NetworkDirection::ClientToServer);

    app.register_message::<MorphRequest>()
        .add_map_entities()
        .add_direction(NetworkDirection::ClientToServer);
}
