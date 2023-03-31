use bevy::{prelude::*, render::texture::ImageSampler, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use common::*;
use level::*;
use player::*;
use ui::*;
use weather::*;

mod common;
mod level;
mod player;
mod ui;
mod weather;

fn main() {
    App::new()
        .add_plugins(
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
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WeatherPlugin)
        .add_plugin(LdtkPlugin)
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            //设置背景透明
            level_background: LevelBackground::Nonexistent,
            ..Default::default()
        })
        .add_event::<SpringUpEvent>()
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
                player_run,
                player_jump,
                player_dash,
                player_die,
                despawn_hair.after(player_die),
                player_revive,
                spawn_hair.after(player_revive),
                spring_up,
                animate_run,
                animate_jump,
                animate_stand,
                aninmate_spring,
                animate_balloon_rope,
                animate_hair,
                animate_dust,
            )
                .in_set(OnUpdate(AppState::Gaming)),
        )
        .register_ldtk_int_cell::<TerrainBundle>(1)
        .register_ldtk_entity::<SpringBundle>("Spring")
        .register_ldtk_entity::<TrapBundle>("Trap")
        .register_ldtk_entity::<SnowdriftBundle>("Snowdrift")
        .register_ldtk_entity::<BalloonRopeBundle>("BalloonRope")
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.projection.scale = CAMERA_SCALE;
    commands.spawn(camera2d_bundle);
}
