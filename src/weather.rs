use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;
use bevy::sprite::Material2dPlugin;

use bevy::prelude::*;
use bevy::reflect::TypePath;

// Plugin that will insert Weather at Z = -10.0, use the custom 'Star Nest' shader
pub struct WeatherPlugin;
impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<WeatherMaterial>::default())
            .add_systems(Startup, (spawn_weather,))
            .add_systems(Update, (update_weather_time,));
    }
}

// Spawn a simple stretched quad that will use of Weather shader
fn spawn_weather(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WeatherMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Rectangle::default().mesh()))),
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(400., 265., 1.0), //适配分辨率 0.3倍
            ..Default::default()
        },
        MeshMaterial2d(materials.add(WeatherMaterial { time: 0.0 })),
    ));
}

#[derive(AsBindGroup, Debug, Clone, TypePath, Asset)]
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
        weather.time += time.delta_secs();
    }
}
