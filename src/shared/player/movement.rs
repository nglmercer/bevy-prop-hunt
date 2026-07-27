use avian3d::math::*;
use avian3d::prelude::*;
use bevy::ecs::query::QueryData;
use bevy::ecs::system::lifetimeless::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use super::PlayerAction;

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
    mut forces: PropPhysicsItem,
) {
    let direction = inputs.clamped_axis_pair(&PlayerAction::Move);
    let delta_secs = time.delta_secs();

    if direction != Vector2::ZERO {
        forces.linear_velocity.x += direction.x * 50. * delta_secs;
        forces.linear_velocity.z -= direction.y * 50. * delta_secs;

        let out = forces.linear_velocity.xz().clamp_length_max(20.);
        forces.linear_velocity.x = out.x;
        forces.linear_velocity.z = out.y;
    }

    if inputs.just_pressed(&PlayerAction::Jump) {
        let hit_data = raycast.cast_shape(
            forces.collider,
            forces.position.0,
            forces.rotation.0,
            Dir3::NEG_Y,
            &ShapeCastConfig {
                max_distance: 0.1,
                compute_contact_on_penetration: false,
                ..default()
            },
            &SpatialQueryFilter::default().with_excluded_entities([player]),
        );

        if hit_data.is_some() {
            forces.linear_velocity.y = 10.;
        }
    }
}
