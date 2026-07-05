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
const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

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
    mut windows: Query<(&Window, &mut CursorOptions)>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    touch_input: Res<Touches>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut toggle_cursor_grab: Local<bool>,
    mut mouse_cursor_grab: Local<bool>,
    mut query: Query<(&mut Transform, &mut FreeCameraState, &FreeCamera), With<Camera>>,
) {
    let dt = time.delta_secs();

    let Ok((mut transform, mut state, config)) = query.single_mut() else {
        return;
    };

    if !state.enabled {
        // don't keep the cursor grabbed if the camera controller was disabled.
        if *toggle_cursor_grab || *mouse_cursor_grab {
            *toggle_cursor_grab = false;
            *mouse_cursor_grab = false;

            for (_, mut cursor_options) in &mut windows {
                cursor_options.grab_mode = CursorGrabMode::None;
                cursor_options.visible = true;
            }
        }
        return;
    }

    let scroll = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => {
            accumulated_mouse_scroll.delta.y / MouseScrollUnit::SCROLL_UNIT_CONVERSION_FACTOR
        }
    };
    // By using exponentiation we ensure that this scales up and down smoothly
    // regardless of the amount of scrolling processed per frame
    state.speed_multiplier *= exp(config.scroll_factor * scroll);
    // Clamp the speed multiplier for safety.
    state.speed_multiplier = state.speed_multiplier.clamp(f32::EPSILON, f32::MAX);

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

    let mut cursor_grab_change = false;
    if key_input.just_pressed(config.keyboard_key_toggle_cursor_grab) {
        *toggle_cursor_grab = !*toggle_cursor_grab;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_pressed(config.mouse_key_cursor_grab) {
        *mouse_cursor_grab = true;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_released(config.mouse_key_cursor_grab) {
        *mouse_cursor_grab = false;
        cursor_grab_change = true;
    }
    let cursor_grab = *mouse_cursor_grab || *toggle_cursor_grab;

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

    // Handle cursor grab
    if cursor_grab_change {
        if cursor_grab {
            for (window, mut cursor_options) in &mut windows {
                if !window.focused {
                    continue;
                }

                cursor_options.grab_mode = CursorGrabMode::Locked;
                cursor_options.visible = false;
            }
        } else {
            for (_, mut cursor_options) in &mut windows {
                cursor_options.grab_mode = CursorGrabMode::None;
                cursor_options.visible = true;
            }
        }
    }

    // Handle mouse input
    if accumulated_mouse_motion.delta != Vec2::ZERO && cursor_grab {
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
    // Axis snapping
    let mod_key_pressed = key_input.pressed(config.key_snap_reverse);
    let mut rotate_to = None;
    if key_input.just_pressed(config.axis_front) {
        if mod_key_pressed {
            rotate_to = Some((Dir3::Z, Dir3::Y));
        } else {
            rotate_to = Some((Dir3::NEG_Z, Dir3::Y));
        }
    }
    if key_input.just_pressed(config.axis_right) {
        if mod_key_pressed {
            rotate_to = Some((Dir3::NEG_X, Dir3::Y));
        } else {
            rotate_to = Some((Dir3::X, Dir3::Y));
        }
    }
    if key_input.just_pressed(config.axis_top) {
        if mod_key_pressed {
            rotate_to = Some((Dir3::NEG_Y, Dir3::NEG_Z));
        } else {
            rotate_to = Some((Dir3::Y, Dir3::Z));
        }
    }
    if let Some((dir, up)) = rotate_to {
        let start = transform.rotation;
        let target = Transform::default().looking_to(dir, up).rotation; // I don't understand why Quat::look_to_rh produce different result.
        let angle = target.angle_between(start);
        let rotation_time = angle / config.rotation_speed;

        if let Ok(interval) = Interval::new(0.0, rotation_time) {
            let curve = SampleAutoCurve::new(interval, [start, target])
                .expect("Interval should be in bounds as start and end are finite numbers");
            state.rotation_curve = Some((0.0, curve));
        }
    }
}
