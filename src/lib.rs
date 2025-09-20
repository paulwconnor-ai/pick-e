// src/lib.rs

use bevy::log::info;
use bevy::prelude::*;
use bevy::render::camera::{OrthographicProjection, Projection, ScalingMode};

pub const LOGICAL_W: f32 = 1280.0;
pub const LOGICAL_H: f32 = 720.0;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    real_start();
}

pub fn real_start() {
    // Show panics and logs
    #[cfg(target_arch = "wasm32")]
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

fn setup(mut commands: Commands) {
    info!("Pick.e starting (logical size {LOGICAL_W}×{LOGICAL_H})");

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

    commands.spawn((
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
