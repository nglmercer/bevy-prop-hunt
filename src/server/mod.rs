use std::collections::HashSet;
use std::time::Duration;

use avian3d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use lightyear::steam::server::SteamServerIo;

use crate::shared::cosmetic_data::{CosmeticData, CosmeticMesh};
use crate::shared::network::{ConnectionState, ConnectionStatus, LocalClient, NetworkConfig};
use crate::shared::physics::PhysicsLayers;
use crate::shared::player::Player;
use crate::shared::protocol::player::MorphRequest;
use crate::test_scene;

pub fn plugin(app: &mut App) {
    app.init_resource::<PeerMetadata>()
        // 15 fps
        .insert_resource(ReplicationMetadata::new(Duration::from_millis(67)))
        .add_plugins((ServerPlugins::default(),))
        .add_systems(Update, (update_morph_cooldowns, handle_morph_requests))
        .add_observer(on_host)
        .add_observer(on_server_started)
        .add_observer(on_new_client)
        .add_observer(on_client_connected);
}

#[derive(Event)]
pub struct Host;

fn on_host(
    _: On<Host>,
    mut commands: Commands,
    mut connection: ResMut<ConnectionState>,
    config: Res<NetworkConfig>,
    servers: Query<(), With<Server>>,
    local_clients: Query<(), With<LocalClient>>,
) {
    if connection.status != ConnectionStatus::Disconnected
        || !servers.is_empty()
        || !local_clients.is_empty()
    {
        println!("[HOST] Ignoring duplicate host attempt");
        return;
    }

    connection.status = ConnectionStatus::Hosting;

    let server = commands
        .spawn((
            Server::default(),
            SteamServerIo {
                target: server::ListenTarget::Addr(config.server_bind_addr),
                config: SessionConfig::default(),
            },
        ))
        .id();

    commands.spawn((
        Name::from("Local Client"),
        LocalClient,
        Client::default(),
        LinkOf { server },
    ));

    commands.trigger(Start { entity: server });

    test_scene(commands);
}

#[derive(Component)]
struct ServerMorphCooldown(Duration);

fn update_morph_cooldowns(
    mut commands: Commands,
    time: Res<Time>,
    mut cooldowns: Query<(Entity, &mut ServerMorphCooldown)>,
) {
    for (entity, mut cooldown) in &mut cooldowns {
        cooldown.0 = cooldown.0.saturating_sub(time.delta());
        if cooldown.0.is_zero() {
            commands.entity(entity).remove::<ServerMorphCooldown>();
        }
    }
}

fn handle_morph_requests(
    mut commands: Commands,
    mut clients: Query<(&mut MessageReceiver<MorphRequest>, &RemoteId), With<ClientOf>>,
    players: Query<(
        Entity,
        &Player,
        &Transform,
        &CollisionLayers,
        Option<&ServerMorphCooldown>,
    )>,
    props: Query<(&Transform, &CollisionLayers), Without<Player>>,
) {
    let mut claimed_targets = HashSet::new();

    for (mut receiver, remote_id) in &mut clients {
        for request in receiver.receive() {
            if claimed_targets.contains(&request.target) {
                continue;
            }

            let Some((player_entity, player_peer, player_transform)) = players
                .iter()
                .find(|(_, player, _, layers, cooldown)| {
                    player.0 == remote_id.0
                        && layers.memberships.has_all(PhysicsLayers::Player)
                        && cooldown.is_none()
                })
                .map(|(entity, player, transform, _, _)| (entity, player.0, transform))
            else {
                continue;
            };

            let Ok((target_transform, target_layers)) = props.get(request.target) else {
                continue;
            };

            if !target_layers.memberships.has_all(PhysicsLayers::Prop)
                || player_transform
                    .translation
                    .distance(target_transform.translation)
                    > 40.
            {
                continue;
            }

            claimed_targets.insert(request.target);

            commands
                .entity(player_entity)
                .remove::<Player>()
                .insert(CollisionLayers {
                    memberships: PhysicsLayers::Prop.into(),
                    ..default()
                });

            commands.entity(request.target).insert((
                Player(player_peer),
                ServerMorphCooldown(Duration::from_secs(1)),
                CollisionLayers {
                    memberships: PhysicsLayers::Player.into(),
                    ..default()
                },
            ));

            break;
        }
    }
}

fn on_server_started(
    _: On<Add, Started>,
    mut commands: Commands,
    local_client: Single<Entity, With<LocalClient>>,
) {
    commands.trigger(Connect {
        entity: *local_client,
    });
}

fn on_new_client(trigger: On<Add, LinkOf>, mut commands: Commands) {
    println!("[on_new_client]");

    commands
        .entity(trigger.entity)
        .insert_if_new(Name::from("Remote Client"))
        .insert(ReplicationSender);
}

fn on_client_connected(
    trigger: On<Add, Connected>,

    mut commands: Commands,
    query: Query<&RemoteId, With<ClientOf>>,
) {
    println!("[on_client_connected]");
    let Ok(peer_id) = query.get(trigger.entity) else {
        return;
    };

    commands.spawn(player_bundle(**peer_id, trigger.entity));
}

fn player_bundle(peer_id: PeerId, owner: Entity) -> impl Bundle {
    (
        Name::new(format!("Player {peer_id}")),
        Player(peer_id),
        RigidBody::Dynamic,
        Collider::capsule(1., 2.),
        CosmeticData::<true> {
            shape: CosmeticMesh::Capsule3d(1., 2.),
        },
        CollisionLayers {
            memberships: PhysicsLayers::Player.into(),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 5., -5.),
            ..default()
        },
        ControlledBy {
            owner,
            lifetime: Lifetime::Persistent,
        },
        Replicate::to_clients(NetworkTarget::All),
        PredictionTarget::to_clients(NetworkTarget::All),
        InterpolationTarget::to_clients(NetworkTarget::All),
    )
}

// fn handle_player_actions(
//     time: Res<Time>,
//     raycast: SpatialQuery,
//     mut query: Query<(Entity, &ActionState<PlayerAction>, PropPhysics)>,
// ) {
//     for (entity, action_state, physics) in &mut query {
//         move_player(&time, &raycast, entity, action_state, physics);
//     }
// }
