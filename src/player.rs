use bevy::color;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::VecDeque;

use crate::{
    camera::CameraShakeEvent,
    common::{
        AnimationBundle, AnimationIndices, AnimationTimer, PLAYER_DASHING_COLOR, PLAYER_DASH_SPEED,
        PLAYER_GRAVITY_SCALE, PLAYER_JUMP_SPEED, PLAYER_RUN_SPEED, PLAYER_SLIDE_SPEED,
        SPRITE_DUST_ORDER, SPRITE_HAIR_ORDER, SPRITE_PLAYER_ORDER, TILE_SIZE,
    },
    level::{Player, PlayerBundle, Snowdrift, Terrain, Trap, LEVEL_TRANSLATION_OFFSET},
    state_machine::PlayerState,
};

// 脸朝向
#[derive(Debug, Component, Clone, Copy, Default, PartialEq, Eq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

// 角色头发
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Hair;

// 灰尘
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Dust;

// 冲刺开始事件
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Event)]
pub struct DashStartEvent;
// 冲刺结束事件
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Event)]
pub struct DashOverEvent;

// 角色是否在地面上
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerGrounded(pub bool);

#[derive(Debug, Default, Resource)]
pub struct PlayerCannotMoveTime(pub f32);

// 角色是否挨着左边/右边的东西
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerNextTo(pub Option<NextToSomething>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum NextToSomething {
    LeftNext,
    RightNext,
}

// 玩家死亡
pub fn player_die(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut collision_er: EventReader<CollisionEvent>,
    mut camera_shake_ew: EventWriter<CameraShakeEvent>,
    q_trap: Query<Entity, With<Trap>>,
    q_player: Query<&Transform, With<Player>>,
) {
    for event in collision_er.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                if q_trap.contains(*entity1) {
                    info!("Player died");
                    commands.entity(*entity2).despawn_recursive();
                    spawn_dust(
                        &mut commands,
                        &mut atlas_layouts,
                        &asset_server,
                        q_player.single().translation.truncate(),
                        Color::default(),
                    );
                    camera_shake_ew.send_default();
                } else if q_trap.contains(*entity2) {
                    info!("Player died");
                    commands.entity(*entity1).despawn_recursive();
                    spawn_dust(
                        &mut commands,
                        &mut atlas_layouts,
                        &asset_server,
                        q_player.single().translation.truncate(),
                        Color::default(),
                    );
                    camera_shake_ew.send_default();
                }
            }
            _ => {}
        }
    }
}

// 玩家复活
pub fn player_revive(
    mut commands: Commands,
    q_player: Query<(), With<Player>>,
    entity_query: Query<(&Transform, &EntityInstance)>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    if q_player.is_empty() {
        for (transform, entity_instance) in &entity_query {
            if entity_instance.identifier == *"Player" {
                spawn_player(
                    &mut commands,
                    &mut atlas_layouts,
                    &asset_server,
                    (transform.translation + LEVEL_TRANSLATION_OFFSET).truncate(),
                );
                break;
            }
        }
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<AssetServer>,
    player_pos: Vec2,
) {
    let texture_handle = asset_server.load("textures/atlas.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(8, 8), 16, 11, None, None);
    let atlas_layout_handle = atlas_layouts.add(texture_atlas);

    commands.spawn(PlayerBundle {
        player: Player,
        sprite: Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                index: 1,
                layout: atlas_layout_handle,
            }),
            ..default()
        },
        transform: Transform::from_translation(player_pos.extend(SPRITE_PLAYER_ORDER)),
        animation_bundle: AnimationBundle {
            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            indices: AnimationIndices {
                index: 0,
                sprite_indices: vec![1, 2, 3, 4],
            },
        },
        facing: Facing::Right,
        collider: Collider::ball(TILE_SIZE / 2.0),
        rigid_body: RigidBody::Dynamic,
        restitution: Restitution::new(0.0),
        rotation_constraints: LockedAxes::ROTATION_LOCKED,
        velocity: Velocity::zero(),
        gravity_scale: GravityScale(PLAYER_GRAVITY_SCALE),
    });
}

