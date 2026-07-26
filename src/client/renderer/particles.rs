use std::time::Duration;

use bevy::prelude::*;
use bevy_hanabi::{EffectProperties, ParticleEffect, VectorValue};
use lightyear::prelude::{Client, MessageManager};

use crate::client::particles::emitters::trail::TrailParticleEmitter;
use crate::client::particles::magic::MagicParticleEffect;
use crate::shared::particles::MagicTrailParticles;
use crate::utils::tween::TransformTween;

pub fn plugin(app: &mut App) {
    app.add_observer(magic_trail_particles);
}

pub fn magic_trail_particles(
    trigger: On<Add, MagicTrailParticles>,
    mut commands: Commands,
    message_manager: Single<&MessageManager, With<Client>>,
    particle: Query<&MagicTrailParticles>,
    query: Query<&Transform>,

    magic_effect: Res<MagicParticleEffect>,
) {
    let Ok(particle) = particle.get(trigger.entity) else {
        return;
    };

    let following = message_manager
        .entity_mapper
        .get_local(particle.following)
        .unwrap_or(particle.following);

    let Ok(target) = query.get(following) else {
        return;
    };

    let normal = (particle.from.translation - target.translation).normalize_or_zero();

    let entity = commands
        .entity(trigger.entity)
        .insert((
            TrailParticleEmitter { following },
            TransformTween::<()> {
                reference: particle.from,
                target: *target,
                duration: Duration::from_millis(500),
                ..default()
            },
            ParticleEffect::new((&**magic_effect).clone()),
            EffectProperties::default()
                .with_properties([(String::from("normal"), VectorValue::new_vec3(normal).into())]),
        ))
        .id();

    commands.delayed().secs(0.9).entity(entity).despawn();
}
