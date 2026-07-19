use std::time::Duration;

use bevy::camera_controller::free_camera::FreeCameraState;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_tweening::{AnimCompletedEvent, Tween};

use crate::client::states::ClientState;
use crate::utils::lenses::{SmoothTransformLens, TweenCommands};

use super::tween::CameraTween;
use super::{CameraMode, CurrentCamera, FreeCamera, PlayerCamera};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            enable_freecam.run_if(in_state(CameraMode::Playing)),
            disable_freecam.run_if(in_state(CameraMode::Freecam)),
        )
            .run_if(in_state(ClientState::Running).and_then(input_just_pressed(KeyCode::Tab))),
    );
}

type OnlyPlayerCamera = (With<PlayerCamera>, Without<FreeCamera>);
type OnlyFreeCamera = (Without<PlayerCamera>, With<FreeCamera>);

type CameraBundle<'a> = (&'a mut Camera, Entity, &'a Transform);

fn calculate_tween_duration(player_transform: &Transform, debug_transform: &Transform) -> u64 {
    let camera_distance = player_transform
        .translation
        .distance(debug_transform.translation);

    (camera_distance.sqrt() * 100.).min(400.).max(200.) as u64
}

fn enable_freecam(
    mut commands: Commands,
    mut player_camera: Single<CameraBundle, OnlyPlayerCamera>,
    mut freecam: Single<(CameraBundle, Option<&mut FreeCameraState>), OnlyFreeCamera>,
) {
    let (ref mut player_camera, player_entity, player_transform) = *player_camera;
    let ((ref mut freecam, freecam_entity, _freecam_transform), ref mut freecam_state) = *freecam;

    if freecam.is_active {
        return;
    }

    commands.set_state(CameraMode::Freecam);
    player_camera.is_active = false;
    freecam.is_active = true;

    commands.entity(player_entity).try_remove::<CurrentCamera>();
    commands.entity(freecam_entity).insert(CurrentCamera);

    let mut end_transform = *player_transform;
    end_transform.translation =
        player_transform.translation + *player_transform.forward() + *player_transform.up();

    if let Some(state) = freecam_state {
        let (yaw, pitch, _roll) = end_transform.rotation.to_euler(EulerRot::YXZ);
        state.yaw = yaw;
        state.pitch = pitch;
    } else {
        let (yaw, pitch, _roll) = player_transform.rotation.to_euler(EulerRot::YXZ);
        let mut state = FreeCameraState::default();
        state.yaw = yaw;
        state.pitch = pitch;

        commands.entity(freecam_entity).insert(state);
    }

    let tween_duration = calculate_tween_duration(player_transform, &end_transform);

    commands
        .entity(freecam_entity)
        .insert(*player_transform)
        .tween_component::<Transform>(
            Tween::new(
                EaseFunction::Linear,
                Duration::from_millis(tween_duration),
                SmoothTransformLens::new(*player_transform, end_transform),
            )
            .with_cycle_completed_event(true),
        )
        .observe(enable_freecam_controls);

    fn enable_freecam_controls(
        _: On<AnimCompletedEvent>,
        mut commands: Commands,
        mut debug_camera: Single<(Entity, &mut FreeCameraState), With<FreeCamera>>,
    ) {
        commands
            .entity(debug_camera.0)
            .insert_if_new(bevy::camera_controller::free_camera::FreeCamera { ..default() });

        debug_camera.1.enabled ^= true;
    }
}

fn disable_freecam(
    mut commands: Commands,
    mut player_camera: Single<CameraBundle, OnlyPlayerCamera>,
    mut freecamera: Single<(CameraBundle, Option<&mut FreeCameraState>), OnlyFreeCamera>,
) {
    let (ref mut player_camera, player_entity, player_transform) = *player_camera;
    let ((ref mut freecam, freecam_entity, freecam_transform), ref mut freecam_state) = *freecamera;

    if !freecam.is_active {
        return;
    }

    commands.set_state(CameraMode::Playing);
    player_camera.is_active = true;
    freecam.is_active = false;

    commands.entity(player_entity).insert(CurrentCamera);
    commands
        .entity(freecam_entity)
        .try_remove::<CurrentCamera>();

    let tween_duration = calculate_tween_duration(player_transform, freecam_transform);

    commands
        .entity(player_entity)
        .insert(*freecam_transform)
        .insert_if_new(CameraTween {
            reference: *freecam_transform,
            duration: Duration::from_millis(tween_duration),
            ..default()
        });

    if let Some(state) = freecam_state {
        state.enabled = false;
    }
}
