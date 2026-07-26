use bevy::prelude::*;
use lightyear::frame_interpolation::FrameInterpolationPlugin;
use lightyear::prelude::*;

use super::cosmetic_data::CosmeticData;

mod physics;

pub fn plugin(app: &mut App) {
    physics::plugin(app);

    app.component::<CosmeticData<false>>().replicate();
    app.component::<CosmeticData<true>>().replicate();
    app.component::<Transform>()
        .replicate_once();

    app.add_plugins(FrameInterpolationPlugin::<Transform>::default());
}
