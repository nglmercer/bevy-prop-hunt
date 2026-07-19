use bevy::prelude::*;

use crate::client::debug_texture::{
    DebugMaterial, DebugTexture, hoverable_texture_material, texture_material, uv_debug_texture,
};
use crate::shared::cosmetic_data::CosmeticData;
use crate::utils::asset_ref::AssetRef;

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
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, cosmetic_data) in cosmetic_data {
        let mesh = match &cosmetic_data.shape {
            AssetRef::Path(asset_path) => assets.load(asset_path),
            AssetRef::Handle(handle) => handle.clone(),
        };

        commands.entity(entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(texture_material(debug_texture.0.clone()))),
        ));
    }
}

fn assign_hoverable_costemics(
    mut commands: Commands,
    cosmetic_data: Query<(Entity, &CosmeticData<true>), Added<CosmeticData<true>>>,
    debug_texture: Res<DebugTexture>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<DebugMaterial>>,
) {
    for (entity, cosmetic_data) in cosmetic_data {
        let mesh = match &cosmetic_data.shape {
            AssetRef::Path(asset_path) => assets.load(asset_path),
            AssetRef::Handle(handle) => handle.clone(),
        };

        commands.entity(entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(hoverable_texture_material(debug_texture.0.clone()))),
        ));
    }
}
