use bevy::{prelude::*, render::texture::ImageSampler, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use common::{AppState, CAMERA_SCALE};
use level::{
    animate_balloon_rope, setup_ldtk_world, spawn_ldtk_entity, BalloonRopeBundle, ColliderBundle,
    SnowdriftBundle, SpringBundle, TerrainBundle, TrapBundle, WoodenStand, WoodenStandBundle,
};

use crate::weather::WeatherPlugin;
use ui::{cleanup_start_menu, enter_gaming, setup_start_menu};

mod common;
mod level;
mod ui;
mod weather;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSampler::nearest_descriptor(),
        }))
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
        .add_startup_system(setup_camera)
        // Start Menu
        .add_system(setup_start_menu.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(enter_gaming.in_set(OnUpdate(AppState::StartMenu)))
        .add_system(cleanup_start_menu.in_schedule(OnExit(AppState::StartMenu)))
        // Gaming
        .add_system(setup_ldtk_world.in_schedule(OnEnter(AppState::Gaming)))
        .add_systems((spawn_ldtk_entity, animate_balloon_rope).in_set(OnUpdate(AppState::Gaming)))
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
