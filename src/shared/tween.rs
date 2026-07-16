use core::fmt;
use std::any::type_name;
use std::marker::PhantomData;
use std::time::Duration;

use bevy::ecs::component::{Mutable, StorageType};
use bevy::prelude::*;

use crate::lenses::smooth_transform_lerp;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_tween::<()>);
}

pub struct TransformTween<Marker: 'static = ()> {
    pub reference: Transform,
    pub target: Transform,
    pub time: Duration,
    pub duration: Duration,
    pub marker: PhantomData<Marker>,
}

impl<Marker> Default for TransformTween<Marker> {
    fn default() -> Self {
        Self {
            reference: Transform::default(),
            target: Transform::default(),
            time: Duration::default(),
            duration: Duration::default(),
            marker: PhantomData::default(),
        }
    }
}

impl<Marker> Clone for TransformTween<Marker> {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference.clone(),
            target: self.target.clone(),
            time: self.time.clone(),
            duration: self.duration.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<Marker> fmt::Debug for TransformTween<Marker> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("reference", &self.reference)
            .field("target", &self.target)
            .field("time", &self.time)
            .field("duration", &self.duration)
            .finish_non_exhaustive()
    }
}

unsafe impl<Marker> Send for TransformTween<Marker> {}
unsafe impl<Marker> Sync for TransformTween<Marker> {}

impl<Marker: 'static> Component for TransformTween<Marker> {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

pub fn update_tween<Marker: 'static>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut TransformTween<Marker>)>,
) {
    for (entity, ref mut transform, ref mut tween) in query.iter_mut() {
        tween.time += time.delta();

        if tween.time > tween.duration {
            commands.entity(entity).remove::<TransformTween<Marker>>();
            return;
        }

        smooth_transform_lerp(
            transform,
            &tween.reference,
            &tween.target,
            tween.time.as_secs_f32() / tween.duration.as_secs_f32(),
        );
    }
}
