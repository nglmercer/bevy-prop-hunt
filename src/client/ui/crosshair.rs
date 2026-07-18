use std::f32::consts::PI;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Crosshair>().add_systems(
        Update,
        update_crosshair.run_if(resource_changed::<Crosshair>),
    );
}

#[derive(Resource, Default)]
pub struct Crosshair {
    pub bottom_loader: Option<f32>,
}

#[derive(Component, Default, Clone)]
struct BottomLoader(pub bool);

pub fn crosshair() -> impl SceneList {
    bsn_list! {
        Node {
            width: px(4),
            height: px(4),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center
            border_radius: {BorderRadius::MAX},
        }
        BackgroundColor(Color::WHITE)
        ,

        Node {
            width: px(30),
            height: px(30),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center
        }
        Transform {
            rotation: {Quat::from_rotation_z(-PI)}
        }
        Children [
            BottomLoader
            Mesh2d(asset_value(crosshair_ring(1.)))
            MeshMaterial2d<ColorMaterial>(asset_value(Color::WHITE))
        ]

    }
}

fn crosshair_ring(progress: f32) -> Ring<CircularSector> {
    Ring::new(
        CircularSector::from_turns(25., 0.25 * progress),
        CircularSector::from_turns(22., 0.25 * progress),
    )
}

fn update_crosshair(
    crosshair: Res<Crosshair>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut bottom_loader: Single<(&mut BottomLoader, &Mesh2d)>,
) {
    let (ref mut bottom_loader_state, bottom_loader_mesh) = *bottom_loader;

    let mut bottom_loader_progress = crosshair.bottom_loader.clone();

    if crosshair.bottom_loader.is_some() ^ bottom_loader_state.0 {
        bottom_loader_state.0 ^= true;
        bottom_loader_progress = Some(0.);
    }

    if let Some(bottom_loader_progress) = bottom_loader_progress
        && let Some(mut bottom_loader_mesh) = meshes.get_mut(bottom_loader_mesh)
    {
        *bottom_loader_mesh = Mesh::from(crosshair_ring(bottom_loader_progress));
    }
}
