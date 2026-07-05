use avian3d::math::*;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::cameras::CameraMode;
use crate::states::GameState;

use super::LocalPlayer;

pub fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        move_player.run_if(in_state(CameraMode::Playing).and_then(in_state(GameState::Running))),
    );
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut LinearVelocity, With<LocalPlayer>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vector2::new(horizontal as Scalar, vertical as Scalar).clamp_length_max(1.0);

    let delta_secs = time.delta_secs();

    if direction != Vector2::ZERO {
        player.x += direction.x * 50. * delta_secs;
        player.z -= direction.y * 50. * delta_secs;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        player.y = 7.;
    }
}
