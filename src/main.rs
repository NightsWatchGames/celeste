use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;

mod weather;
use crate::weather::WeatherPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window{
                resolution: WindowResolution::new(950.0,700.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(WeatherPlugin)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .add_startup_system(setup_camera)
        .run();
}

pub fn setup_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk"),
        ..Default::default()
    });
}
