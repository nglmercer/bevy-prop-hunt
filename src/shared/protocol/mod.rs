use bevy::prelude::*;
use lightyear::prelude::*;

use super::cosmetic_data::CosmeticData;
use super::player::Player;

mod particles;
mod physics;

pub fn plugin(app: &mut App) {
    physics::plugin(app);
    particles::plugin(app);

    app.component::<Player>().replicate_once();
    app.component::<Name>()
        .replicate_once_filtered::<Without<Client>>();

    app.component::<CosmeticData<false>>().replicate_once();
    app.component::<CosmeticData<true>>().replicate_once();
    app.component::<Transform>().replicate_once();
}
