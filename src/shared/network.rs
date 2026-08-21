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

        let steam_app_id = match env::var("PROP_HUNT_STEAM_APP_ID") {
            Ok(value) => value.parse().unwrap_or_else(|_| {
                panic!("PROP_HUNT_STEAM_APP_ID must be a valid numeric Steam App ID")
            }),
            Err(env::VarError::NotPresent) if cfg!(feature = "dev") => defaults.steam_app_id,
            Err(env::VarError::NotPresent) => {
                panic!("PROP_HUNT_STEAM_APP_ID must be set for non-dev builds")
            }
            Err(env::VarError::NotUnicode(_)) => {
                panic!("PROP_HUNT_STEAM_APP_ID must be valid UTF-8")
            }
        };

        Self {
            server_addr: socket_addr_from_env("PROP_HUNT_SERVER_ADDR", defaults.server_addr),
            server_bind_addr: socket_addr_from_env(
                "PROP_HUNT_SERVER_BIND_ADDR",
                defaults.server_bind_addr,
            ),
            steam_app_id,
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
        // Position mode applies prediction/correction/frame interpolation to
        // Avian's Position and Rotation components. The renderer adds the
        // matching FrameInterpolate markers to dynamic entities below.
        lightyear::frame_interpolation::FrameInterpolationPlugin::<Position>::default(),
        lightyear::frame_interpolation::FrameInterpolationPlugin::<Rotation>::default(),
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
