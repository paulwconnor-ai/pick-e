use bevy::math::primitives::Circle;
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;
use bevy::render::mesh::Mesh;
use bevy::sprite::MaterialMesh2dBundle;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    // Show panics in the browser console
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Pick.e getting set up...");

    // Camera with a white clear-colour
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::rgb(1.0, 1.0, 1.0)),
            ..default()
        },
        ..default()
    });

    // Circle (explicit type)
    let mesh = meshes.add(Mesh::from(Circle::new(50.0)));
    let material = materials.add(ColorMaterial::from(Color::rgb_u8(201, 230, 240)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh.into(),
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
