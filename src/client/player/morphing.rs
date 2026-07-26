use std::time::Duration;

use avian3d::prelude::{CollisionLayers, SpatialQuery, SpatialQueryFilter};
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_hanabi::{EffectProperties, ParticleEffect, VectorValue};

use crate::client::camera::tween::{CameraSystemsSet, CameraTween};
use crate::client::camera::{CameraMode, CurrentCamera, PlayerCamera};
use crate::client::debug_texture::DebugMaterial;
use crate::client::particles::emitters::trail::TrailParticleEmitter;
use crate::client::particles::magic::MagicParticleEffect;
use crate::client::states::ClientState;
use crate::client::ui::crosshair::Crosshair;
use crate::shared::physics::PhysicsLayers;
use crate::shared::player::{LocalPlayer, Player};
use crate::utils::tween::TransformTween;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
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
            .run_if(in_state(ClientState::Running)),
    );
}

fn handle_morph(
    mut commands: Commands,
    magic_effect: Res<MagicParticleEffect>,
    current_player: Single<
        (Entity, &Transform),
        (
            With<LocalPlayer>,
            Without<MorphColddown>,
            (Without<PropTarget>, Without<CurrentCamera>),
        ),
    >,
    target: Single<
        (Entity, &Transform),
        (
            With<PropTarget>,
            (Without<LocalPlayer>, Without<CurrentCamera>),
        ),
    >,
    camera: Single<
        (Entity, &Transform),
        (
            With<CurrentCamera>,
            With<PlayerCamera>,
            (Without<LocalPlayer>, Without<PropTarget>),
        ),
    >,
) {
    commands
        .entity(target.0)
        .remove::<PropTarget>()
        .insert(Player)
        .insert(LocalPlayer)
        .insert(MorphColddown(Duration::from_secs(1)))
        .insert(CollisionLayers {
            memberships: PhysicsLayers::Player.into(),
            ..default()
        });

    commands
        .entity(current_player.0)
        .remove::<LocalPlayer>()
        .remove::<Player>()
        .insert(CollisionLayers {
            memberships: PhysicsLayers::Prop.into(),
            ..default()
        });

    let normal = (current_player.1.translation - target.1.translation).normalize_or_zero();

    let entity = commands
        .spawn((
            TrailParticleEmitter {
                following: target.0,
            },
            TransformTween::<()> {
                reference: *current_player.1,
                target: *target.1,
                duration: Duration::from_millis(500),
                ..default()
            },
            // DespawnOnTime::new(Duration::from_millis(900)),
            ParticleEffect::new((&**magic_effect).clone()),
            EffectProperties::default()
                .with_properties([(String::from("normal"), VectorValue::new_vec3(normal).into())]),
        ))
        .id();

    commands.delayed().secs(0.9).entity(entity).despawn();

    commands.entity(camera.0).insert(CameraTween {
        reference: camera.1.clone(),
        duration: Duration::from_millis(300),
        ..default()
    });
}

#[derive(Component)]
pub struct MorphColddown(pub Duration);

fn update_colddown(
    mut commands: Commands,
    time: Res<Time>,
    mut crosshair: ResMut<Crosshair>,
    mut colddowns: Query<(Entity, &mut MorphColddown, Has<LocalPlayer>)>,
) {
    for (entity, mut colddown, is_player) in colddowns.iter_mut() {
        colddown.0 = colddown.0.saturating_sub(time.delta());

        if colddown.0.is_zero() {
            commands.entity(entity).remove::<MorphColddown>();
            if is_player {
                crosshair.bottom_loader = None;
            }
        } else if is_player {
            crosshair.bottom_loader = Some(colddown.0.as_secs_f32());
        }
    }
}

#[derive(Component)]
#[component(on_add = hightlight_target, on_remove= unhightlight_target)]
pub struct PropTarget;

fn retarget(
    mut commands: Commands,
    raycaster: SpatialQuery,
    camera: Single<(&Transform, &PlayerCamera), With<CurrentCamera>>,
    old_target: Option<Single<Entity, With<PropTarget>>>,
    _: Single<(), (Without<MorphColddown>, With<LocalPlayer>)>,
) {
    let Some(hit) = raycaster.cast_ray(
        camera.0.translation + camera.0.forward() * camera.1.player_distance,
        camera.0.forward(),
        40.,
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
    let Some(material) = world.get::<MeshMaterial3d<DebugMaterial>>(ctx.entity) else {
        return;
    };

    let handle = material.id();

    let trans_start = world.resource::<Time>().elapsed_secs_wrapped();

    let mut materials = world.resource_mut::<Assets<DebugMaterial>>();

    let Some(mut m) = materials.get_mut(handle) else {
        return;
    };

    m.extension.is_active = true;
    m.extension.trans_start = trans_start;
}

fn unhightlight_target(mut world: DeferredWorld, ctx: HookContext) {
    let Some(material) = world.get::<MeshMaterial3d<DebugMaterial>>(ctx.entity) else {
        return;
    };

    let handle = material.id();

    let trans_start = world.resource::<Time>().elapsed_secs_wrapped();

    let mut materials = world.resource_mut::<Assets<DebugMaterial>>();

    let Some(mut m) = materials.get_mut(handle) else {
        return;
    };

    m.extension.is_active = false;
    m.extension.trans_start = trans_start;
}
