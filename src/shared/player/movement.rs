use avian3d::math::*;
use avian3d::prelude::*;
use bevy::ecs::query::QueryData;
use bevy::ecs::system::lifetimeless::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::shared::physics::PhysicsLayers;

use super::PlayerAction;

const PLAYER_ACCELERATION: f32 = 50.;
const PLAYER_MAX_HORIZONTAL_SPEED: f32 = 20.;
const PLAYER_HORIZONTAL_FRICTION: f32 = 12.;
const PLAYER_GROUND_CHECK_DISTANCE: f32 = 0.25;
const PLAYER_JUMP_SPEED: f32 = 10.;
const PLAYER_COYOTE_TIME: f32 = 0.1;
const PLAYER_JUMP_BUFFER_TIME: f32 = 0.1;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct JumpState {
    coyote_timer: f32,
    jump_buffer_timer: f32,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PropPhysics {
    collider: Read<Collider>,
    position: Read<Position>,
    rotation: Read<Rotation>,
    linear_velocity: Write<LinearVelocity>,
}

pub fn move_player(
    time: &Time,
    raycast: &SpatialQuery,
    player: Entity,
    inputs: &ActionState<PlayerAction>,
    jump_state: &mut JumpState,
    mut forces: PropPhysicsItem,
) {
    let direction = inputs.clamped_axis_pair(&PlayerAction::Move);
    let delta_secs = time.delta_secs();

    if direction != Vector2::ZERO {
        forces.linear_velocity.x += direction.x * PLAYER_ACCELERATION * delta_secs;
        forces.linear_velocity.z -= direction.y * PLAYER_ACCELERATION * delta_secs;
    } else {
        let friction = (PLAYER_HORIZONTAL_FRICTION * delta_secs).min(1.);
        forces.linear_velocity.x *= 1. - friction;
        forces.linear_velocity.z *= 1. - friction;
    }

    let horizontal_velocity = forces
        .linear_velocity
        .xz()
        .clamp_length_max(PLAYER_MAX_HORIZONTAL_SPEED);
    forces.linear_velocity.x = horizontal_velocity.x;
    forces.linear_velocity.z = horizontal_velocity.y;

    let grounded = raycast
        .cast_shape(
            forces.collider,
            forces.position.0,
            forces.rotation.0,
            Dir3::NEG_Y,
            &ShapeCastConfig::from_max_distance(PLAYER_GROUND_CHECK_DISTANCE),
            &SpatialQueryFilter::default()
                .with_mask([PhysicsLayers::Map, PhysicsLayers::Prop])
                .with_excluded_entities([player]),
        )
        .is_some();

    update_jump(
        jump_state,
        grounded,
        inputs.just_pressed(&PlayerAction::Jump),
        delta_secs,
        &mut forces.linear_velocity.y,
    );
}

fn update_jump(
    jump_state: &mut JumpState,
    grounded: bool,
    jump_pressed: bool,
    delta_secs: f32,
    vertical_velocity: &mut f32,
) {
    if grounded {
        jump_state.coyote_timer = PLAYER_COYOTE_TIME;
    } else {
        jump_state.coyote_timer = (jump_state.coyote_timer - delta_secs).max(0.);
    }

    if jump_pressed {
        jump_state.jump_buffer_timer = PLAYER_JUMP_BUFFER_TIME;
    } else {
        jump_state.jump_buffer_timer = (jump_state.jump_buffer_timer - delta_secs).max(0.);
    }

    if jump_state.coyote_timer > 0. && jump_state.jump_buffer_timer > 0. {
        *vertical_velocity = PLAYER_JUMP_SPEED;
        jump_state.coyote_timer = 0.;
        jump_state.jump_buffer_timer = 0.;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use avian3d::prelude::{
        Collider, CollisionLayers, Gravity, LinearVelocity, PhysicsPlugins, Position, RigidBody,
        Rotation, SpatialQuery,
    };
    use bevy::{app::App, prelude::*, time::TimeUpdateStrategy};
    use leafwing_input_manager::prelude::ActionState;

    use super::{
        JumpState, PLAYER_JUMP_SPEED, PlayerAction, PropPhysics, move_player, update_jump,
    };

    #[derive(Component)]
    struct TestPlayer;

    fn apply_movement(
        time: Res<Time>,
        raycast: SpatialQuery,
        mut query: Query<
            (
                Entity,
                &ActionState<PlayerAction>,
                &mut JumpState,
                PropPhysics,
            ),
            With<TestPlayer>,
        >,
    ) {
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

    #[test]
    fn jumps_immediately_when_grounded() {
        let mut jump_state = JumpState::default();
        let mut vertical_velocity = 0.;

        update_jump(
            &mut jump_state,
            true,
            true,
            1. / 60.,
            &mut vertical_velocity,
        );

        assert_eq!(vertical_velocity, PLAYER_JUMP_SPEED);
        assert_eq!(jump_state.coyote_timer, 0.);
        assert_eq!(jump_state.jump_buffer_timer, 0.);
    }

    #[test]
    fn accepts_jump_during_coyote_time() {
        let mut jump_state = JumpState::default();
        let mut vertical_velocity = 0.;

        update_jump(
            &mut jump_state,
            true,
            false,
            1. / 60.,
            &mut vertical_velocity,
        );
        update_jump(&mut jump_state, false, true, 0.05, &mut vertical_velocity);

        assert_eq!(vertical_velocity, PLAYER_JUMP_SPEED);
    }

    #[test]
    fn buffers_jump_until_landing() {
        let mut jump_state = JumpState::default();
        let mut vertical_velocity = 0.;

        update_jump(
            &mut jump_state,
            false,
            true,
            1. / 60.,
            &mut vertical_velocity,
        );
        update_jump(&mut jump_state, true, false, 0.05, &mut vertical_velocity);

        assert_eq!(vertical_velocity, PLAYER_JUMP_SPEED);
    }

    #[test]
    fn does_not_jump_after_timers_expire() {
        let mut jump_state = JumpState::default();
        let mut vertical_velocity = 0.;

        update_jump(
            &mut jump_state,
            false,
            true,
            1. / 60.,
            &mut vertical_velocity,
        );
        update_jump(&mut jump_state, false, false, 0.2, &mut vertical_velocity);

        assert_eq!(vertical_velocity, 0.);
    }

    #[test]
    fn shape_cast_ground_detection_allows_a_physics_jump() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            PhysicsPlugins::default(),
            bevy::asset::AssetPlugin::default(),
            bevy::mesh::MeshPlugin,
        ))
        .insert_resource(Gravity::ZERO)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
            1. / 60.,
        )));
        app.finish();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                RigidBody::Static,
                Collider::cuboid(20., 1., 20.),
                Position::default(),
                Rotation::default(),
                CollisionLayers {
                    memberships: super::PhysicsLayers::Map.into(),
                    ..default()
                },
            ));

            let mut action_state = ActionState::default();
            action_state.press(&PlayerAction::Jump);

            commands.spawn((
                TestPlayer,
                RigidBody::Dynamic,
                Collider::cuboid(1., 2., 1.),
                Position(Vec3::new(0., 1.5, 0.)),
                Rotation::default(),
                LinearVelocity::default(),
                CollisionLayers {
                    memberships: super::PhysicsLayers::Player.into(),
                    ..default()
                },
                action_state,
                JumpState::default(),
            ));
        })
        .add_systems(FixedUpdate, apply_movement);

        for _ in 0..3 {
            app.update();
        }

        let mut query = app
            .world_mut()
            .query_filtered::<&LinearVelocity, With<TestPlayer>>();
        let velocity = query.single(app.world()).unwrap();
        assert_eq!(velocity.y, PLAYER_JUMP_SPEED);
    }
}
