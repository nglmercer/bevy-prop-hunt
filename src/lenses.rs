use bevy::app::Propagate;
use bevy::ecs::component::Mutable;
use bevy::prelude::*;
use bevy_tweening::{AnimTarget, IntoBoxedTweenable, Lens, TweenAnim};

use crate::opacity::Opacity;

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
        target.translation = self.start.translation.lerp(
            self.end.translation,
            EaseFunction::SineOut.sample_clamped(ratio),
        );
        target.rotation = self.start.rotation.lerp(
            self.end.rotation,
            EaseFunction::SineInOut.sample_clamped(ratio),
        );
        target.scale = self.start.scale.lerp(self.end.scale, ratio);
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
