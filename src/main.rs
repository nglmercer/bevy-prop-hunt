use std::f32::consts::{FRAC_PI_2, PI, TAU};

use avian3d::prelude::*;
use bevy::feathers::FeathersPlugins;
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;
use bevy_tweening::TweeningPlugin;

use self::cameras::{CameraMode, CurrentCamera, FreeCamera, PlayerCamera};
use self::debug_texture::{
    DebugMaterial, spawn_debug_texture, spawn_hoverable_debug_texture, uv_debug_texture,
};
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
            MaterialPlugin::<DebugMaterial>::default(),
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
        .add_systems(
            Startup,
            (
                cameras.spawn(),
                client::ui::crosshair::crosshair.spawn(),
                test_scene,
            )
                .chain(),
        )
        .run()
}

fn cameras() -> impl SceneList {
    bsn_list! {
        Camera2d
        Camera {
            order: 10,
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
        #FreeCamera
        FreeCamera
        Camera3d
        Camera {
            is_active: false
        }
        Projection::from(PerspectiveProjection {
            fov: 80_f32.to_radians(),
            ..default()
        })
    }
}

fn test_scene(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image = images.add(uv_debug_texture());

    fn wall(x: f32, y: f32, image: Handle<Image>) -> impl Scene {
        bsn! {
            template_value(RigidBody::Static)
            template_value(Collider::cuboid(200. * y.abs() + 0.2, 20., 200. * x.abs() + 0.2))
            Mesh3d(asset_value(Plane3d::new(vec3(x, 0., y), vec2(90. * y.abs() + 10., 90. * x.abs() + 10.))))
            Transform {
                translation: vec3(-100. * x, 10., -100. * y),
                rotation: {Quat::from_rotation_y(y * TAU)}
            }
            MeshMaterial3d<DebugMaterial>(asset_value(spawn_hoverable_debug_texture(image)))
            CollisionLayers {
                memberships: PhysicsLayers::Prop,
            }
        }
    }

    fn prop(collider: Collider, mesh: impl Into<Mesh>, image: Handle<Image>) -> impl Scene {
        bsn! {
            template_value(RigidBody::Dynamic)
            template_value(collider)
            Mesh3d(asset_value(mesh))
            MeshMaterial3d<DebugMaterial>(asset_value(spawn_hoverable_debug_texture(image)))
            CollisionLayers {
                memberships: PhysicsLayers::Prop,
            }
            Transform {
                translation: Vec3::new(rand::random_range(-20.0..20.0), 5., rand::random_range(-20.0..20.0)),
            }
        }
    }

    commands.spawn_scene_list(bsn_list! [
        #Floor
        template_value(RigidBody::Static)
        template_value(Collider::cuboid(200., 0.5, 200.))
        Mesh3d(asset_value(Plane3d::new(Vec3::Y, Vec2::splat(100.))))
        MeshMaterial3d<StandardMaterial>(asset_value(spawn_debug_texture(image.clone())))
        CollisionLayers {
            memberships: PhysicsLayers::Map,
        }
        ,
        wall(1., 0., image.clone()),
        wall(-1., 0., image.clone()),
        wall(0., 1., image.clone()),
        wall(0., -1., image.clone()),

        #Player
        Player
        LocalPlayer
        template_value(RigidBody::Dynamic)
        template_value(Collider::capsule(1., 2.))
        Mesh3d(asset_value(Capsule3d::new(1., 2.)))
        Transform {
            translation: Vec3::new(0., 2., -5.),
        }
        MeshMaterial3d<DebugMaterial>(asset_value(spawn_hoverable_debug_texture(image.clone())))
        CollisionLayers {
            memberships: PhysicsLayers::Player,
        }
        ,
        prop(Collider::capsule(1., 2.), Capsule3d::new(1., 2.), image.clone()),
        prop(Collider::capsule(1., 2.), Capsule3d::new(1., 2.), image.clone()),
        prop(Collider::capsule(1., 2.), Capsule3d::new(1., 2.), image.clone()),
        prop(Collider::capsule(1., 2.), Capsule3d::new(1., 2.), image.clone()),

        prop(Collider::cone(1., 2.), Cone::new(1., 2.), image.clone()),
        prop(Collider::cone(1., 2.), Cone::new(1., 2.), image.clone()),
        prop(Collider::cone(1., 2.), Cone::new(1., 2.), image.clone()),
        prop(Collider::cone(1., 2.), Cone::new(1., 2.), image.clone()),

        prop(Collider::cuboid(2., 2., 2.), Cuboid::new(2., 2., 2.), image.clone()),
        prop(Collider::cuboid(2., 2., 2.), Cuboid::new(2., 2., 2.), image.clone()),
        prop(Collider::cuboid(2., 2., 2.), Cuboid::new(2., 2., 2.), image.clone()),
        prop(Collider::cuboid(2., 2., 2.), Cuboid::new(2., 2., 2.), image.clone()),
    ]);

    commands.trigger(Pause);
}
