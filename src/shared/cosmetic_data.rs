use avian3d::prelude::CollisionLayers;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::physics::PhysicsLayers;

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub enum CosmeticMesh {
    #[default]
    Unassigned,
    Capsule3d(f32, f32),
    Cone(f32, f32),
    Cuboid(f32, f32, f32),
    Plane3d(Vec3, Vec2),
}

impl CosmeticMesh {
    pub fn resolve(self, assets: &mut Assets<Mesh>) -> Handle<Mesh> {
        match self {
            CosmeticMesh::Unassigned => {
                warn!("Some mesh is unassigned");
                assets.reserve_handle()
            }
            CosmeticMesh::Capsule3d(a, b) => assets.add(Capsule3d::new(a, b)),
            CosmeticMesh::Cone(a, b) => assets.add(Cone::new(a, b)),
            CosmeticMesh::Cuboid(a, b, c) => assets.add(Cuboid::new(a, b, c)),
            CosmeticMesh::Plane3d(a, b) => assets.add(Plane3d::new(a, b)),
        }
    }
}

#[derive(Component, Serialize, Deserialize, FromTemplate)]
pub struct CosmeticData<const HOVER: bool> {
    pub shape: CosmeticMesh,
}

#[derive(Default)]
pub struct CosmeticDataProps {
    pub layer: PhysicsLayers,
}

impl<const HOVER: bool> SceneComponent for CosmeticData<HOVER> {
    type Props = CosmeticDataProps;

    fn scene(props: Self::Props) -> impl Scene {
        bsn! {
            CollisionLayers {
                memberships: {props.layer}
            }
        }
    }
}
