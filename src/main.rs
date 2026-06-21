use std::time::Duration;

use bevy::camera_controller::free_camera::*;
use bevy::feathers::FeathersPlugins;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_tweening::{AnimCompletedEvent, AnimTargetKind, Tween, TweeningPlugin};

use self::cameras::{CurrentCamera, DebugCamera, PlayerCamera, run_freecamera_controller};
use self::debug_texture::spawn_debug_texture;
use self::lenses::{SmoothTransformLens, TweenCommands};
use self::opacity::OpacityPlugin;
use self::pause_menu::PauseState;
use self::player::Player;
use self::states::GameState;

mod cameras;
mod debug_texture;
mod lenses;
mod opacity;
mod pause_menu;
mod player;
mod states;
mod templates;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TweeningPlugin,
            OpacityPlugin,
            pause_menu::plugin,
            FeathersPlugins,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, test_scene)
        .add_systems(
            Update,
            toggle_debug_camera
                .run_if(in_state(GameState::Running))
                .run_if(input_just_pressed(KeyCode::Tab)),
        )
        .add_systems(
            RunFixedMainLoop,
            (
                run_freecamera_controller.run_if(in_state(GameState::Running)),
                rotate_freecam_to.run_if(not(in_state(PauseState::Paused))),
            )
                .chain()
                .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        )
        .run()
}

fn test_scene(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_texture = spawn_debug_texture(&mut images, &mut materials).material;

    commands.queue_spawn_scene_list(bsn_list! [
        Camera2d
        Camera {
            order: 1,
        }
        ,
        #Floor
        Mesh3d(asset_value(Plane3d::new(Vec3::Y, Vec2::splat(50.))))
        MeshMaterial3d<StandardMaterial>({debug_texture.clone()})
        ,
        #PlayerCamera
        PlayerCamera
        Camera3d
        Projection::from(PerspectiveProjection {
            fov: 80_f32.to_radians(),
            ..default()
        })
        Transform {
            translation: Vec3::new(0., 6., 5.),
        }
        ,
        #DebugCamera
        DebugCamera
        Camera3d
        Camera {
            is_active: false
        }
        Projection::from(PerspectiveProjection {
            fov: 80_f32.to_radians(),
            ..default()
        })
        ,
        #Player
        Player
        Mesh3d(asset_value(Capsule3d::new(1., 2.)))
        Transform {
            translation: Vec3::new(0., 1., -5.),
        }
        MeshMaterial3d<StandardMaterial>(debug_texture)
        ,
    ]);
}

fn toggle_debug_camera(
    mut commands: Commands,
    mut player_camera: Single<
        (&mut Camera, Entity, &Transform),
        (With<PlayerCamera>, Without<DebugCamera>),
    >,
    mut debug_camera: Single<
        (
            &mut Camera,
            Entity,
            &Transform,
            Option<&mut FreeCameraState>,
        ),
        (Without<PlayerCamera>, With<DebugCamera>),
    >,
) {
    let is_player_cam_enabled = player_camera.0.is_active;

    let (ref mut player_camera, player_entity, player_transform) = *player_camera;
    let (ref mut debug_camera, debug_entity, debug_transform, ref mut free_camera_state) =
        *debug_camera;

    let camera_distance = player_transform
        .translation
        .distance(debug_transform.translation);
    let tween_duration = (camera_distance.sqrt() * 100.).min(400.).max(200.) as u64;

    if is_player_cam_enabled {
        player_camera.is_active = false;
        debug_camera.is_active = true;

        commands.entity(player_entity).try_remove::<CurrentCamera>();
        commands.entity(debug_entity).insert(CurrentCamera);

        let mut end_transform = *player_transform;
        end_transform.translation =
            player_transform.translation + *player_transform.forward() + *player_transform.up();

        if let Some(state) = free_camera_state {
            let (yaw, pitch, _roll) = end_transform.rotation.to_euler(EulerRot::YXZ);
            state.yaw = yaw;
            state.pitch = pitch;
        }

        commands
            .entity(debug_entity)
            .insert(*player_transform)
            .tween_component::<Transform>(
                Tween::new(
                    EaseFunction::Linear,
                    Duration::from_millis(tween_duration),
                    SmoothTransformLens::new(*player_transform, end_transform),
                )
                .with_cycle_completed_event(true),
            )
            .observe(enable_debug_camera);

        fn enable_debug_camera(
            trigger: On<AnimCompletedEvent>,
            mut commands: Commands,
            mut debug_camera: Single<(Entity, Option<&mut FreeCameraState>), With<DebugCamera>>,
        ) {
            if let AnimTargetKind::Component { entity: target } = trigger.target
                && target == debug_camera.0
            {
                if let Some(state) = &mut debug_camera.1 {
                    state.enabled = true;
                } else {
                    commands
                        .entity(debug_camera.0)
                        .insert((FreeCamera { ..default() }, FreeCameraState::default()));
                }
            }
        }
    } else {
        player_camera.is_active = true;
        debug_camera.is_active = false;

        commands.entity(player_entity).insert(CurrentCamera);
        commands.entity(debug_entity).try_remove::<CurrentCamera>();

        commands
            .entity(player_entity)
            .insert(*debug_transform)
            .tween_component::<Transform>(Tween::new(
                EaseFunction::Linear,
                Duration::from_millis(tween_duration),
                SmoothTransformLens::new(*debug_transform, *player_transform),
            ));

        if let Some(state) = free_camera_state {
            state.enabled = false;
        }
    }
}
