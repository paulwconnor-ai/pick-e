use bevy::log::info;
use bevy::math::primitives::Circle;
use bevy::prelude::*;
use bevy::render::camera::{Projection, ScalingMode};
use bevy::render::mesh::Mesh;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-canvas".into()),
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

    let logical_width = 800.0;
    let logical_height = 600.0;

    commands.spawn((
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(1.0, 1.0, 1.0)),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: logical_width,
                height: logical_height,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, 0.0, 999.0),
        GlobalTransform::default(),
    ));

    let mesh_handle = meshes.add(Mesh::from(Circle::new(50.0)));
    let mat_handle = materials.add(ColorMaterial::from(Color::srgb_u8(201, 230, 240)));

    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(mat_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
