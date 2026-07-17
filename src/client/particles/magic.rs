use std::ops::Deref;

use bevy::prelude::*;
use bevy_hanabi::Gradient;
use bevy_hanabi::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, register_particle);
}

#[derive(Resource)]
pub struct MagicParticleEffect(pub Handle<EffectAsset>);

impl Deref for MagicParticleEffect {
    type Target = Handle<EffectAsset>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Handle<EffectAsset>> for MagicParticleEffect {
    fn as_ref(&self) -> &Handle<EffectAsset> {
        &self.0
    }
}

fn register_particle(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let w = ExprWriter::default();

    let normal = w.add_property("normal", Vec3::ZERO.into());
    let normal = w.prop(normal);

    let init_pos = SetAttributeModifier::new(Attribute::POSITION, w.lit(Vec3::ZERO).expr());

    let init_vel = SetAttributeModifier::new(
        Attribute::VELOCITY,
        {
            let perpx = (w.lit(0.) - normal.clone().y()).vec3(normal.clone().x(), w.lit(0.));
            let perpy = normal.clone().cross(perpx.clone()).normalized();
            normal * w.lit(3.)
                + (w.rand(ScalarType::Float) - w.lit(0.5)) * w.lit(3.) * perpx
                + (w.rand(ScalarType::Float) - w.lit(0.5)) * w.lit(3.) * perpy
        }
        .expr(),
    );

    let init_lifetime = SetAttributeModifier {
        attribute: Attribute::LIFETIME,
        value: w.lit(0.5).expr(),
    };

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0., Vec3::splat(0.80));
    size_gradient.add_key(0.75, Vec3::splat(0.35));
    size_gradient.add_key(1., Vec3::splat(0.));

    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0., Vec4::new(0., 0., 1., 1.));
    color_gradient.add_key(0.75, Vec4::new(0., 0., 1., 0.25));
    color_gradient.add_key(1., Vec4::new(0., 0., 1., 0.));

    let effect = EffectAsset::new(
        512,
        SpawnerSettings::new(15.0.into(), 0.4.into(), 0.5.into(), 1),
        w.finish(),
    )
    .with_name("MagicEffect")
    .with_simulation_space(SimulationSpace::Global)
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .render(SizeOverLifetimeModifier {
        gradient: size_gradient.into(),
        ..default()
    })
    .render(OrientModifier::new(OrientMode::FaceCameraPosition))
    .render(ColorOverLifetimeModifier::new(color_gradient.into()));

    let effect = effects.add(effect);

    commands.insert_resource(MagicParticleEffect(effect));
}
