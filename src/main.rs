use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSampler,
    window::WindowResolution,
};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
                default_sampler: ImageSampler::nearest_descriptor(),
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
    .add_plugin(FrameTimeDiagnosticsPlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(LdtkPlugin);

    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugin(WeatherPlugin);
    }

    app.add_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            //设置背景透明
            level_background: LevelBackground::Nonexistent,
            ..Default::default()
        })
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
        .add_startup_system(setup_camera)
        // Start Menu
        .add_system(setup_start_menu.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(enter_gaming.in_set(OnUpdate(AppState::StartMenu)))
        .add_system(cleanup_start_menu.in_schedule(OnExit(AppState::StartMenu)))
        // Gaming
        .add_system(setup_ldtk_world.in_schedule(OnEnter(AppState::Gaming)))
        .add_system(
            spawn_ldtk_entity
                .in_base_set(CoreSet::PreUpdate)
                .run_if(in_state(AppState::Gaming)),
        )
        .add_systems(
            (
                spring_up,
                snowdrift_broken,
                wooden_stand_through,
                aninmate_spring,
                animate_balloon_rope,
                camera_follow,
                camera_shake,
            )
                .in_set(OnUpdate(AppState::Gaming)),
        )
        .add_systems(
            (
                player_run,
                player_move,
                player_jump,
                player_dash,
                player_dash_over,
                player_climb,
                player_die,
                despawn_hair.after(player_die),
                player_revive,
                spawn_hair.after(player_revive),
                handle_player_collision,
                player_grounded_detect,
                player_next_to_detect,
                player_facing_update,
            )
                .in_set(OnUpdate(AppState::Gaming)),
        )
        .add_systems(
            (
                animate_run,
                animate_jump,
                animate_stand,
                animate_dash,
                animate_hair,
                animate_dust,
            )
                .in_set(OnUpdate(AppState::Gaming)),
        )
        // TODO 下一版本可简化
        .add_systems(
            (player_state_machine,)
                .in_base_set(CoreSet::PostUpdate)
                .distributive_run_if(in_gaming_state),
        )
        .register_ldtk_int_cell::<TerrainBundle>(1)
        .register_ldtk_entity::<SpringBundle>("Spring")
        .register_ldtk_entity::<TrapBundle>("Trap")
        .register_ldtk_entity::<SnowdriftBundle>("Snowdrift")
        .register_ldtk_entity::<BalloonRopeBundle>("BalloonRope")
        .run();
}
