use std::f32::consts::TAU;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use lightyear::prelude::{NetworkTarget, Replicate};

use self::shared::cosmetic_data::{CosmeticData, CosmeticMesh};
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
            client::plugin,
            server::plugin,
            shared::network::plugin,
            shared::protocol::plugin,
            utils::tween::plugin,
            bevy_inspector_egui::bevy_egui::EguiPlugin::default(),
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ))
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
                shape: CosmeticMesh::Plane3d(vec3(x, 0., y), vec2(90. * y.abs() + 10., 90. * x.abs() + 10.)),
            }
            Transform {
                translation: vec3(-100. * x, 10., -100. * y),
                rotation: {Quat::from_rotation_y(y * TAU)}
            }
            template_value(Replicate::to_clients(NetworkTarget::All))
        }
    }

    fn prop(collider: Collider, shape: CosmeticMesh) -> impl Scene {
        bsn! {
            template_value(RigidBody::Dynamic)
            template_value(collider)
            @CosmeticData<true> {
                @layer: PhysicsLayers::Prop,
                shape,
            }
            Transform {
                translation: Vec3::new(rand::random_range(-70.0..70.0), 5., rand::random_range(-70.0..70.0)),
            }
            template_value(Replicate::to_clients(NetworkTarget::All))
            // template_value(PredictionTarget::to_clients(NetworkTarget::All))
        }
    }

    commands.spawn_scene_list(bsn_list! [
        #Floor
        template_value(RigidBody::Static)
        template_value(Collider::cuboid(200., 0.5, 200.))
        @CosmeticData<false> {
            @layer: PhysicsLayers::Map,
            shape: CosmeticMesh::Plane3d({Vec3::Y}, Vec2::splat(100.))
        }
        CollisionLayers {
            memberships: PhysicsLayers::Map,
        }
        template_value(Replicate::to_clients(NetworkTarget::All))
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
            shape: CosmeticMesh::Capsule3d(1., 2.)
        }
        Transform {
            translation: Vec3::new(0., 2., -5.),
        }
        ,
    ]);

    for _ in 0..10 {
        commands.spawn_scene(prop(
            Collider::capsule(1., 2.),
            CosmeticMesh::Capsule3d(1., 2.),
        ));
        commands.spawn_scene(prop(Collider::cone(1., 2.), CosmeticMesh::Cone(1., 2.)));
        commands.spawn_scene(prop(
            Collider::cuboid(2., 2., 2.),
            CosmeticMesh::Cuboid(2., 2., 2.),
        ));
    }
}
