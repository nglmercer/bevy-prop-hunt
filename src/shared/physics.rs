use avian3d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default)]
pub enum PhysicsLayers {
    #[default]
    Default,
    Map,
    Prop,
    Player,
}
