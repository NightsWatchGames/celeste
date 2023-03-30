use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::{AnimationBundle, AnimationIndices, AnimationTimer, TILE_SIZE},
    level::{Facing, Player, PlayerBundle, Trap, LEVEL_TRANSLATION_OFFSET},
};

// 玩家死亡
pub fn player_die(
    mut commands: Commands,
    mut collision_er: EventReader<CollisionEvent>,
    q_trap: Query<Entity, With<Trap>>,
) {
    for event in collision_er.iter() {
        match event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                if q_trap.contains(*entity1) {
                    info!("Player died");
                    commands.entity(*entity2).despawn_recursive();
                } else if q_trap.contains(*entity2) {
                    info!("Player died");
                    commands.entity(*entity1).despawn_recursive();
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    if q_player.is_empty() {
        for (transform, entity_instance) in &entity_query {
            if entity_instance.identifier == *"Player" {
                spawn_player(
                    &mut commands,
                    &mut texture_atlases,
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
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    player_pos: Vec2,
) {
    let texture_handle = asset_server.load("textures/atlas.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 11, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(PlayerBundle {
        player: Player,
        sprite_bundle: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(1),
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(player_pos.extend(10.0)),
            ..default()
        },
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
        rotation_constraints: LockedAxes::ROTATION_LOCKED,
        velocity: Velocity::zero(),
        gravity_scale: GravityScale(10.0),
    });
}

// 角色奔跑
pub fn player_run(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &mut Facing), With<Player>>,
) {
    for (mut velocity, mut facing) in &mut q_player {
        if keyboard_input.pressed(KeyCode::A) {
            velocity.linvel.x = -50.0;
            *facing = Facing::Left;
        } else if keyboard_input.pressed(KeyCode::D) {
            velocity.linvel.x = 50.0;
            *facing = Facing::Right;
        } else {
            // 不按键时停止左右移动
            velocity.linvel.x = 0.0;
        }
    }
}

// 角色跳跃
pub fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut q_player {
        // 没有y轴速度，防止二段跳
        if keyboard_input.pressed(KeyCode::K) && velocity.linvel.y.abs() < 0.1 {
            velocity.linvel = Vec2::new(0.0, 300.0);
        }
    }
}

// 角色冲刺/冲撞
pub fn player_dash(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &Facing), With<Player>>,
) {
    for (mut velocity, facing) in &mut q_player {
        if keyboard_input.pressed(KeyCode::J) {
            if *facing == Facing::Left {
                velocity.linvel = Vec2::new(-100.0, 0.0);
            }
            if *facing == Facing::Right {
                velocity.linvel = Vec2::new(100.0, 0.0);
            }
        }
    }
}

// 地面奔跑动画
pub fn animate_run(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<
        (
            &Facing,
            &mut AnimationTimer,
            &mut AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    // TODO 暂时通过这个判断处于run状态
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::D]) {
        for (facing, mut timer, mut indices, mut sprite) in &mut q_player {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                // 切换到下一个sprite
                sprite.index = if indices.index == indices.sprite_indices.len() - 1 {
                    indices.index = 0;
                    indices.sprite_indices[indices.index]
                } else {
                    indices.index += 1;
                    indices.sprite_indices[indices.index]
                };
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
    mut q_player: Query<(&Velocity, &Facing, &mut TextureAtlasSprite), With<Player>>,
) {
    for (velocity, facing, mut sprite) in &mut q_player {
        // TODO 暂时通过这个判断处于jump状态
        if velocity.linvel.y.abs() > 0.1 {
            sprite.index = 3;
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
    mut q_player: Query<(&Velocity, &Facing, &mut TextureAtlasSprite), With<Player>>,
) {
    for (velocity, facing, mut sprite) in &mut q_player {
        // TODO 暂时通过这个判断处于stand状态
        if velocity.linvel.x.abs() < 0.1 && velocity.linvel.y.abs() < 0.1 {
            sprite.index = 1;
            if *facing == Facing::Left {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        }
    }
}
