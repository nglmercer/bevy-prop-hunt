use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use lightyear::steam::server::SteamServerIo;

use crate::shared::cosmetic_data::{CosmeticData, CosmeticMesh};
use crate::shared::network::SERVER_PORT;
use crate::shared::player::Player;
use crate::test_scene;

pub fn plugin(app: &mut App) {
    app.init_resource::<PeerMetadata>()
        // 15 fps
        .insert_resource(ReplicationMetadata::new(Duration::from_millis(67)))
        .add_plugins((ServerPlugins::default(),))
        .add_observer(on_host)
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

    commands.spawn((Client::default(), LinkOf { server }));

    commands.trigger(Start { entity: server });
    commands.trigger(crate::client::Connect { host_mode: true });

    test_scene(commands);
}

fn on_new_client(trigger: On<Add, LinkOf>, mut commands: Commands) {
    println!("[on_new_client]");

    commands
        .entity(trigger.entity)
        .insert((ReplicationSender, Name::from("Client")));
}

fn on_client_connected(
    trigger: On<Add, Connected>,

    mut commands: Commands,
    query: Query<&RemoteId, With<ClientOf>>,
) {
    println!("[on_client_connected]");
    let Ok(_) = query.get(trigger.entity) else {
        return;
    };

    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::capsule(1., 2.),
        CosmeticData::<true> {
            shape: CosmeticMesh::Capsule3d(1., 2.),
        },
        Transform {
            translation: Vec3::new(0., 5., -5.),
            ..default()
        },
        ControlledBy {
            owner: trigger.entity,
            lifetime: Lifetime::Persistent,
        },
        Replicate::to_clients(NetworkTarget::All),
        PredictionTarget::to_clients(NetworkTarget::All),
        InterpolationTarget::to_clients(NetworkTarget::All),
    ));
}
