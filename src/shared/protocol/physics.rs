use avian3d::prelude::*;
use bevy::app::App;
use lightyear::prelude::*;

pub fn plugin(app: &mut App) {
    app.component::<LinearVelocity>()
        .replicate()
        .predict()
        .with_rollback_condition(linear_velocity_should_rollback);

    app.component::<AngularVelocity>()
        .replicate()
        .predict()
        .with_rollback_condition(angular_velocity_should_rollback);

    app.component::<Position>()
        .replicate()
        .predict()
        .with_rollback_condition(position_should_rollback)
        .add_linear_correction_fn()
        .add_linear_interpolation();

    app.component::<Rotation>()
        .replicate()
        .predict()
        .with_rollback_condition(rotation_should_rollback)
        .add_linear_correction_fn()
        .add_linear_interpolation();
}

fn position_should_rollback(this: &Position, that: &Position) -> bool {
    (this.0 - that.0).length() >= 0.01
}

fn rotation_should_rollback(this: &Rotation, that: &Rotation) -> bool {
    this.angle_between(*that) >= 0.01
}

fn linear_velocity_should_rollback(this: &LinearVelocity, that: &LinearVelocity) -> bool {
    (this.0 - that.0).length() >= 0.01
}

fn angular_velocity_should_rollback(this: &AngularVelocity, that: &AngularVelocity) -> bool {
    (this.0 - that.0).length() >= 0.01
}
