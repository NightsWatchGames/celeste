use bevy::color;
use bevy::prelude::*;

use crate::common::AppState;

#[derive(Component)]
pub struct OnStartMenuScreen;

pub fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("textures/atlas.png");
    let mut atlas_layout = TextureAtlasLayout::new_empty(UVec2::new(128, 88));
    atlas_layout.add_texture(URect {
        min: UVec2::new(72 + 1, 32 + 1),
        max: UVec2::new(128, 64),
    });
    let atlas_layout_handle = atlas_layouts.add(atlas_layout);

    commands.spawn((
        OnStartMenuScreen,
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: atlas_layout_handle,
            }),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 1.0, 1.0),
            scale: Vec3::new(3., 3., 1.0), //放大3倍
            ..Default::default()
        },
    ));

    commands
        .spawn((
            OnStartMenuScreen,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::top(Val::Px(200.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PRESS ENTER KEY"),
                TextColor(color::palettes::basic::GRAY.into()),
                TextFont {
                    font: asset_server.load("fonts/ThaleahFat_TTF.ttf"),
                    font_size: 40.0,
                    ..default()
                },
            ));
        });
}

pub fn enter_gaming(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
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
