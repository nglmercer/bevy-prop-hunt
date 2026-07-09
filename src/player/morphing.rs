use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::color::palettes::css::BLUE;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

use crate::cameras::CurrentCamera;
use crate::cameras::tween::CameraSystemsSet;
use crate::physics::PhysicsLayers;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedPostUpdate, (retarget,).chain().after(CameraSystemsSet));
}

#[derive(Component)]
#[component(on_add = hightlight_target, on_remove= unhightlight_target)]
pub struct PropTarget;

fn retarget(
    mut commands: Commands,
    raycaster: SpatialQuery,
    camera: Single<&Transform, With<CurrentCamera>>,
    old_target: Option<Single<Entity, With<PropTarget>>>,
) {
    let Some(hit) = raycaster.cast_ray(
        camera.translation,
        camera.forward(),
        50.,
        false,
        &SpatialQueryFilter::default().with_mask(PhysicsLayers::Prop),
    ) else {
        if let Some(old_target) = old_target {
            commands.entity(*old_target).remove::<PropTarget>();
        }

        return;
    };

    if let Some(old_target) = old_target {
        if hit.entity == *old_target {
            return;
        }

        commands.entity(*old_target).remove::<PropTarget>();
    }

    commands.entity(hit.entity).insert(PropTarget);
}

fn hightlight_target(mut world: DeferredWorld, ctx: HookContext) {
    let Some(material) = world.get::<MeshMaterial3d<StandardMaterial>>(ctx.entity) else {
        return;
    };

    let handle = material.id();

    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

    let Some(mut m) = materials.get_mut(handle) else {
        return;
    };

    m.base_color = BLUE.into();
}

fn unhightlight_target(mut world: DeferredWorld, ctx: HookContext) {
    let Some(material) = world.get::<MeshMaterial3d<StandardMaterial>>(ctx.entity) else {
        return;
    };

    let handle = material.id();

    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

    let Some(mut m) = materials.get_mut(handle) else {
        return;
    };

    m.base_color = Color::WHITE;
}
