use bevy::prelude::*;
use lightyear::input::config::InputConfig;
use lightyear::prelude::input::leafwing;
use lightyear::prelude::*;

use crate::shared::player::{Player, PlayerAction};

pub fn plugin(app: &mut App) {
    app.component::<Player>().replicate_once();

    app.add_plugins(leafwing::InputPlugin::<PlayerAction> {
        config: InputConfig::<PlayerAction> {
            rebroadcast_inputs: false,
            ..default()
        },
    });
}
