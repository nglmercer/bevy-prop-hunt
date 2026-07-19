use std::f32::consts::TAU;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

use self::shared::cosmetic_data::CosmeticData;
use self::shared::physics::PhysicsLayers;
use self::shared::player::{LocalPlayer, Player};

mod client;
mod server;
mod shared;
mod utils;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TweeningPlugin,
            PhysicsPlugins::default(),
            client::plugin,
            utils::tween::plugin,
        ))
        .add_systems(Startup, test_scene)
        .run()
}

fn test_scene(mut commands: Commands) {
    // let image = images.add(uv_debug_texture());

    fn wall(x: f32, y: f32) -> impl Scene {
        bsn! {
            template_value(RigidBody::Static)
            template_value(Collider::cuboid(200. * y.abs() + 0.2, 20., 200. * x.abs() + 0.2))
            @CosmeticData<false> {
                @layer: PhysicsLayers::Map,
                shape: asset_value(Plane3d::new(vec3(x, 0., y), vec2(90. * y.abs() + 10., 90. * x.abs() + 10.))),
            }
            Transform {
                translation: vec3(-100. * x, 10., -100. * y),
                rotation: {Quat::from_rotation_y(y * TAU)}
            }
        }
    }

    fn prop(collider: Collider, mesh: impl Into<Mesh>) -> impl Scene {
        bsn! {
            template_value(RigidBody::Dynamic)
            template_value(collider)
            @CosmeticData<true> {
                @layer: PhysicsLayers::Prop,
                shape: asset_value(mesh)
            }
            Transform {
                translation: Vec3::new(rand::random_range(-70.0..70.0), 5., rand::random_range(-70.0..70.0)),
            }
        }
    }

    commands.spawn_scene_list(bsn_list! [
        #Floor
        template_value(RigidBody::Static)
        template_value(Collider::cuboid(200., 0.5, 200.))
        @CosmeticData<false> {
            @layer: PhysicsLayers::Map,
            shape: asset_value(Plane3d::new(Vec3::Y, Vec2::splat(100.)))
        }
        CollisionLayers {
            memberships: PhysicsLayers::Map,
        }
        ,
        wall(1., 0.),
        wall(-1., 0.),
        wall(0., 1.),
        wall(0., -1.),

        #Player
        Player
        LocalPlayer
        template_value(RigidBody::Dynamic)
        template_value(Collider::capsule(1., 2.))
        @CosmeticData<true> {
            @layer: PhysicsLayers::Player,
            shape: asset_value(Capsule3d::new(1., 2.))
        }
        Transform {
            translation: Vec3::new(0., 2., -5.),
        }
        ,
    ]);

    for _ in 0..10 {
        commands.spawn_scene(prop(Collider::capsule(1., 2.), Capsule3d::new(1., 2.)));
        commands.spawn_scene(prop(Collider::cone(1., 2.), Cone::new(1., 2.)));
        commands.spawn_scene(prop(Collider::cuboid(2., 2., 2.), Cuboid::new(2., 2., 2.)));
    }
}
