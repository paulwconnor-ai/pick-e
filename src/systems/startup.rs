use crate::bundles::camera::camera_2d_bundle;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(camera_2d_bundle());

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
