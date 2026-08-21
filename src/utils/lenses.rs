use bevy::ecs::component::Mutable;
use bevy::prelude::*;
use bevy_tweening::{AnimTarget, IntoBoxedTweenable, Lens, TweenAnim};

use crate::utils::opacity::Opacity;

pub trait TweenCommands {
    fn tween_component<C: Component<Mutability = Mutable> + 'static>(
        &mut self,
        tween: impl IntoBoxedTweenable,
    ) -> &mut Self;
}

impl TweenCommands for EntityCommands<'_> {
    fn tween_component<C: Component<Mutability = Mutable> + 'static>(
        &mut self,
        tween: impl IntoBoxedTweenable,
    ) -> &mut Self {
        let anim_target = AnimTarget::component::<C>(self.id());

        self.insert((TweenAnim::new(tween), anim_target))
    }
}

pub fn smooth_transform_lerp(
    target: &mut Transform,
    start: &Transform,
    end: &Transform,
    ratio: f32,
) {
    target.translation = start
        .translation
        .lerp(end.translation, EaseFunction::SineOut.sample_clamped(ratio));
    target.rotation = start
        .rotation
        .lerp(end.rotation, EaseFunction::SineInOut.sample_clamped(ratio));
    target.scale = start.scale.lerp(end.scale, ratio);
}

#[derive(Default)]
pub struct SmoothTransformLens {
    pub start: Transform,
    pub end: Transform,
}

impl SmoothTransformLens {
    pub fn new(start: Transform, end: Transform) -> Self {
        Self { start, end }
    }
}

impl Lens<Transform> for SmoothTransformLens {
    fn lerp(&mut self, mut target: Mut<'_, Transform>, ratio: f32) {
        smooth_transform_lerp(&mut target, &self.start, &self.end, ratio);
    }
}

#[derive(Clone, Copy)]
pub struct FadeLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Opacity> for FadeLens {
    fn lerp(&mut self, mut target: Mut<'_, Opacity>, ratio: f32) {
        target.set_changed();
        target.0 = self.start.lerp(self.end, ratio);
    }
}
