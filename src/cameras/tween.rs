use std::time::Duration;

use avian3d::math::Dir;
use avian3d::prelude::{PhysicsSystems, SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use crate::lenses::smooth_transform_lerp;
use crate::physics::PhysicsLayers;
use crate::player::LocalPlayer;

use super::{CameraMode, FreeCamera, PlayerCamera};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSystemsSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedPostUpdate,
        (
            update_player_camera,
            update_fixed_player_camera,
            update_tween_player_camera,
        )
            .chain()
            .after(PhysicsSystems::Last)
            .in_set(CameraSystemsSet),
    );
}

fn update_player_camera(
    raycaster: SpatialQuery,
    cam_mode: Res<State<CameraMode>>,
    mut camera: Single<&mut PlayerCamera>,
    freecamera: Single<&Transform, (With<FreeCamera>, Without<PlayerCamera>)>,
    player: Single<&Transform, (With<LocalPlayer>, Without<FreeCamera>)>,
) {
    match cam_mode.get() {
        CameraMode::Playing => {}
        CameraMode::Freecam => {
            camera.rot = Transform::default()
                .looking_at(player.translation - freecamera.translation, Vec3::Y)
                .rotation;
            let euler = camera.rot.to_euler(EulerRot::YXZ);
            camera.yaw = euler.0;
            camera.pitch = euler.1;
        }
    }

    let base_pos = player.translation + Vec3::new(0., 2., 0.);
    let camera_dir = camera.rot * Vec3::new(0., 2., 10.);

    let target_pos = base_pos + camera_dir;

    let Ok((dir, length)) = Dir::new_and_length(camera_dir) else {
        return;
    };

    let Some(hit_data) = raycaster.cast_ray(
        base_pos,
        dir,
        length,
        false,
        &SpatialQueryFilter::default().with_mask(PhysicsLayers::Map),
    ) else {
        camera.target_pos = target_pos;
        return;
    };

    camera.target_pos = base_pos + dir * hit_data.distance;
}

fn update_fixed_player_camera(
    mut camera: Single<(&mut Transform, &PlayerCamera), Without<CameraTween>>,
) {
    let (ref mut camera_trans, camera_player) = *camera;

    camera_trans.translation = camera_player.target_pos;
    camera_trans.rotation = camera_player.rot;
}

#[derive(Component, Debug, Clone)]
pub(super) struct CameraTween {
    pub reference: Transform,
    pub time: Duration,
    pub duration: Duration,
}

fn update_tween_player_camera(
    mut commands: Commands,
    time: Res<Time>,
    mut camera: Single<(Entity, &mut Transform, &mut CameraTween, &PlayerCamera)>,
) {
    let (entity, ref mut transform, ref mut camera_tween, camera_player) = *camera;

    camera_tween.time += time.delta();

    if camera_tween.time > camera_tween.duration {
        commands.entity(entity).remove::<CameraTween>();
        return;
    }

    let mut target_transform = Transform::default();
    target_transform.translation = camera_player.target_pos;
    target_transform.rotation = camera_player.rot;

    smooth_transform_lerp(
        transform,
        &camera_tween.reference,
        &target_transform,
        camera_tween.time.as_secs_f32() / camera_tween.duration.as_secs_f32(),
    );
}
