use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::level::Player;

// 角色左右移动
pub fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in &mut q_player {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 50.0 * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 50.0 * time.delta_seconds();
        }
    }
}

// 角色跳跃
pub fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut q_player {
        if keyboard_input.pressed(KeyCode::K) {
            velocity.linvel = Vec2::new(0.0, 100.0);
        }
    }
}