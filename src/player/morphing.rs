use std::time::Duration;

use avian3d::prelude::{CollisionLayers, SpatialQuery, SpatialQueryFilter};
use bevy::color::palettes::css::BLUE;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use crate::cameras::tween::{CameraSystemsSet, CameraTween};
use crate::cameras::{CameraMode, CurrentCamera, PlayerCamera};
use crate::physics::PhysicsLayers;
use crate::states::GameState;

use super::{LocalPlayer, Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedPostUpdate,
        (retarget.run_if(in_state(CameraMode::Playing)),)
            .chain()
            .after(CameraSystemsSet),
    )
    .add_systems(
        Update,
        (
            update_colddown,
            handle_morph
                .run_if(in_state(CameraMode::Playing))
                .run_if(input_just_pressed(MouseButton::Left)),
        )
            .run_if(in_state(GameState::Running)),
    );
}

fn handle_morph(
    mut commands: Commands,
    current_player: Single<Entity, (With<LocalPlayer>, Without<MorphColddown>)>,
    target: Single<Entity, With<PropTarget>>,
    camera: Single<(Entity, &Transform), (With<CurrentCamera>, With<PlayerCamera>)>,
) {
    commands
        .entity(*target)
        .remove::<PropTarget>()
        .insert(Player)
        .insert(LocalPlayer)
        .insert(CollisionLayers {
            memberships: PhysicsLayers::Player.into(),
            ..default()
        });

    commands
        .entity(*current_player)
        .remove::<LocalPlayer>()
        .remove::<Player>()
        .insert(CollisionLayers {
            memberships: PhysicsLayers::Prop.into(),
            ..default()
        });

    commands.entity(camera.0).insert(CameraTween {
        reference: camera.1.clone(),
        time: Duration::ZERO,
        duration: Duration::from_millis(300),
    });
}

#[derive(Component)]
pub struct MorphColddown(pub Duration);

fn update_colddown(
    mut commands: Commands,
    time: Res<Time>,
    mut colddowns: Query<(Entity, &mut MorphColddown)>,
) {
    for (entity, mut colddown) in colddowns.iter_mut() {
        colddown.0 = colddown.0.saturating_sub(time.delta());

        if colddown.0.is_zero() {
            commands.entity(entity).remove::<MorphColddown>();
        }
    }
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
