use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::constants::{LOGICAL_H, LOGICAL_W};

pub fn camera_2d_bundle() -> Camera2dBundle {
    Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::rgb(1.0, 1.0, 1.0)),
            ..default()
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: LOGICAL_W,
                height: LOGICAL_H,
            },
            near: -1000.0,
            far: 1000.0,
            ..default()
        }
        .into(),
        ..default()
    }
}
