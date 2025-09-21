use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy_rapier2d::prelude::*;

use crate::components::cmd_vel::CmdVel;
use crate::components::lidar::*;
use crate::constants::HERO_RADIUS_PX;

/// Marker for input control
#[derive(Component)]
pub struct HeroController;

pub const HERO_RADIUS: f32 = HERO_RADIUS_PX;
pub const HERO_SIZE: Vec2 = Vec2::new(HERO_RADIUS * 2.0, HERO_RADIUS * 2.0);

pub fn hero_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        sprite_bundle(asset_server),
        physics_bundle(),
        perception_bundle(),
        HeroController,
        CmdVel::default(),
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
        transform: Transform::from_xyz(0.0, 200.0, 0.0),
        ..default()
    }
}

fn physics_bundle() -> impl Bundle {
    (
        RigidBody::Dynamic,
        Collider::ball(HERO_RADIUS),
        Velocity::default(),
        Damping {
            linear_damping: 2.0,
            angular_damping: 2.0,
        },
    )
}

pub fn perception_bundle() -> impl Bundle {
    (LidarSensor, LidarEmitter::default())
}