// 角色奔跑
pub fn player_run(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<&mut Velocity, With<Player>>,
    player_state: Res<PlayerState>,
) {
    if q_player.is_empty() {
        return;
    }
    if *player_state == PlayerState::Running || *player_state == PlayerState::Standing {
        let mut velocity = q_player.single_mut();
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.linvel.x = -PLAYER_RUN_SPEED;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.linvel.x = PLAYER_RUN_SPEED;
        } else {
            // 不按键时停止左右奔跑
            velocity.linvel.x = 0.0;
        }
    }
}

// 角色左右移动（空中）
pub fn player_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<&mut Velocity, With<Player>>,
    player_state: Res<PlayerState>,
    player_next_to: Res<PlayerNextTo>,
    mut player_cannot_move_time: ResMut<PlayerCannotMoveTime>,
    time: Res<Time>,
) {
    if player_cannot_move_time.0 > 0.0 {
        player_cannot_move_time.0 -= time.delta_secs();
    }
    if q_player.is_empty() {
        return;
    }
    if player_cannot_move_time.0 > 0.0 {
        // 无法移动
        return;
    }
    if *player_state == PlayerState::Jumping || *player_state == PlayerState::Climbing {
        let mut velocity = q_player.single_mut();
        if keyboard_input.pressed(KeyCode::KeyA) {
            if player_next_to.0.is_some() && player_next_to.0.unwrap() == NextToSomething::LeftNext
            {
            } else {
                velocity.linvel.x = -PLAYER_RUN_SPEED;
            }
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            if player_next_to.0.is_some() && player_next_to.0.unwrap() == NextToSomething::RightNext
            {
            } else {
                velocity.linvel.x = PLAYER_RUN_SPEED;
            }
        } else {
            // 不按键时停止左右移动
            velocity.linvel.x = 0.0;
        }
    }
}

// 角色跳跃
pub fn player_jump(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &Transform), With<Player>>,
    player_state: Res<PlayerState>,
    player_next_to: Res<PlayerNextTo>,
    mut player_cannot_move_time: ResMut<PlayerCannotMoveTime>,
) {
    if q_player.is_empty() {
        return;
    }
    if *player_state == PlayerState::Standing
        || *player_state == PlayerState::Running
        || *player_state == PlayerState::Climbing
    {
        let (mut velocity, transform) = q_player.single_mut();
        // TODO 可允许角色在非跳跃进入下坠状态时能够进行跳跃
        if keyboard_input.just_pressed(KeyCode::KeyK) {
            if keyboard_input.pressed(KeyCode::KeyS) {
                // 同时按下和跳跃键，不向上跳
                return;
            }
            if *player_state == PlayerState::Climbing {
                // 蹬墙跳
                if player_next_to.0.is_some()
                    && player_next_to.0.unwrap() == NextToSomething::LeftNext
                {
                    velocity.linvel = Vec2::new(100., 200.);
                }
                if player_next_to.0.is_some()
                    && player_next_to.0.unwrap() == NextToSomething::RightNext
                {
                    velocity.linvel = Vec2::new(-100., 200.);
                }
                player_cannot_move_time.0 = 0.2;
            } else {
                velocity.linvel = Vec2::new(0.0, PLAYER_JUMP_SPEED);
            }
            spawn_dust(
                &mut commands,
                &mut atlas_layouts,
                &asset_server,
                transform.translation.truncate(),
                Color::default(),
            );
        }
    }
}

// 角色冲刺/冲撞
pub fn player_dash(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &Facing, &Transform, &mut GravityScale), With<Player>>,
    mut spawn_dust_cd: Local<f32>,
    mut dash_timer: Local<f32>,
    mut camera_shake_ew: EventWriter<CameraShakeEvent>,
    mut dash_start_ew: EventWriter<DashStartEvent>,
    mut dash_over_ew: EventWriter<DashOverEvent>,
    player_state: Res<PlayerState>,
    time: Res<Time>,
) {
    if q_player.is_empty() {
        return;
    }
    // 冲刺期间不能再次冲刺
    if keyboard_input.just_pressed(KeyCode::KeyJ) && *player_state != PlayerState::Dashing {
        *dash_timer = 0.2;
        dash_start_ew.send_default();
        camera_shake_ew.send_default();
    }

    let (mut velocity, facing, transform, mut gravity_scale) = q_player.single_mut();
    if *dash_timer > 0.0 && *player_state == PlayerState::Dashing {
        *dash_timer -= time.delta_secs();
        if *facing == Facing::Left {
            velocity.linvel = Vec2::new(-PLAYER_DASH_SPEED, 0.0);
        } else if *facing == Facing::Right {
            velocity.linvel = Vec2::new(PLAYER_DASH_SPEED, 0.0);
        }
        // 重力为0
        gravity_scale.0 = 0.0;

        if *spawn_dust_cd > 0.0 {
            *spawn_dust_cd -= time.delta_secs();
        } else {
            spawn_dust(
                &mut commands,
                &mut atlas_layouts,
                &asset_server,
                transform.translation.truncate(),
                PLAYER_DASHING_COLOR,
            );
            // 重置cd
            *spawn_dust_cd = 0.02;
        }
        if *dash_timer <= 0.0 {
            // 冲刺自然结束（未产生碰撞）
            dash_over_ew.send_default();
        }
    }
}

