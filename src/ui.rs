use bevy::prelude::*;

use crate::common::AppState;

#[derive(Component)]
pub struct OnStartMenuScreen;

pub fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/atlas.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(128., 88.));
    texture_atlas.add_texture(Rect {
        min: Vec2::new(72. + 0.5, 32. + 0.5),
        max: Vec2::new(128., 64.),
    });
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        OnStartMenuScreen,
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 1.0, 1.0),
                scale: Vec3::new(3., 3., 1.0), //放大3倍
                ..Default::default()
            },
            ..default()
        },
    ));

    commands
        .spawn((
            OnStartMenuScreen,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::top(Val::Px(200.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "PRESS ENTER KEY",
                    TextStyle {
                        font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                        font_size: 40.0,
                        color: Color::GRAY,
                    },
                ),
                ..default()
            });
        });
}

pub fn enter_gaming(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        app_state.set(AppState::Gaming);
    }
}

pub fn cleanup_start_menu(
    mut commands: Commands,
    q_start_menu: Query<Entity, With<OnStartMenuScreen>>,
) {
    for entity in &q_start_menu {
        commands.entity(entity).despawn_recursive();
    }
}
