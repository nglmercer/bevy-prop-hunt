use bevy::prelude::*;
use bevy_hanabi::EffectProperties;

use crate::shared::tween::{TransformTween, update_tween};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_emitter.before(update_tween::<()>));
}

#[derive(Component)]
#[require(Transform, EffectProperties)]
pub struct TrailParticleEmitter {
    #[entities]
    pub following: Entity,
}

fn update_emitter(
    mut emitters: Query<(
        &TrailParticleEmitter,
        &mut EffectProperties,
        &mut TransformTween,
        &Transform,
    )>,

    entities: Query<&Transform, Without<TrailParticleEmitter>>,
) {
    for (emitter, ref mut props, ref mut tween, transform) in emitters.iter_mut() {
        let Ok(following) = entities.get(emitter.following) else {
            continue;
        };

        let normal = (transform.translation - following.translation).normalize_or_zero();
        props.set("normal", normal.into());

        tween.target = *following;
    }
}
