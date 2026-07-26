use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;

pub const SERVER_PORT: u16 = 6767;

pub fn plugin(app: &mut App) {
    app.add_steam_resources(480);

    app.add_plugins((
        lightyear::avian3d::plugin::LightyearAvianPlugin {
            replication_mode: lightyear::avian3d::plugin::AvianReplicationMode::Position,
            ..default()
        },
        PhysicsPlugins::default()
            .build()
            // disable the position<>transform sync plugins as it is handled by lightyear_avian
            .disable::<PhysicsTransformPlugin>()
            .disable::<PhysicsInterpolationPlugin>()
            // disable Sleeping plugin as it can mess up physics rollbacks
            .disable::<IslandPlugin>()
            .disable::<IslandSleepingPlugin>(),
    ));
}
