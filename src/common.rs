use bevy::prelude::*;

pub const CAMERA_SCALE: f32 = 0.25;
pub const TILE_SIZE: f32 = 8.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    StartMenu,
    Gaming,
}

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Clone, Debug, Default, Bundle)]
pub struct AnimationBundle {
    pub timer: AnimationTimer,
    pub indices: AnimationIndices,
}
