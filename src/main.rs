use bevy::image::ImageSamplerDescriptor;
use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use camera::*;
use common::*;
use level::*;
use player::*;
use state_machine::*;
use ui::*;
use weather::*;

mod camera;
mod common;
mod level;
mod player;
mod state_machine;
mod ui;
mod weather;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin {
                // 像素画放大后仍保证清晰
                default_sampler: ImageSamplerDescriptor::nearest(),
            })
            .set(WindowPlugin {
                //设置窗口大小 1100*750
                primary_window: Some(Window {
                    #[cfg(target_os = "windows")]
                    position: WindowPosition::Centered(MonitorSelection::Primary), //窗口居中
                    resolution: WindowResolution::new(1200.0, 800.0),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    // .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugins(LdtkPlugin);

    // TODO fix
    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     app.add_plugins(WeatherPlugin);
    // }

    app.init_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LevelSelection::index(0))
        .insert_resource(PlayerState::Standing)
        .insert_resource(CameraState::Following)
        .insert_resource(PlayerGrounded(false))
        .insert_resource(PlayerNextTo(None))
        .insert_resource(PlayerCannotMoveTime(0.0))
        .register_type::<PlayerState>()
        .register_type::<PlayerGrounded>()
        .register_type::<PlayerNextTo>()
        .add_event::<SpringUpEvent>()
        .add_event::<CameraShakeEvent>()
        .add_event::<DashStartEvent>()
        .add_event::<DashOverEvent>()
        .add_systems(Startup, (setup_camera,))
        // Start Menu
        .add_systems(OnEnter(AppState::StartMenu), (setup_start_menu,))
        .add_systems(
            Update,
            (enter_gaming,).run_if(in_state(AppState::StartMenu)),
        )
        .add_systems(OnExit(AppState::StartMenu), (cleanup_start_menu,))
        // Gaming
        .add_systems(OnEnter(AppState::Gaming), (setup_ldtk_world,))
        .add_systems(
            PreUpdate,
            (spawn_ldtk_entity,).run_if(in_state(AppState::Gaming)),
        )
        .add_systems(
            Update,
            (
                spring_up,
                snowdrift_broken,
                wooden_stand_through,
                aninmate_spring,
                animate_balloon_rope,
                camera_follow,
                camera_shake,
                (
                    player_run,
                    player_move,
                    player_jump,
                    player_dash,
                    player_dash_over,
                    player_climb,
                    player_die,
                ),
                despawn_hair.after(player_die),
                player_revive,
                spawn_hair.after(player_revive),
                handle_player_collision,
                player_grounded_detect,
                player_next_to_detect,
                player_facing_update,
                (
                    animate_run,
                    animate_jump,
                    animate_stand,
                    animate_dash,
                    animate_hair,
                    animate_dust,
                ),
            )
                .run_if(in_state(AppState::Gaming)),
        )
        .add_systems(
            PostUpdate,
            (player_state_machine,).run_if(in_state(AppState::Gaming)),
        )
        .register_ldtk_int_cell::<TerrainBundle>(1)
        .register_ldtk_entity::<SpringBundle>("Spring")
        .register_ldtk_entity::<TrapBundle>("Trap")
        .register_ldtk_entity::<SnowdriftBundle>("Snowdrift")
        .register_ldtk_entity::<BalloonRopeBundle>("BalloonRope")
        .run();
}
