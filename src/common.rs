use bevy::prelude::*;

pub const CAMERA_SCALE: f32 = 0.3;
pub const TILE_SIZE: f32 = 8.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    StartMenu,
    Gaming,
}
