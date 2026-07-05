use std::time::Duration;

use bevy::prelude::*;

use crate::lenses::smooth_transform_lerp;
use crate::player::LocalPlayer;

use super::PlayerCamera;

pub fn plugin(app: &mut App) {
    app.add_systems(Last, (update_fixed_player_camera, update_player_camera));
}

fn update_fixed_player_camera(
    mut camera: Single<
        &mut Transform,
        (
            With<PlayerCamera>,
            Without<LocalPlayer>,
            Without<CameraTween>,
        ),
    >,
    player: Single<&Transform, (Without<PlayerCamera>, With<LocalPlayer>)>,
) {
    // TODO: Raycast (Walls layers) to target camera position and put the camera on hit pos
    let target_pos = player.translation + Vec3::new(0., 4., 10.);
    camera.translation = target_pos;
}

#[derive(Component, Debug, Clone)]
pub struct CameraTween {
    pub reference: Transform,
    pub time: Duration,
    pub duration: Duration,
}

fn update_player_camera(
    mut commands: Commands,
    time: Res<Time>,
    mut camera: Single<
        (Entity, &mut Transform, &mut CameraTween),
        (With<PlayerCamera>, Without<LocalPlayer>),
    >,
    player: Single<&Transform, (Without<PlayerCamera>, With<LocalPlayer>)>,
) {
    // TODO: Raycast (Walls layers) to target camera position and put the camera on hit pos
    let target_pos = player.translation + Vec3::new(0., 4., 10.);

    let (entity, ref mut transform, ref mut camera_tween) = *camera;

    camera_tween.time += time.delta();

    if camera_tween.time > camera_tween.duration {
        commands.entity(entity).remove::<CameraTween>();
        return;
    }

    let mut target_transform = Transform::default();
    target_transform.translation = target_pos;

    smooth_transform_lerp(
        transform,
        &camera_tween.reference,
        &target_transform,
        camera_tween.time.as_secs_f32() / camera_tween.duration.as_secs_f32(),
    );
}
