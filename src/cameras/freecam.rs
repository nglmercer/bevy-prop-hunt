use std::f32::consts::PI;

use bevy::camera_controller::free_camera::{
    FreeCamera, FreeCameraState, VerticalMovementAxis, rotate_freecam_to,
};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::math::ops::exp;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

use crate::pause_menu::PauseState;
use crate::states::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        (
            run_freecamera_controller.run_if(in_state(GameState::Running)),
            rotate_freecam_to.run_if(not(in_state(PauseState::Paused))),
        )
            .chain()
            .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
    );
}

/// Scales mouse motion into yaw/pitch movement.
///
/// Based on Valorant's default sensitivity, not entirely sure why it is exactly 1.0 / 180.0,
/// but we're guessing it is a misunderstanding between degrees/radians and then sticking with
/// it because it felt nice.
pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

// Modified version from [bevy_camera_controller]
/// Updates the camera's position and orientation based on user input.
///
/// - [`FreeCamera`] contains static configuration such as key bindings, movement speed, and sensitivity.
/// - [`FreeCameraState`] stores the dynamic runtime state, including pitch, yaw, velocity, and enable flags.
///
/// This system is typically added via the [`FreeCameraPlugin`].
///
/// Axis snapping takes priority over mouse movement.
fn run_freecamera_controller(
    time: Res<Time<Real>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    touch_input: Res<Touches>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut FreeCameraState, &FreeCamera), With<Camera>>,
) {
    let dt = time.delta_secs();

    let Ok((mut transform, mut state, config)) = query.single_mut() else {
        return;
    };

    // Handle key input
    let mut axis_input = Vec3::ZERO;
    if key_input.pressed(config.key_forward) {
        axis_input.z += 1.0;
    }
    if key_input.pressed(config.key_back) {
        axis_input.z -= 1.0;
    }
    if key_input.pressed(config.key_right) {
        axis_input.x += 1.0;
    }
    if key_input.pressed(config.key_left) {
        axis_input.x -= 1.0;
    }
    if key_input.pressed(config.key_up) {
        axis_input.y += 1.0;
    }
    if key_input.pressed(config.key_down) {
        axis_input.y -= 1.0;
    }

    // Update velocity
    if axis_input != Vec3::ZERO {
        let max_speed = if key_input.pressed(config.key_run) {
            config.run_speed * state.speed_multiplier
        } else {
            config.walk_speed * state.speed_multiplier
        };
        state.velocity = axis_input.normalize() * max_speed;
    } else {
        let friction = config.friction.clamp(0.0, f32::MAX);
        state.velocity.smooth_nudge(&Vec3::ZERO, friction, dt);
        if state.velocity.length_squared() < 1e-6 {
            state.velocity = Vec3::ZERO;
        }
    }

    // Apply movement update
    if state.velocity != Vec3::ZERO {
        let forward = *transform.forward();
        let right = *transform.right();
        let up = match config.vertical_movement_axis {
            VerticalMovementAxis::World => Vec3::Y,
            VerticalMovementAxis::Local => *transform.up(),
        };
        transform.translation += state.velocity.x * dt * right
            + state.velocity.y * dt * up
            + state.velocity.z * dt * forward;
    }

    // Handle mouse input
    if accumulated_mouse_motion.delta != Vec2::ZERO {
        // Apply look update
        state.pitch = (state.pitch
            - accumulated_mouse_motion.delta.y * RADIANS_PER_DOT * config.sensitivity)
            .clamp(-PI / 2., PI / 2.);
        state.yaw -= accumulated_mouse_motion.delta.x * RADIANS_PER_DOT * config.sensitivity;
        transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, state.yaw, state.pitch);
    }

    // Handle touch input
    for touch in touch_input.iter() {
        if touch.delta() != Vec2::ZERO {
            state.pitch = (state.pitch - touch.delta().y * RADIANS_PER_DOT * config.sensitivity)
                .clamp(-PI / 2., PI / 2.);
            state.yaw -= touch.delta().x * RADIANS_PER_DOT * config.sensitivity;
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, state.yaw, state.pitch);
        }
    }
}