pub fn player_dash_over(
    mut q_player: Query<(&mut Velocity, &mut GravityScale), With<Player>>,
    mut dash_over_er: EventReader<DashOverEvent>,
) {
    if q_player.is_empty() {
        return;
    }
    let (mut velocity, mut gravity_scale) = q_player.single_mut();
    if dash_over_er.read().next().is_some() {
        velocity.linvel.x = 0.0;
        gravity_scale.0 = PLAYER_GRAVITY_SCALE;
    }
}

// 角色爬墙
pub fn player_climb(
    mut q_player: Query<(&mut Velocity, &mut GravityScale), With<Player>>,
    player_state: Res<PlayerState>,
    mut last_player_state: Local<PlayerState>,
) {
    if q_player.is_empty() {
        return;
    }
    let (mut velocity, mut gravity_scale) = q_player.single_mut();
    // 进入Climbing
    if *player_state == PlayerState::Climbing && *last_player_state != PlayerState::Climbing {
        // 向下滑动
        velocity.linvel = Vec2::new(0.0, -PLAYER_SLIDE_SPEED);
        // 重力为0
        gravity_scale.0 = 0.0;
    }

    // 退出Climbing
    if *player_state != PlayerState::Climbing && *last_player_state == PlayerState::Climbing {
        gravity_scale.0 = PLAYER_GRAVITY_SCALE;
    }

    *last_player_state = *player_state;
}

// 地面奔跑动画
pub fn animate_run(
    mut q_player: Query<
        (
            &Facing,
            &mut AnimationTimer,
            &mut AnimationIndices,
            &mut Sprite,
        ),
        With<Player>,
    >,
    player_state: Res<PlayerState>,
    time: Res<Time>,
) {
    if *player_state == PlayerState::Running {
        for (facing, mut timer, mut indices, mut sprite) in &mut q_player {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                // 切换到下一个sprite
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if indices.index == indices.sprite_indices.len() - 1 {
                        indices.index = 0;
                        indices.sprite_indices[indices.index]
                    } else {
                        indices.index += 1;
                        indices.sprite_indices[indices.index]
                    };
                }
                if *facing == Facing::Left {
                    sprite.flip_x = true;
                } else {
                    sprite.flip_x = false;
                }
            }
        }
    }
}

// 跳跃动画
pub fn animate_jump(
    mut q_player: Query<(&Facing, &mut Sprite), With<Player>>,
    player_state: Res<PlayerState>,
) {
    if *player_state == PlayerState::Jumping {
        for (facing, mut sprite) in &mut q_player {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 3;
            }
            if *facing == Facing::Left {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        }
    }
}

// 站立动画
pub fn animate_stand(
    mut q_player: Query<(&Facing, &mut Sprite), With<Player>>,
    player_state: Res<PlayerState>,
) {
    if *player_state == PlayerState::Standing {
        for (facing, mut sprite) in &mut q_player {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 1;
            }
            if *facing == Facing::Left {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        }
    }
}

// 冲刺动画
pub fn animate_dash(
    mut q_player: Query<(&Facing, &mut Sprite), With<Player>>,
    player_state: Res<PlayerState>,
) {
    if *player_state == PlayerState::Dashing {
        for (facing, mut sprite) in &mut q_player {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 131;
            }
            if *facing == Facing::Left {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        }
    }
}

