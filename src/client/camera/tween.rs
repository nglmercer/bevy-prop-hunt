use avian3d::math::Dir;
use avian3d::parry::math::Pose3;
use avian3d::prelude::{Collider, PhysicsSystems, SpatialQuery, SpatialQueryFilter};
use bevy::color::palettes::css::RED;
use bevy::prelude::*;

use crate::shared::physics::PhysicsLayers;
use crate::shared::player::LocalPlayer;
use crate::utils::tween::{self, TransformTween};

use super::{CameraMode, FreeCamera, PlayerCamera};

pub struct SealedCameraTween;
pub type CameraTween = TransformTween<SealedCameraTween>;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSystemsSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (
            update_player_camera,
            update_fixed_player_camera,
            update_tween_player_camera,
            tween::update_tween::<SealedCameraTween>,
        )
            .chain()
            .after(PhysicsSystems::Last)
            .in_set(CameraSystemsSet),
    );
}

#[allow(clippy::type_complexity)]
fn update_player_camera(
    mut gizmos: Gizmos,
    raycaster: SpatialQuery,
    cam_mode: Res<State<CameraMode>>,
    mut camera: Single<&mut PlayerCamera>,
    freecamera: Single<&Transform, (With<FreeCamera>, Without<PlayerCamera>)>,
    player: Single<(&Transform, &Collider), (With<LocalPlayer>, Without<FreeCamera>)>,
) {
    match cam_mode.get() {
        CameraMode::Playing => {}
        CameraMode::Freecam => {
            camera.rot = Transform::default()
                .looking_at(player.0.translation - freecamera.translation, Vec3::Y)
                .rotation;
            let euler = camera.rot.to_euler(EulerRot::YXZ);
            camera.yaw = euler.0;
            camera.pitch = euler.1;
        }
    }

    let shape = player
        .1
        .shape_scaled()
        .compute_aabb(&Pose3::from_parts(Vec3::ZERO, player.0.rotation));

    let base_pos = player.0.translation + Vec3::new(0., shape.maxs.y, 0.);
    let camera_dir = camera.rot * Vec3::new(0., 2., 10.);

    let target_pos = base_pos + camera_dir;

    let Ok((dir, length)) = Dir::new_and_length(camera_dir) else {
        return;
    };

    let hit_data = raycaster.cast_ray(
        base_pos,
        dir,
        length,
        false,
        &SpatialQueryFilter::default().with_mask(PhysicsLayers::Map),
    );

    let (target_pos, player_distance) = match hit_data {
        Some(hit_data) => (base_pos + dir * hit_data.distance, hit_data.distance),
        None => (
            target_pos,
            vec3(base_pos.x, camera.target_pos.y, base_pos.z).distance(target_pos),
        ),
    };

    if *cam_mode.get() == CameraMode::Freecam {
        gizmos.sphere(target_pos, 0.5, RED);
    }

    camera.target_pos = target_pos;
    camera.player_distance = player_distance;
}

fn update_fixed_player_camera(
    mut camera: Single<(&mut Transform, &PlayerCamera), Without<TransformTween<SealedCameraTween>>>,
) {
    let (ref mut camera_trans, camera_player) = *camera;

    camera_trans.translation = camera_player.target_pos;
    camera_trans.rotation = camera_player.rot;
}

fn update_tween_player_camera(
    mut camera: Single<(&mut TransformTween<SealedCameraTween>, &PlayerCamera)>,
) {
    let (ref mut camera_tween, camera_player) = *camera;

    let target_transform = Transform {
        translation: camera_player.target_pos,
        rotation: camera_player.rot,
        ..default()
    };

    camera_tween.target = target_transform;
}
