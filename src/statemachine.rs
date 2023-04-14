use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{level::{Facing, Player}, common::PLAYER_DASH_SPEED, player::{DashOverEvent, DashStartEvent}};

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
    if velocity.linvel.y.abs() < 0.1 && velocity.linvel.x.abs() < 0.1 {
        *player_state = PlayerState::Standing;
        return;
    }
    // Running状态
    if velocity.linvel.x.abs() > 10.0 && velocity.linvel.y.abs() < 10.0 {
        *player_state = PlayerState::Running;
        return;
    }
    // Dashing状态
    if (velocity.linvel.x.abs() - PLAYER_DASH_SPEED).abs() < 1.0 && velocity.linvel.y.abs() < 1.0 {
        dbg!(velocity.linvel);
        *player_state = PlayerState::Dashing;
        return;
    }
    // Jumping状态
    // TODO 检测是否在地面
    if velocity.linvel.y.abs() > 10.0 && *player_state != PlayerState::Climbing  {
        *player_state = PlayerState::Jumping;
        return;
    }
}
