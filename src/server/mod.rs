use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use avian3d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use lightyear::steam::server::SteamServerIo;

use crate::shared::cosmetic_data::{CosmeticData, CosmeticMesh};
use crate::shared::network::{LocalClient, SERVER_PORT};
use crate::shared::physics::PhysicsLayers;
use crate::shared::player::Player;
use crate::test_scene;

pub fn plugin(app: &mut App) {
    app.init_resource::<PeerMetadata>()
        // 15 fps
        .insert_resource(ReplicationMetadata::new(Duration::from_millis(67)))
        .add_plugins((ServerPlugins::default(),))
        // .add_systems(FixedUpdate, handle_player_actions)
        .add_observer(on_host)
        .add_observer(on_server_started)
        .add_observer(on_new_client)
        .add_observer(on_client_connected);
}

#[derive(Event)]
pub struct Host;

fn on_host(_: On<Host>, mut commands: Commands) {
    let server = commands
        .spawn((
            Server::default(),
            SteamServerIo {
                target: server::ListenTarget::Addr(SocketAddr::new(
                    Ipv4Addr::UNSPECIFIED.into(),
                    SERVER_PORT,
                )),
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
