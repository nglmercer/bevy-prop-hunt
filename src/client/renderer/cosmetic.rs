use avian3d::prelude::{Position, Rotation};
use bevy::prelude::*;
use lightyear::frame_interpolation::FrameInterpolate;

use crate::client::debug_texture::{
    DebugMaterial, DebugTexture, hoverable_texture_material, texture_material, uv_debug_texture,
};
use crate::shared::cosmetic_data::CosmeticData;

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, spawn_debug_texture)
        .add_systems(
            Update,
            (assign_static_costemics, assign_hoverable_costemics),
        );
}

fn spawn_debug_texture(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.insert_resource(DebugTexture(images.add(uv_debug_texture())));
}

fn assign_static_costemics(
    mut commands: Commands,
    cosmetic_data: Query<(Entity, &CosmeticData<false>), Added<CosmeticData<false>>>,
    debug_texture: Res<DebugTexture>,
    mut assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, cosmetic_data) in cosmetic_data {
        commands.entity(entity).insert((
            Mesh3d(cosmetic_data.shape.resolve(&mut assets)),
            MeshMaterial3d(materials.add(texture_material(debug_texture.0.clone()))),
        ));
    }
}

fn assign_hoverable_costemics(
    mut commands: Commands,
    cosmetic_data: Query<(Entity, &CosmeticData<true>), Added<CosmeticData<true>>>,
    debug_texture: Res<DebugTexture>,
    mut assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DebugMaterial>>,
) {
    for (entity, cosmetic_data) in cosmetic_data {
        commands.entity(entity).insert((
            // AvianReplicationMode::Position replicates and predicts Avian's
            // Position/Rotation components. Keep frame interpolation on those
            // same components so Lightyear's correction and visual history are
            // applied to the values that drive the rendered Transform.
            FrameInterpolate::<Position>::default(),
            FrameInterpolate::<Rotation>::default(),
            Mesh3d(cosmetic_data.shape.resolve(&mut assets)),
            MeshMaterial3d(materials.add(hoverable_texture_material(debug_texture.0.clone()))),
        ));
    }
}
