use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use ui::setup_start_menu;

mod ui;
mod weather;
use crate::weather::WeatherPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(950.0, 700.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(WeatherPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_start_menu)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
