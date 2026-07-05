use avian3d::prelude::*;
use bevy::feathers::FeathersPlugins;
use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

use self::cameras::{CameraMode, FreeCamera, PlayerCamera};
use self::debug_texture::spawn_debug_texture;
use self::opacity::OpacityPlugin;
use self::player::{LocalPlayer, Player};
use self::states::GameState;

mod cameras;
mod debug_texture;
mod lenses;
mod opacity;
mod pause_menu;
mod player;
mod states;
mod templates;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TweeningPlugin,
            PhysicsPlugins::default(),
            FeathersPlugins,
            OpacityPlugin,
            cameras::plugins,
            pause_menu::plugin,
            player::local_player::plugin,
        ))
        .init_state::<GameState>()
        .insert_state(CameraMode::Playing)
        .add_systems(Startup, test_scene)
        .run()
}

fn test_scene(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_texture = spawn_debug_texture(&mut images, &mut materials).material;

    commands.queue_spawn_scene_list(bsn_list! [
        Camera2d
        Camera {
            order: 1,
        }
        ,
        #Floor
        template_value(RigidBody::Static)
        template_value(Collider::cuboid(100., 0.5, 100.))
        Mesh3d(asset_value(Plane3d::new(Vec3::Y, Vec2::splat(50.))))
        MeshMaterial3d<StandardMaterial>({debug_texture.clone()})
        ,
        #PlayerCamera
        PlayerCamera
        Camera3d
        Projection::from(PerspectiveProjection {
            fov: 80_f32.to_radians(),
            ..default()
        })
        Transform {
            translation: Vec3::new(0., 6., 5.),
        }
        ,
        #DebugCamera
        FreeCamera
        Camera3d
        Camera {
            is_active: false
        }
        Projection::from(PerspectiveProjection {
            fov: 80_f32.to_radians(),
            ..default()
        })
        ,
        #Player
        Player
        LocalPlayer
        template_value(RigidBody::Dynamic)
        template_value(Collider::capsule(1., 2.))
        Mesh3d(asset_value(Capsule3d::new(1., 2.)))
        Transform {
            translation: Vec3::new(0., 2., -5.),
        }
        MeshMaterial3d<StandardMaterial>(debug_texture)
        ,
    ]);
}
