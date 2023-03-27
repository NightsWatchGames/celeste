use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::TILE_SIZE;

pub const LEVEL_TRANSLATION_OFFSET: Vec3 = Vec3::new(-250.0, -200.0, 0.0);

// 陷阱
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Trap;

// 弹簧
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Spring;

// 雪堆
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Snowdrift;

// 木架
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct WoodenStand;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct TerrainBundle {
    #[from_int_grid_cell]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct SpringBundle {
    pub spring: Spring,
    #[sprite_sheet_bundle("textures/atlas.png", 8.0, 8.0, 16, 11, 0.0, 0.0, 19)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct TrapBundle {
    pub trap: Trap,
    #[sprite_sheet_bundle("textures/atlas.png", 8.0, 8.0, 16, 11, 0.0, 0.0, 17)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Clone, Default, Bundle)]
pub struct WoodenStandBundle {
    pub wooden_stand: WoodenStand,
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct SnowdriftBundle {
    pub snowdrift: Snowdrift,
    #[sprite_sheet_bundle("textures/atlas.png", 16.0, 16.0, 8, 5, 0.0, 0.0, 16)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        if int_grid_cell.value == 1 {
            ColliderBundle {
                collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
                rigid_body: RigidBody::Fixed,
            }
        } else {
            panic!("Unsupported int grid cell value")
        }
    }
}

pub fn setup_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        transform: Transform::from_translation(Vec3::ZERO + LEVEL_TRANSLATION_OFFSET),
        ..Default::default()
    });
}

pub fn spawn_ldtk_entity(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, entity_instance) in entity_query.iter() {
        println!("{:?}, {:?}", entity_instance, transform.translation);
        if entity_instance.identifier == *"WoodenStand" {
            let texture_handle = asset_server.load("textures/atlas.png");
            let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(128., 88.));
            texture_atlas.add_texture(Rect {
                min: Vec2::new(72., 16.),
                max: Vec2::new(88., 24.),
            });
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let mut translation = transform.translation + LEVEL_TRANSLATION_OFFSET;
            translation.z = 10.0;
            commands.spawn(WoodenStandBundle {
                wooden_stand: WoodenStand,
                sprite_bundle: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        color: Color::GREEN,
                        index: 0,
                        ..default()
                    },
                    texture_atlas: texture_atlas_handle,
                    transform: Transform::from_translation(translation),
                    ..default()
                },
            });
        }
    }
}
