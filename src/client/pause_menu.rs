use std::time::Duration;

use bevy::feathers::controls::*;
use bevy::feathers::dark_theme::create_dark_theme;
use bevy::feathers::theme::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::window::CursorGrabMode;
use bevy::window::CursorOptions;
use bevy_tweening::AnimTarget;
use bevy_tweening::Tween;
use bevy_tweening::TweenAnim;
use bevy_tweening::lens::UiPositionLens;

use crate::client::states::ClientState;
use crate::utils::{lenses::FadeLens, opacity::Opacity};

pub fn plugin(app: &mut App) {
    app.insert_resource(UiTheme(create_dark_theme()))
        .init_state::<PauseState>()
        .add_systems(
            Update,
            (
                (|mut commands: Commands| commands.trigger(Pause))
                    .run_if(in_state(PauseState::Ready)),
                (|mut commands: Commands| commands.trigger(Resume))
                    .run_if(in_state(PauseState::Paused)),
            )
                .run_if(input_just_pressed(KeyCode::Escape)),
        )
        .add_observer(show_pause)
        .add_observer(hide_pause)
        .add_observer(animate_enter)
        .add_observer(animate_exit);
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    Ready,
    Transition,
    Paused,
}

const BUTTON_MILLIS: u64 = 400;
const MENU_DURATION: Duration = Duration::from_millis(BUTTON_MILLIS + 100 * 2);

#[derive(Event)]
pub struct Pause;

#[derive(Event)]
pub struct Resume;

#[derive(Component, Default, Clone, Copy)]
pub struct PauseMenu;

fn pause_menu() -> impl Scene {
    bsn! {
        PauseMenu
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            Node {
                width: percent(100.),
                height: percent(100.),
                max_width: px(300),
                max_height: px(300),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly
            }
            Children [
                menu_button(0, "Resume")
                on(|_: On<Activate>, mut commands: Commands| commands.trigger(Resume))
                ,
                menu_button(1, "Settings")
                ,
                menu_button(2, "Quit")
                on(|_: On<Activate>, mut app_exit_writer: MessageWriter<AppExit>| {app_exit_writer.write(AppExit::Success);})
            ]
        ]
    }
}

fn menu_button(index: usize, label: impl Into<String>) -> impl Scene {
    bsn! {
        @FeathersButton
        Children [ Text(label) ThemedText ]
        Node {
            width: percent(100),
            height: px(50),
        }
        MenuButton(index)
        EnterAnimation
    }
}

fn hide_pause(
    _: On<Resume>,
    mut windows: Query<(&Window, &mut CursorOptions)>,
    state: Res<State<PauseState>>,
    mut commands: Commands,
    pause_menu: Single<Entity, With<PauseMenu>>,
    menu_buttons: Query<Entity, With<MenuButton>>,
) {
    if *state != PauseState::Paused {
        return;
    }

    commands.set_state(PauseState::Transition);
    commands.set_state(ClientState::Running);

    commands
        .delayed()
        .duration(MENU_DURATION)
        .set_state(PauseState::Ready);

    commands
        .delayed()
        .duration(MENU_DURATION)
        .entity(pause_menu.entity())
        .despawn();

    for menu_button in menu_buttons {
        commands.entity(menu_button).insert(ExitAnimation);
    }

    for (window, mut cursor_options) in &mut windows {
        if !window.focused {
            continue;
        }

        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
    }
}

fn show_pause(
    _: On<Pause>,
    mut cursors: Query<&mut CursorOptions, With<Window>>,
    state: Res<State<PauseState>>,
    mut commands: Commands,
) {
    if *state != PauseState::Ready {
        return;
    }

    commands.set_state(PauseState::Transition);
    commands.set_state(ClientState::Paused);

    commands
        .delayed()
        .duration(MENU_DURATION)
        .set_state(PauseState::Paused);

    commands.spawn_scene(pause_menu());

    for mut cursor_options in &mut cursors {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct MenuButton(pub usize);

#[derive(Component, Default, Clone, Copy)]
struct EnterAnimation;

#[derive(Component, Default, Clone, Copy)]
struct ExitAnimation;

fn animate_enter(
    trigger: On<Insert, EnterAnimation>,
    mut commands: Commands,
    enter_animation: Query<&MenuButton, With<EnterAnimation>>,
) {
    let Some(MenuButton(index)) = enter_animation.get(trigger.entity).ok() else {
        return;
    };

    commands
        .entity(trigger.entity)
        .try_remove::<EnterAnimation>()
        .insert(Opacity(0.));

    commands
        .delayed()
        .duration(Duration::from_millis((100 * index) as u64 + BUTTON_MILLIS))
        .entity(trigger.entity);

    commands
        .delayed()
        .duration(Duration::from_millis((100 * index) as u64))
        .entity(trigger.entity)
        .with_child((
            AnimTarget::component::<Opacity>(trigger.entity),
            TweenAnim::new(Tween::new(
                EaseFunction::CubicOut,
                Duration::from_millis(BUTTON_MILLIS),
                FadeLens { start: 0., end: 1. },
            )),
        ))
        .with_child((
            AnimTarget::component::<Node>(trigger.entity),
            TweenAnim::new(Tween::new(
                EaseFunction::CubicOut,
                Duration::from_millis(BUTTON_MILLIS),
                UiPositionLens {
                    start: UiRect {
                        left: percent(-50),
                        ..default()
                    },
                    end: UiRect {
                        left: percent(0),
                        ..default()
                    },
                },
            )),
        ));
}

fn animate_exit(
    trigger: On<Insert, ExitAnimation>,
    mut commands: Commands,
    enter_animation: Query<&MenuButton, With<ExitAnimation>>,
) {
    let Some(MenuButton(index)) = enter_animation.get(trigger.entity).ok() else {
        return;
    };

    commands
        .entity(trigger.entity)
        .try_remove::<ExitAnimation>()
        .insert(Opacity(1.));

    commands
        .delayed()
        .duration(Duration::from_millis((100 * index) as u64))
        .entity(trigger.entity)
        .with_child((
            AnimTarget::component::<Opacity>(trigger.entity),
            TweenAnim::new(Tween::new(
                EaseFunction::CubicIn,
                Duration::from_millis(BUTTON_MILLIS),
                FadeLens { start: 1., end: 0. },
            )),
        ))
        .with_child((
            AnimTarget::component::<Node>(trigger.entity),
            TweenAnim::new(Tween::new(
                EaseFunction::CubicIn,
                Duration::from_millis(BUTTON_MILLIS),
                UiPositionLens {
                    start: UiRect {
                        left: percent(0),
                        ..default()
                    },
                    end: UiRect {
                        left: percent(50),
                        ..default()
                    },
                },
            )),
        ));
}
