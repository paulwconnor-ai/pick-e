// src/lib.rs

use bevy::log::info;
use bevy::prelude::*;
use bevy::render::camera::{OrthographicProjection, Projection, ScalingMode};
use wasm_bindgen::prelude::*;

const LOGICAL_W: f32 = 1280.0;
const LOGICAL_H: f32 = 720.0;

#[wasm_bindgen(start)]
pub fn start() {
    // Show Rust panics in the browser console
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Use your HTML canvas (index.html should have: <canvas id="bevy-canvas"></canvas>)
                canvas: Some("#bevy-canvas".into()),
                fit_canvas_to_parent: true, // let CSS parent control size
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    info!("Pick.e (camera-fixed world) starting…");

    // 2D camera with white clear color and a FIXED logical extent of 800x600.
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(1.0, 1.0, 1.0)),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: LOGICAL_W,
                height: LOGICAL_H,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::default(),
        GlobalTransform::default(),
    ));

    // Demo geometry: a light-blue square (100x100 logical units) at the origin.
    // (Using SpriteBundle here to avoid mesh feature churn; easy to swap later.)
    commands.spawn((
        // Colored, untextured quad (100x100 logical units)
        Sprite {
            color: Color::srgb_u8(201, 230, 240),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}
