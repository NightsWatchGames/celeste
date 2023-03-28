use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2dPlugin;
use bevy::{
    reflect::TypeUuid,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use bevy::prelude::*;

// Plugin that will insert Weather at Z = -10.0, use the custom 'Star Nest' shader
pub struct WeatherPlugin;
impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<WeatherMaterial>::default())
            .add_startup_system(spawn_weather)
            .add_system(update_weather_time);
    }
}

// Spawn a simple stretched quad that will use of Weather shader
fn spawn_weather(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WeatherMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(400., 265., 1.0), //适配分辨率 0.3倍
            ..Default::default()
        },
        material: materials.add(WeatherMaterial { time: 0.0 }),
        ..Default::default()
    });
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
struct WeatherMaterial {
    #[uniform(0)]
    time: f32,
}

impl Material2d for WeatherMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/weather.wgsl".into()
    }
}

fn update_weather_time(time: Res<Time>, mut weathers: ResMut<Assets<WeatherMaterial>>) {
    for (_, weather) in weathers.iter_mut() {
        weather.time += time.delta_seconds();
    }
}
