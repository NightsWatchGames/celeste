use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    level::{Facing, Player},
    player::{DashOverEvent, DashStartEvent, PlayerGrounded},
};

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
    player_grounded: Res<PlayerGrounded>,
    mut dash_start_er: EventReader<DashStartEvent>,
    mut dash_over_er: EventReader<DashOverEvent>,
) {
    if q_player.is_empty() {
        return;
    }
    if dash_start_er.iter().next().is_some() {
        *player_state = PlayerState::Dashing;
        return;
    }
    if *player_state == PlayerState::Dashing && dash_over_er.iter().next().is_none() {
        // 持续保持dashing状态直至接收到DashOverEvent
        return;
    }
    let (mut velocity, mut facing) = q_player.single_mut();
    // Standing状态
    if player_grounded.0 && velocity.linvel.x.abs() < 0.1 {
        *player_state = PlayerState::Standing;
        return;
    }
    // Running状态
    if player_grounded.0 && velocity.linvel.x.abs() > 1.0 {
        *player_state = PlayerState::Running;
        return;
    }
    // Jumping状态
    // TODO 不在爬墙状态
    if !player_grounded.0 {
        *player_state = PlayerState::Jumping;
        return;
    }
}