// 创建角色头发
pub fn spawn_hair(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    q_hair: Query<(), With<Hair>>,
    q_player: Query<&Transform, With<Player>>,
) {
    if !q_player.is_empty() && q_hair.is_empty() {
        let columns = 6;
        let texture_handle = asset_server.load("textures/hair.png");
        let atlas_layout = TextureAtlasLayout::from_grid(
            UVec2::new(8, 8),
            columns,
            1,
            Some(UVec2::new(1, 1)),
            Some(UVec2::new(1, 1)),
        );
        let atlas_layout_handle = atlas_layouts.add(atlas_layout);

        for player_transfrom in &q_player {
            for i in 0..columns {
                commands.spawn((
                    Hair,
                    Sprite {
                        image: texture_handle.clone(),
                        color: color::palettes::basic::RED.into(),
                        texture_atlas: Some(TextureAtlas {
                            index: i as usize,
                            layout: atlas_layout_handle.clone(),
                        }),
                        ..default()
                    },
                    Transform::from_translation(
                        player_transfrom
                            .translation
                            .truncate()
                            .extend(SPRITE_HAIR_ORDER),
                    ),
                ));
            }
        }
    }
}

pub fn despawn_hair(
    mut commands: Commands,
    q_hair: Query<Entity, With<Hair>>,
    q_player: Query<&Transform, With<Player>>,
) {
    if q_player.is_empty() && !q_hair.is_empty() {
        for entity in &q_hair {
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_player_collision(
    q_player: Query<(&Velocity, &GravityScale), With<Player>>,
    q_snowdrift: Query<(), With<Snowdrift>>,
    mut collision_er: EventReader<CollisionEvent>,
    mut dash_over_ew: EventWriter<DashOverEvent>,
    player_state: Res<PlayerState>,
    mut player_cannot_move_time: ResMut<PlayerCannotMoveTime>,
) {
    if q_player.is_empty() {
        return;
    }
    for collision in collision_er.read() {
        match collision {
            CollisionEvent::Started(entity1, entity2, _) => {
                // 蹬墙跳后产生碰撞时，立刻解除不能移动的限制
                player_cannot_move_time.0 = 0.0;
                let (_player_entity, other_entity) = if q_player.contains(*entity1) {
                    (*entity1, *entity2)
                } else if q_player.contains(*entity2) {
                    (*entity2, *entity1)
                } else {
                    continue;
                };
                info!("Player collision");
                // 碰撞到雪堆
                if q_snowdrift.contains(other_entity) {
                    info!("Player collision with snowdrift");
                    if *player_state == PlayerState::Dashing {
                        dash_over_ew.send_default();
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn animate_hair(
    mut q_hair: Query<(&mut Transform, &mut Sprite), (With<Hair>, Without<Player>)>,
    q_player: Query<(&Transform, &Facing), With<Player>>,
    player_state: Res<PlayerState>,
    mut hair_flow: Local<VecDeque<Vec2>>,
) {
    if q_player.is_empty() || q_hair.is_empty() {
        return;
    }
    // hair_flow记录最近5帧player位置
    hair_flow.push_front(q_player.single().0.translation.truncate());
    if hair_flow.len() > 5 {
        hair_flow.pop_back();
    }

    // 头发排序
    let mut hair: Vec<(Mut<Transform>, Mut<Sprite>)> = q_hair.iter_mut().collect();
    hair.sort_by(|single_hair1, single_hair2| {
        single_hair1
            .1
            .texture_atlas
            .as_ref()
            .unwrap()
            .index
            .cmp(&single_hair2.1.texture_atlas.as_ref().unwrap().index)
    });

    // 依次往每个bucket（hair_flow槽）里放，直至放满，多余hair放到最后的bucket
    let bucket_size = hair.iter().len() / hair_flow.len();
    let mut bucket_index = 0;
    let mut count = 0;
    for single_hair in hair.iter_mut() {
        single_hair.0.translation = hair_flow
            .get(bucket_index)
            .unwrap()
            .extend(SPRITE_HAIR_ORDER);
        single_hair.1.flip_x = if *q_player.single().1 == Facing::Left {
            true
        } else {
            false
        };
        count += 1;
        if count > bucket_size {
            // 下一个bucket
            bucket_index = if bucket_index < hair_flow.len() - 1 {
                bucket_index + 1
            } else {
                bucket_index
            };
            count = 0;
        }
    }

    // 冲刺期间头发变色
    if *player_state == PlayerState::Dashing {
        for (_, mut sprite) in &mut q_hair {
            sprite.color = PLAYER_DASHING_COLOR;
        }
    } else {
        for (_, mut sprite) in &mut q_hair {
            sprite.color = color::palettes::basic::RED.into();
        }
    }
}

pub fn spawn_dust(
    commands: &mut Commands,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<AssetServer>,
    dust_pos: Vec2,
    dust_color: Color,
) {
    let texture_handle = asset_server.load("textures/atlas.png");
    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(8, 8), 16, 11, None, None);
    let atlas_layout_handle = atlas_layouts.add(atlas_layout);

    commands.spawn((
        Dust,
        Sprite {
            image: texture_handle,
            color: dust_color,
            texture_atlas: Some(TextureAtlas {
                index: 29,
                layout: atlas_layout_handle,
            }),
            ..default()
        },
        Transform::from_translation(dust_pos.extend(SPRITE_DUST_ORDER)),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AnimationIndices {
            index: 0,
            sprite_indices: vec![29, 30, 31],
        },
    ));
}

pub fn animate_dust(
    mut commands: Commands,
    mut q_dust: Query<
        (
            Entity,
            &mut AnimationTimer,
            &mut AnimationIndices,
            &mut Sprite,
        ),
        With<Dust>,
    >,
    time: Res<Time>,
) {
    for (entity, mut timer, mut indices, mut sprite) in &mut q_dust {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // 切换到下一个sprite
            if let Some(atlas) = &mut sprite.texture_atlas {
                if indices.index == indices.sprite_indices.len() - 1 {
                    commands.entity(entity).despawn();
                } else {
                    indices.index += 1;
                    atlas.index = indices.sprite_indices[indices.index];
                };
            }
        }
    }
}

pub fn player_grounded_detect(
    q_player: Query<&Transform, With<Player>>,
    mut player_grounded: ResMut<PlayerGrounded>,
    mut last: Local<(f32, isize)>,
) {
    if q_player.is_empty() {
        return;
    }
    // 通过检测y轴坐标连续多帧是否变化来判断落地
    let pos = q_player.single().translation.truncate();
    if (pos.y * 10.).round() == last.0 {
        last.1 += 1;
    } else {
        last.1 -= 1;
    }
    last.1 = last.1.clamp(0, 5);

    if last.1 == 5 && !player_grounded.0 {
        player_grounded.0 = true;
    } else if last.1 < 2 && player_grounded.0 {
        player_grounded.0 = false;
    }

    last.0 = (pos.y * 10.).round();
}

pub fn player_next_to_detect(
    rapier_context: Single<&RapierContext>,
    q_player: Query<&Transform, With<Player>>,
    q_terrain: Query<&GlobalTransform, With<Terrain>>,
    mut player_next_to: ResMut<PlayerNextTo>,
) {
    if q_player.is_empty() {
        return;
    }
    let player_pos = q_player.single().translation.truncate();
    if let Some((entity, _toi)) = rapier_context.cast_ray(
        player_pos + Vec2::new(-TILE_SIZE / 2. - 0.1, 0.),
        Vec2::NEG_X,
        1.0,
        true,
        QueryFilter::default(),
    ) {
        if q_terrain.contains(entity) {
            player_next_to.0 = Some(NextToSomething::LeftNext);
        }
    } else if let Some((entity, _toi)) = rapier_context.cast_ray(
        player_pos + Vec2::new(TILE_SIZE / 2. + 0.1, 0.),
        Vec2::X,
        1.0,
        true,
        QueryFilter::default(),
    ) {
        if q_terrain.contains(entity) {
            player_next_to.0 = Some(NextToSomething::RightNext);
        }
    } else {
        player_next_to.0 = None;
    }
}

pub fn player_facing_update(mut q_player: Query<(&Velocity, &mut Facing), With<Player>>) {
    if q_player.is_empty() {
        return;
    }
    let (velocity, mut facing) = q_player.single_mut();
    if velocity.linvel.x > 0. {
        *facing = Facing::Right;
    } else if velocity.linvel.x < 0. {
        *facing = Facing::Left;
    }
}
