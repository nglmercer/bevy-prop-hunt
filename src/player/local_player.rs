use avian3d::math::*;
use avian3d::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;

use crate::cameras::CameraMode;
use crate::cameras::PlayerCamera;
use crate::cameras::RADIANS_PER_DOT;
use crate::states::GameState;

use super::LocalPlayer;

pub fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        (move_player, update_look)
            .run_if(in_state(CameraMode::Playing).and_then(in_state(GameState::Running))),
    );
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera: Single<&PlayerCamera>,
    raycast: SpatialQuery,
    mut player: Single<(Entity, &Transform, &mut LinearVelocity, &Collider), With<LocalPlayer>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vector2::from_angle(camera.yaw)
        .rotate(Vector2::new(horizontal as Scalar, vertical as Scalar).clamp_length_max(1.0));

    let delta_secs = time.delta_secs();

    if direction != Vector2::ZERO {
        player.2.x += direction.x * 50. * delta_secs;
        player.2.z -= direction.y * 50. * delta_secs;

        let out = player.2.xz().clamp_length_max(20.);
        player.2.x = out.x;
        player.2.z = out.y;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        let hit_data = raycast.cast_shape(
            player.3,
            player.1.translation,
            player.1.rotation,
            Dir3::NEG_Y,
            &ShapeCastConfig {
                max_distance: 0.1,
                compute_contact_on_penetration: false,
                ..default()
            },
            &SpatialQueryFilter::default().with_excluded_entities([player.0]),
        );

        if hit_data.is_some() {
            player.2.y = 10.;
        }
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
