use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    level::Player,
    player::{DashOverEvent, DashStartEvent, NextToSomething, PlayerGrounded, PlayerNextTo},
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_player: Query<&Velocity, With<Player>>,
    mut player_state: ResMut<PlayerState>,
    player_grounded: Res<PlayerGrounded>,
    player_next_to: Res<PlayerNextTo>,
    mut dash_start_er: EventReader<DashStartEvent>,
    mut dash_over_er: EventReader<DashOverEvent>,
) {
    if q_player.is_empty() {
        return;
    }
    if dash_start_er.read().next().is_some() {
        *player_state = PlayerState::Dashing;
        return;
    }
    if *player_state == PlayerState::Dashing && dash_over_er.read().next().is_none() {
        // 持续保持dashing状态直至接收到DashOverEvent
        return;
    }
    let velocity = q_player.single();
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
    // Climbing状态
    if player_next_to.0.is_some() {
        if player_next_to.0.unwrap() == NextToSomething::LeftNext
            && keyboard_input.pressed(KeyCode::KeyA)
            || player_next_to.0.unwrap() == NextToSomething::RightNext
                && keyboard_input.pressed(KeyCode::KeyD)
        {
            *player_state = PlayerState::Climbing;
            return;
        }
    }
    // Jumping状态
    if !player_grounded.0 {
        *player_state = PlayerState::Jumping;
        return;
    }
}
