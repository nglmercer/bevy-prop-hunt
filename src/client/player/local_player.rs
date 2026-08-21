use avian3d::math::*;
use avian3d::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use leafwing_input_manager::prelude::{ActionState, InputMap};
use lightyear::input::client::InputSystems;
use lightyear::prelude::{Predicted, Server};

use crate::client::{
    camera::{CameraMode, PlayerCamera, RADIANS_PER_DOT},
    states::ClientState,
};
use crate::shared::player::PlayerAction;
use crate::shared::player::movement::move_player;
use crate::shared::player::movement::{JumpState, PropPhysics};
use crate::shared::player::{LocalPlayer, Player};

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            update_look
                .run_if(in_state(CameraMode::Playing).and_then(in_state(ClientState::Running))),
            handle_player_actions
                .run_if(in_state(CameraMode::Playing).and_then(in_state(ClientState::Running))),
        ),
    )
    .add_systems(Update, diag_leafwing_input)
    .add_systems(
        FixedPreUpdate,
        (
            gate_player_input,
            update_player_actions
                .run_if(in_state(CameraMode::Playing).and_then(in_state(ClientState::Running))),
        )
            .before(InputSystems::BufferClientInputs)
            .chain(),
    );
}

// DIAG: temporary. Distinguishes "leafwing never captures input (A)" from
// "leafwing captures it but server's re-write drops the edge (B)" on the host.
#[allow(clippy::type_complexity)]
fn diag_leafwing_input(
    players: Query<
        (
            Entity,
            &Player,
            &ActionState<PlayerAction>,
            Has<InputMap<PlayerAction>>,
        ),
        With<LocalPlayer>,
    >,
) {
    // Additionally print once if a local player exists at all but lacks an
    // InputMap (possibility A: no input capture on the host entity).
    for (entity, player, action_state, has_map) in &players {
        if action_state.just_pressed(&PlayerAction::Jump) {
            println!(
                "[input-diag] LEAFWING local player entity={entity:?} peer={:?} has_inputmap={has_map} just_pressed(Jump)",
                player.0
            );
        }
    }
}

fn gate_player_input(
    client_state: Res<State<ClientState>>,
    camera_mode: Res<State<CameraMode>>,
    mut input: Query<&mut ActionState<PlayerAction>, With<LocalPlayer>>,
) {
    let enabled =
        *client_state.get() == ClientState::Running && *camera_mode.get() == CameraMode::Playing;

    for mut input in &mut input {
        if enabled {
            input.enable();
        } else {
            input.reset_all();
            input.disable();
        }
    }
}

fn update_player_actions(
    camera: Option<Single<&PlayerCamera>>,
    input: Option<Single<&mut ActionState<PlayerAction>, With<LocalPlayer>>>,
) {
    let (Some(camera), Some(mut input)) = (camera, input) else {
        return;
    };

    let dir = input.clamped_axis_pair(&PlayerAction::Move);
    let dir = Vector2::from_angle(camera.yaw).rotate(dir);

    input.set_axis_pair(&PlayerAction::Move, dir);
}

#[allow(clippy::type_complexity)]
fn handle_player_actions(
    time: Res<Time>,
    raycast: SpatialQuery,
    servers: Query<(), With<Server>>,
    mut query: Query<
        (
            Entity,
            &ActionState<PlayerAction>,
            &mut JumpState,
            PropPhysics,
        ),
        (With<Predicted>, With<LocalPlayer>),
    >,
) {
    // A host-server shares the ECS world with its local client. The server
    // simulation below is authoritative there, so do not run client
    // prediction on the same world or the host would be simulated twice.
    if !servers.is_empty() {
        return;
    }

    for (entity, action_state, mut jump_state, physics) in &mut query {
        move_player(
            &time,
            &raycast,
            entity,
            action_state,
            &mut jump_state,
            physics,
        );
    }
}

fn update_look(
    mut camera: Single<&mut PlayerCamera>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if accumulated_mouse_motion.delta != Vec2::ZERO {
        camera.pitch = (camera.pitch - accumulated_mouse_motion.delta.y * RADIANS_PER_DOT)
            .clamp(-PI / 2., PI / 2.);
        camera.yaw -= accumulated_mouse_motion.delta.x * RADIANS_PER_DOT;
        camera.rot = Quat::from_euler(EulerRot::ZYX, 0.0, camera.yaw, camera.pitch);
    }
}
