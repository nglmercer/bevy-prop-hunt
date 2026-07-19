use bevy::feathers::FeathersPlugins;
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;

use crate::utils::opacity::OpacityPlugin;

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
        ui::crosshair::plugin,
    ))
    .init_state::<states::ClientState>()
    .insert_state(camera::CameraMode::Playing)
    .add_systems(Startup, (cameras.spawn(), ui::crosshair::crosshair.spawn()))
    .add_systems(PostStartup, start_paused);
}

fn cameras() -> impl SceneList {
    bsn_list! {
        Camera2d
        Camera {
            order: 10,
        }
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
