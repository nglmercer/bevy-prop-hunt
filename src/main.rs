use avian3d::prelude::*;
use bevy::feathers::FeathersPlugins;
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;
use bevy_tweening::TweeningPlugin;

use self::cameras::{CameraMode, CurrentCamera, FreeCamera, PlayerCamera};
use self::debug_texture::{spawn_debug_texture, uv_debug_texture};
use self::opacity::OpacityPlugin;
use self::pause_menu::Pause;
use self::physics::PhysicsLayers;
use self::player::{LocalPlayer, Player};
use self::states::GameState;

mod cameras;
mod client;
mod debug_texture;
mod lenses;
mod opacity;
mod pause_menu;
mod physics;
mod player;
mod shared;
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
            HanabiPlugin,
            cameras::plugins,
            player::plugins,
            pause_menu::plugin,
            client::plugins,
            shared::timed::plugin,
            shared::tween::plugin,
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
    let image = images.add(uv_debug_texture());

    commands.queue_spawn_scene_list(bsn_list! [
        Camera2d
        Camera {
            order: 10,
        }
        ,
        Node {
            width: px(2),
            height: px(2),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center
        }
        BackgroundColor(Color::WHITE)
        ,
        #Floor
        template_value(RigidBody::Static)
        template_value(Collider::cuboid(100., 0.5, 100.))
        Mesh3d(asset_value(Plane3d::new(Vec3::Y, Vec2::splat(50.))))
        MeshMaterial3d<StandardMaterial>({spawn_debug_texture(image.clone(), &mut materials)})
        CollisionLayers {
            memberships: PhysicsLayers::Map,
        }
        ,
        #PlayerCamera
        PlayerCamera
        CurrentCamera
        Transform {
            translation: Vec3::new(0., 6., 5.),
        }
        Camera3d
        Projection::from(PerspectiveProjection {
            fov: 80_f32.to_radians(),
            ..default()
        })
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
        MeshMaterial3d<StandardMaterial>({spawn_debug_texture(image.clone(), &mut materials)})
        CollisionLayers {
            memberships: PhysicsLayers::Player,
        }
        ,
        // Prop
        template_value(RigidBody::Dynamic)
        template_value(Collider::cone(1., 2.))
        Mesh3d(asset_value(Cone::new(1., 2.)))
        Transform {
            translation: Vec3::new(-10., 2., -5.),
        }
        MeshMaterial3d<StandardMaterial>({spawn_debug_texture(image.clone(), &mut materials)})
        CollisionLayers {
            memberships: PhysicsLayers::Prop,
        }
        ,
        // Prop
        template_value(RigidBody::Dynamic)
        template_value(Collider::cuboid(1., 1., 1.))
        Mesh3d(asset_value(Cuboid::new(1., 1., 1.)))
        Transform {
            translation: Vec3::new(10., 2., -5.),
        }
        MeshMaterial3d<StandardMaterial>({spawn_debug_texture(image.clone(), &mut materials)})
        CollisionLayers {
            memberships: PhysicsLayers::Prop,
        }
        ,
    ]);

    commands.trigger(Pause);
}
