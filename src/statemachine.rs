use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::level::{Facing, Player};

#[derive(Debug, Resource, Clone, Copy, Default, PartialEq, Eq, Reflect)]
#[reflect(Resource)]
pub enum PlayerState {
    #[default]
    Standing,
    Running,
    Dashing,
    Jumping,
    Climbing,
}

pub fn player_state_machine(
    mut q_player: Query<(&mut Velocity, &mut Facing), With<Player>>,
    mut player_state: ResMut<PlayerState>,
    mut player_on_ground: Local<bool>,
) {
    if q_player.is_empty() {
        return;
    }
    let (mut velocity, mut facing) = q_player.single_mut();
    if velocity.linvel.y.abs() < 0.1 && velocity.linvel.x.abs() < 0.1 {
        *player_state = PlayerState::Standing;
        return;
    }
    // TODO 检测是否在地面
    if velocity.linvel.y.abs() > 10.0 && *player_state != PlayerState::Climbing {
        *player_state = PlayerState::Jumping;
        return;
    }
}
