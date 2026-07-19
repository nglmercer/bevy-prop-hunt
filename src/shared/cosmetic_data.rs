use avian3d::prelude::CollisionLayers;
use bevy::prelude::*;

use crate::utils::asset_ref::AssetRef;

use super::physics::PhysicsLayers;

#[derive(Component, FromTemplate)]
pub struct CosmeticData<const HOVER: bool> {
    pub shape: AssetRef<Mesh>,
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
