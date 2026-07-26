use bevy::prelude::*;
use lightyear::prelude::LocalId;

use crate::shared::network::LocalClient;
use crate::shared::player::{LocalPlayer, Player, RemotePlayer};

pub mod local_player;
pub mod morphing;

pub fn plugin(app: &mut App) {
    app.add_plugins((local_player::plugin, morphing::plugin))
        .add_observer(identify_players);
}

fn identify_players(
    trigger: On<Add, Player>,
    mut commands: Commands,
    local_client: Single<&LocalId, With<LocalClient>>,
    query: Query<&Player>,
) {
    let Ok(player) = query.get(trigger.entity) else {
        return;
    };

    if local_client.0 == player.0 {
        commands.entity(trigger.entity).insert(LocalPlayer);
    } else {
        commands.entity(trigger.entity).insert(RemotePlayer);
    }
}
