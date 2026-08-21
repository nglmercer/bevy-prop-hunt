use std::net::{Ipv4Addr, SocketAddr};

use bevy::feathers::FeathersPlugins;
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;
use bevy_inspector_egui::bevy_egui::{
    EguiContext, EguiGlobalSettings, EguiPlugin, PrimaryEguiContext,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::*;
use lightyear::steam::client::SteamClientIo;

use crate::shared::network::{ConnectionState, ConnectionStatus, LocalClient, NetworkConfig};
use crate::utils::opacity::OpacityPlugin;

use self::states::ClientState;

mod camera;
mod debug_texture;
mod particles;
mod pause_menu;
mod player;
mod renderer;
mod states;
mod ui;

pub fn plugin(app: &mut bevy::app::App) {
    app.add_plugins((
        MaterialPlugin::<debug_texture::DebugMaterial>::default(),
        FeathersPlugins,
        OpacityPlugin,
        HanabiPlugin,
        camera::plugins,
        player::plugin,
        particles::plugins,
        pause_menu::plugin,
        renderer::cosmetic::plugin,
        renderer::particles::plugin,
        ui::crosshair::plugin,
        ClientPlugins::default(),
        EguiPlugin::default(),
        WorldInspectorPlugin::new().run_if(in_state(ClientState::Paused)),
    ))
    .insert_resource(EguiGlobalSettings {
        auto_create_primary_context: false,
        ..default()
    })
    .init_state::<states::ClientState>()
    .insert_state(camera::CameraMode::Playing)
    .add_systems(Startup, (cameras.spawn(), ui::crosshair::crosshair.spawn()))
    .add_systems(PostStartup, start_paused)
    .add_observer(on_connect)
    .add_observer(on_local_client_connected)
    .add_observer(on_local_client_disconnected);
}

#[derive(Event)]
pub struct Connect {
    pub host_mode: bool,
}

fn on_connect(
    ev: On<Connect>,
    mut commands: Commands,
    mut connection: ResMut<ConnectionState>,
    config: Res<NetworkConfig>,
    local_clients: Query<(), With<LocalClient>>,
) {
    println!("[CONNECT] Handling");
    if ev.host_mode {
        println!("[CONNECT] Host mode");
        return;
    }

    if connection.status != ConnectionStatus::Disconnected || !local_clients.is_empty() {
        println!("[CONNECT] Ignoring duplicate client connection attempt");
        return;
    }

    connection.status = ConnectionStatus::Connecting;

    println!("[CONNECT] Client mode");

    let server_addr = config.server_addr;

    let entity = commands
        .spawn((
            Name::from("Local Client"),
            LocalClient,
            Client::default(),
            PredictionManager::default(),
            Link::new(None),
            LocalAddr(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0)),
            PeerAddr(server_addr),
            SteamClientIo {
                target: client::ConnectTarget::Addr(server_addr),
                config: SessionConfig::default(),
            },
        ))
        .id();

    commands.trigger(lightyear::prelude::Connect { entity });
}

fn on_local_client_connected(
    trigger: On<Add, Connected>,
    mut connection: ResMut<ConnectionState>,
    local_client: Query<(), (With<LocalClient>, With<Connected>)>,
) {
    if local_client.get(trigger.entity).is_ok() {
        connection.status = ConnectionStatus::Connected;
    }
}

fn on_local_client_disconnected(
    trigger: On<Add, Disconnected>,
    mut commands: Commands,
    mut connection: ResMut<ConnectionState>,
    local_client: Query<(), With<LocalClient>>,
) {
    if local_client.get(trigger.entity).is_err() {
        return;
    }

    connection.status = ConnectionStatus::Disconnected;
    commands.entity(trigger.entity).despawn();
}

fn cameras() -> impl SceneList {
    bsn_list! {
        Camera2d
        Camera {
            order: 10,
        }
        EguiContext
        PrimaryEguiContext
        ,
        #FreeCamera
        camera::FreeCamera
        Camera3d
        Camera {
            is_active: false
        }
        Projection::from(PerspectiveProjection {
            fov: 80_f32.to_radians(),
            ..default()
        })
        ,
        #PlayerCamera
        camera::PlayerCamera
        camera::CurrentCamera
        Transform {
            translation: Vec3::new(0., 6., 5.),
        }
        Camera3d
        Projection::from(PerspectiveProjection {
            fov: 80_f32.to_radians(),
            ..default()
        })
    }
}

fn start_paused(mut commands: Commands) {
    commands.trigger(pause_menu::Pause);
}
