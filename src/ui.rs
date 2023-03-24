use bevy::prelude::*;

pub fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/atlas.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(128., 88.));
    texture_atlas.add_texture(Rect {
        min: Vec2::new(72., 32.),
        max: Vec2::new(128., 64.),
    });
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(10.)),
        ..default()
    });
}
