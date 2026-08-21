use std::time::Duration;

use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::MessageSender;

use crate::client::camera::tween::{CameraSystemsSet, CameraTween};
use crate::client::camera::{CameraMode, CurrentCamera, PlayerCamera};
use crate::client::debug_texture::DebugMaterial;
use crate::client::states::ClientState;
use crate::client::ui::crosshair::Crosshair;
use crate::shared::network::LocalClient;
use crate::shared::particles::MagicTrailParticles;
use crate::shared::physics::PhysicsLayers;
use crate::shared::player::{LocalPlayer, Player, PlayerAction};
use crate::shared::protocol::player::{MorphChannel, MorphRequest};

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
            handle_morph.run_if(in_state(CameraMode::Playing)),
        )
            .run_if(in_state(ClientState::Running)),
    );
}

#[allow(clippy::type_complexity)]
fn handle_morph(
    mut commands: Commands,
    message_sender: Option<Single<&mut MessageSender<MorphRequest>, With<LocalClient>>>,
    action_state: Option<Single<&ActionState<PlayerAction>, With<LocalPlayer>>>,
    current_player: Option<
        Single<
            (Entity, &Transform, &Player),
            (
                With<LocalPlayer>,
                Without<MorphColddown>,
                (Without<PropTarget>, Without<CurrentCamera>),
            ),
        >,
    >,
    target: Option<
        Single<
            (Entity, &Transform),
            (
                With<PropTarget>,
                (Without<LocalPlayer>, Without<CurrentCamera>),
            ),
        >,
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
    let (Some(mut message_sender), Some(action_state), Some(current_player), Some(target)) =
        (message_sender, action_state, current_player, target)
    else {
        return;
    };

    if !action_state.just_pressed(&PlayerAction::Morph) {
        return;
    }

    message_sender.send::<MorphChannel>(MorphRequest { target: target.0 });

    // The server performs the actual component and collision-layer transfer. The
    // local cooldown only prevents duplicate requests while that update is in flight.
    commands
        .entity(target.0)
        .insert(MorphColddown(Duration::from_secs(1)))
        .remove::<PropTarget>();

    commands.spawn(MagicTrailParticles {
        from: *current_player.1,
        following: target.0,
    });

    commands.entity(camera.0).insert(CameraTween {
        reference: *camera.1,
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
            crosshair.bottom_loader = None;
        } else if is_player {
            crosshair.bottom_loader = Some(colddown.0.as_secs_f32());
        }
    }
}

#[derive(Component)]
#[component(on_add = hightlight_target, on_remove= unhightlight_target)]
pub struct PropTarget;

#[allow(clippy::type_complexity)]
fn retarget(
    mut commands: Commands,
    raycaster: SpatialQuery,
    camera: Single<(&Transform, &PlayerCamera), With<CurrentCamera>>,
    old_target: Option<Single<Entity, With<PropTarget>>>,
    local_player: Option<Single<(), (Without<MorphColddown>, With<LocalPlayer>)>>,
) {
    if local_player.is_none() {
        return;
    }

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
