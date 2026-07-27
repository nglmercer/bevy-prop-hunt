use bevy::prelude::*;
use lightyear::prelude::*;

use super::cosmetic_data::CosmeticData;

mod particles;
mod physics;
mod player;

pub fn plugin(app: &mut App) {
    player::plugin(app);
    physics::plugin(app);
    particles::plugin(app);

    app.component::<Name>()
        .replicate_once_filtered::<Without<Client>>();

    app.component::<CosmeticData<false>>().replicate_once();
    app.component::<CosmeticData<true>>().replicate_once();
    app.component::<Transform>().replicate_once();
}
