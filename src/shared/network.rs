use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use std::env;
use std::net::{Ipv4Addr, SocketAddr};

pub const DEFAULT_SERVER_PORT: u16 = 6767;
pub const DEFAULT_STEAM_APP_ID: u32 = 480;

#[derive(Resource, Clone, Debug)]
pub struct NetworkConfig {
    pub server_addr: SocketAddr,
    pub server_bind_addr: SocketAddr,
    pub steam_app_id: u32,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            server_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), DEFAULT_SERVER_PORT),
            server_bind_addr: SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), DEFAULT_SERVER_PORT),
            steam_app_id: DEFAULT_STEAM_APP_ID,
        }
    }
}

impl NetworkConfig {
    pub fn from_env() -> Self {
        let defaults = Self::default();

        Self {
            server_addr: socket_addr_from_env("PROP_HUNT_SERVER_ADDR", defaults.server_addr),
            server_bind_addr: socket_addr_from_env(
                "PROP_HUNT_SERVER_BIND_ADDR",
                defaults.server_bind_addr,
            ),
            steam_app_id: env::var("PROP_HUNT_STEAM_APP_ID")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(defaults.steam_app_id),
        }
    }
}

fn socket_addr_from_env(name: &str, default: SocketAddr) -> SocketAddr {
    let Some(value) = env::var_os(name) else {
        return default;
    };

    match value.to_string_lossy().parse() {
        Ok(address) => address,
        Err(error) => {
            eprintln!(
                "Invalid {name} value {:?} ({error}); using {default}",
                value.to_string_lossy()
            );
            default
        }
    }
}

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ConnectionState {
    pub status: ConnectionStatus,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Hosting,
    Connecting,
    Connected,
}

pub fn plugin(app: &mut App) {
    let config = NetworkConfig::from_env();

    app.insert_resource(config.clone())
        .init_resource::<ConnectionState>()
        .add_steam_resources(config.steam_app_id);

    app.add_plugins((
        lightyear::avian3d::plugin::LightyearAvianPlugin {
            replication_mode: lightyear::avian3d::plugin::AvianReplicationMode::Position,
            ..default()
        },
        PhysicsPlugins::default()
            .build()
            // disable the position<>transform sync plugins as it is handled by lightyear_avian
            .disable::<PhysicsTransformPlugin>()
            .disable::<PhysicsInterpolationPlugin>()
            // disable Sleeping plugin as it can mess up physics rollbacks
            .disable::<IslandPlugin>()
            .disable::<IslandSleepingPlugin>(),
    ));
}

#[derive(Component)]
pub struct LocalClient;
