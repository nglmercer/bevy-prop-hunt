use std::time::Duration;

use bevy::prelude::*;

use super::tween::update_tween;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, despawn_them.after(update_tween::<()>));
}

#[derive(Component)]
pub struct DespawnOnTime {
    pub elapsed: Duration,
    pub duration: Duration,
}

impl DespawnOnTime {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            elapsed: Duration::ZERO,
        }
    }
}

fn despawn_them(
    mut commands: Commands,
    time: Res<Time>,
    mut entities: Query<(Entity, &mut DespawnOnTime)>,
) {
    for (entity, ref mut despawn_on_time) in entities.iter_mut() {
        despawn_on_time.elapsed += time.delta();

        if despawn_on_time.elapsed > despawn_on_time.duration {
            commands.entity(entity).despawn();
        }
    }
}
