use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use lightyear::steam::server::SteamServerIo;

use crate::shared::network::SERVER_PORT;
use crate::test_scene;

pub fn plugin(app: &mut App) {
    app.init_resource::<PeerMetadata>()
        // 15 fps
        .insert_resource(ReplicationMetadata::new(Duration::from_millis(67)))
        .add_plugins((ServerPlugins::default(),))
        .add_observer(on_host)
        .add_observer(on_new_client);
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
