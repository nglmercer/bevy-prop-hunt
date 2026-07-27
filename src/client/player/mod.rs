use bevy::prelude::*;
use leafwing_input_manager::prelude::{GamepadStick, InputMap, VirtualDPad};
use lightyear::prelude::LocalId;

use crate::shared::network::LocalClient;
use crate::shared::player::{LocalPlayer, Player, PlayerAction, RemotePlayer};

pub mod local_player;
pub mod morphing;

pub fn plugin(app: &mut App) {
    app.add_plugins((local_player::plugin, morphing::plugin))
        .add_observer(identify_players)
        .add_observer(clean_player);
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
        commands.entity(trigger.entity).insert((
            LocalPlayer,
            InputMap::<PlayerAction>::default()
                .with(PlayerAction::Jump, KeyCode::Space)
                .with(PlayerAction::Jump, GamepadButton::South)
                .with(PlayerAction::Morph, MouseButton::Left)
                .with(PlayerAction::Morph, GamepadButton::East)
                .with_dual_axis(PlayerAction::Move, VirtualDPad::wasd())
                .with_dual_axis(PlayerAction::Move, VirtualDPad::arrow_keys())
                .with_dual_axis(PlayerAction::Move, GamepadStick::LEFT),
        ));
    } else {
        commands.entity(trigger.entity).insert(RemotePlayer);
    }
}

fn clean_player(trigger: On<Remove, Player>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .remove::<InputMap<PlayerAction>>();
}
