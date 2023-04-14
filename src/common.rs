use bevy::prelude::*;

pub const CAMERA_SCALE: f32 = 0.25;
pub const TILE_SIZE: f32 = 8.0;
pub const PLAYER_GRAVITY_SCALE: f32 = 10.0;
pub const PLAYER_DASHING_COLOR: Color = Color::rgb(
    41 as f32 / u8::MAX as f32,
    173 as f32 / u8::MAX as f32,
    255 as f32 / u8::MAX as f32,
);
pub const PLAYER_DASH_SPEED: f32 = 200.0;
pub const PLAYER_JUMP_SPEED: f32 = 300.0;

// sprite z轴顺序
pub const SPRITE_DUST_ORDER: f32 = 2.0;
pub const SPRITE_HAIR_ORDER: f32 = 3.0;
pub const SPRITE_PLAYER_ORDER: f32 = 4.0;

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
    // 表示sprite_indices数组下标
    pub index: usize,
    pub sprite_indices: Vec<usize>,
}

#[derive(Clone, Debug, Default, Bundle)]
pub struct AnimationBundle {
    pub timer: AnimationTimer,
    pub indices: AnimationIndices,
}

pub fn in_gaming_state(app_state: Res<State<AppState>>) -> bool {
    app_state.0 == AppState::Gaming
}