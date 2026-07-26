use bevy::app::App;
use lightyear::prelude::*;

use crate::shared::particles::MagicTrailParticles;

pub fn plugin(app: &mut App) {
    app.component::<MagicTrailParticles>().replicate_once();
}
