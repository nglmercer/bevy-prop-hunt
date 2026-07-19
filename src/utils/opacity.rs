use bevy::app::Propagate;
use bevy::prelude::*;

pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_insert_opacity)
            .add_systems(Update, on_changed_opacity);
    }
}

#[derive(Component, Clone, Copy)]
pub struct Opacity(pub f32);

impl Default for Opacity {
    fn default() -> Self {
        Self(1.)
    }
}

fn on_insert_opacity(trigger: On<Insert, Opacity>, mut commands: Commands, query: Query<&Opacity>) {
    let Some(Opacity(alpha)) = query.get(trigger.entity).ok() else {
        return;
    };

    let alpha = *alpha;

    let Some(mut entity) = commands.get_spawned_entity(trigger.entity).ok() else {
        return;
    };

    entity.queue(move |mut entity: EntityWorldMut| {
        macro_rules! set_alpha {
            ($C:ty |$t:ident| $e:expr) => {
                if let Some(mut $t) = entity.get_mut::<$C>() {
                    $e.set_alpha(alpha);
                }
            };
        }

        set_alpha!(TextColor | t | t);
        set_alpha!(BackgroundColor | t | t);
        set_alpha!(Propagate<TextColor> |t| t.0);
        set_alpha!(Propagate<BackgroundColor> |t| t.0);
    });
}

fn on_changed_opacity(
    mut commands: Commands,
    query: Query<(
        Entity,
        Ref<Opacity>,
        (
            Option<Ref<TextColor>>,
            Option<Ref<BackgroundColor>>,
            Option<Ref<Propagate<TextColor>>>,
            Option<Ref<Propagate<BackgroundColor>>>,
        ),
    )>,
) {
    for (entity, opacity, props) in query {
        if !opacity.is_changed()
            && !props.0.is_some_and(|p| p.is_changed())
            && !props.1.is_some_and(|p| p.is_changed())
            && !props.2.is_some_and(|p| p.is_changed())
            && !props.3.is_some_and(|p| p.is_changed())
        {
            continue;
        }

        let alpha = opacity.0;

        let Some(mut entity) = commands.get_spawned_entity(entity).ok() else {
            continue;
        };

        entity.queue(move |mut entity: EntityWorldMut| {
            macro_rules! set_alpha {
                ($C:ty |$t:ident| $e:expr) => {
                    if let Some(mut $t) = entity.get_mut::<$C>() {
                        $e.set_alpha(alpha);
                    }
                };
            }

            set_alpha!(TextColor | t | t);
            set_alpha!(BackgroundColor | t | t);
            set_alpha!(Propagate<TextColor> |t| t.0);
            set_alpha!(Propagate<BackgroundColor> |t| t.0);
        });
    }
}
