use bevy::{prelude::*, render::texture::ImageSampler, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use common::AppState;

use crate::weather::WeatherPlugin;
use ui::{cleanup_start_menu, enter_gaming, setup_start_menu};

mod common;
mod ui;
mod weather;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSampler::nearest_descriptor(),
        }))
        .add_plugin(WorldInspectorPlugin::new())
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
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.projection.scale = 0.1;
    commands.spawn(camera2d_bundle);
}

pub fn setup_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk"),
        ..Default::default()
    });
}