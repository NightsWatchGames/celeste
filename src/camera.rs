use bevy::prelude::*;
use rand::Rng;

use crate::{common::CAMERA_SCALE, level::Player};

// 相机最小移动距离，若小于此距离，则移动这个最小距离的长度
const CAMERA_MIN_MOVE_DISTANCE: f32 = 0.1;
// 每帧逼近剩余距离的百分比
const CAMERA_MOVE_INTERPOLATE: f32 = 0.05;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Event)]
pub struct CameraShakeEvent;

#[derive(Debug, Resource, Clone, Copy, Default, PartialEq, Eq)]
pub enum CameraState {
    #[default]
    Following,
    Shaking,
}

pub fn setup_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = CAMERA_SCALE;
    commands.spawn((Camera2d, projection));
}

// 相机跟随角色
pub fn camera_follow(
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player: Query<&Transform, With<Player>>,
    camera_state: Res<CameraState>,
) {
    if q_player.is_empty() {
        return;
    }
    if *camera_state != CameraState::Following {
        return;
    }
    let player_pos = q_player.single().translation.truncate();
    let camera_pos = q_camera.single().translation.truncate();
    let mut camera_transform = q_camera.single_mut();
    if camera_pos.distance(player_pos) < 0.1 {
        // 视为已达到player位置
        return;
    }
    if camera_pos.distance(player_pos) < CAMERA_MIN_MOVE_DISTANCE {
        // 直接移动到player位置
        camera_transform.translation.x = player_pos.x;
        camera_transform.translation.y = player_pos.y;
        return;
    }

    // 相机下一帧位置
    let camera_next_pos = camera_pos + (player_pos - camera_pos) * CAMERA_MOVE_INTERPOLATE;
    camera_transform.translation.x = camera_next_pos.x;
    camera_transform.translation.y = camera_next_pos.y;
}

// 相机抖动
pub fn camera_shake(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    mut camera_shake_er: EventReader<CameraShakeEvent>,
    mut shake_timer: Local<f32>,
    mut camera_state: ResMut<CameraState>,
    time: Res<Time>,
) {
    if !camera_shake_er.is_empty() {
        // 重置计时器，秒
        *shake_timer = 0.2;
        *camera_state = CameraState::Shaking;
        camera_shake_er.clear();
    }
    if *shake_timer > 0.0 {
        // 产生抖动效果
        *shake_timer -= time.delta_secs();
        let mut rng = rand::thread_rng();
        for mut camera in &mut q_camera {
            camera.translation.x += rng.gen_range(-1.0..1.0);
            camera.translation.y += rng.gen_range(-1.0..1.0);
        }
    } else {
        // 抖动结束
        *camera_state = CameraState::Following;
    }
}
