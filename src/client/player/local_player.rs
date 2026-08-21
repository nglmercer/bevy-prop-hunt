use avian3d::math::*;
use avian3d::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::input::client::InputSystems;
use lightyear::prelude::Predicted;

use crate::client::{
    camera::{CameraMode, PlayerCamera, RADIANS_PER_DOT},
    states::ClientState,
};
use crate::shared::player::LocalPlayer;
use crate::shared::player::PlayerAction;
use crate::shared::player::movement::PropPhysics;
use crate::shared::player::movement::move_player;

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            update_look
                .run_if(in_state(CameraMode::Playing).and_then(in_state(ClientState::Running))),
            handle_player_actions.run_if(in_state(ClientState::Running)),
        ),
    )
    .add_systems(
        FixedPreUpdate,
        (update_player_actions
            .before(InputSystems::BufferClientInputs)
            .run_if(in_state(CameraMode::Playing).and_then(in_state(ClientState::Running))),)
            .chain(),
    );
}

fn update_player_actions(
    camera: Single<&PlayerCamera>,
    mut input: Single<&mut ActionState<PlayerAction>, With<LocalPlayer>>,
) {
    let dir = input.clamped_axis_pair(&PlayerAction::Move);
    let dir = Vector2::from_angle(camera.yaw).rotate(dir);

    input.set_axis_pair(&PlayerAction::Move, dir);
}

fn handle_player_actions(
    time: Res<Time>,
    raycast: SpatialQuery,
    mut query: Query<(Entity, &ActionState<PlayerAction>, PropPhysics), With<Predicted>>,
    // is_server: Query<(), With<Server>>,
) {
    // if !is_server.is_empty() {
    //     return;
    // }

    for (entity, action_state, physics) in &mut query {
        move_player(&time, &raycast, entity, action_state, physics);
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
