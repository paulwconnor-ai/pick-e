use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy_rapier2d::prelude::*; // Assumes you're using Rapier for physics

// Marker for input control
#[derive(Component)]
pub struct HeroController;

// Controls the size of the hero sprite and collider
pub const HERO_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub const HERO_RADIUS: f32 = HERO_SIZE.x * 0.5;

pub fn hero_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        sprite_bundle(asset_server),
        physics_bundle(),
        HeroController,
        Name::new("Hero"),
    )
}

fn sprite_bundle(asset_server: &AssetServer) -> SpriteBundle {
    SpriteBundle {
        texture: asset_server.load("textures/hero.png"),
        sprite: Sprite {
            custom_size: Some(HERO_SIZE),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }
}

fn physics_bundle() -> impl Bundle {
    (
        RigidBody::Dynamic,
        Collider::ball(HERO_RADIUS),
        Velocity::default(), // You can modify velocity via system each frame
        LockedAxes::ROTATION_LOCKED, // Prevent spinning
        Damping {
            linear_damping: 2.0,
            angular_damping: 2.0,
        },
    )
}
