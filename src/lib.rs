use bevy::log::info;
use bevy::math::primitives::Circle;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-canvas".into()), // now it's targeting the actual canvas
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Pick.e getting set up...");

    // Camera: bundles are deprecated in 0.16; use components.
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(1.0, 1.0, 1.0)),
            ..default()
        },
    ));

    // Circle: use Mesh2d + MeshMaterial2d components (no MaterialMesh2dBundle in 0.16).
    let mesh_handle = meshes.add(Mesh::from(Circle::new(50.0)));
    let mat_handle = materials.add(ColorMaterial::from(Color::srgb_u8(201, 230, 240)));

    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(mat_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
