use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection, Projection, ScalingMode};

use crate::constants::{LOGICAL_H, LOGICAL_W};

pub fn camera_2d_bundle() -> Camera2dBundle {
    Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::rgb(1.0, 1.0, 1.0)),
            ..default()
        },
        ..default()
    }
}

/*pub fn camera_2d_bundle() -> impl Bundle {
    (
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::rgb(1.0, 1.0, 1.0)),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: LOGICAL_W,
                height: LOGICAL_H,
            },
            ..OrthographicProjection::default()
        }),
        Transform::default(),
        GlobalTransform::default(),
    )
}*/
