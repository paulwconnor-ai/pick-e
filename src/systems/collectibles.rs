use crate::components::collectible::{Collectible, CollectionStats};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_collectibles(mut commands: Commands, asset_server: Res<AssetServer>) {
    const RADIUS: f32 = 25.0;
    const TEXTURE_SIZE: f32 = 256.0; // update to match your actual sprite
    let scale = RADIUS * 2.0 / TEXTURE_SIZE;

    let texture = asset_server.load("textures/collectible.png");

    let positions = [
        Vec2::new(100.0, 100.0),
        Vec2::new(-50.0, 150.0),
        Vec2::new(200.0, -75.0),
    ];

    for pos in positions {
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform {
                    translation: pos.extend(1.0),
                    scale: Vec3::splat(scale), // sprite matches collider
                    ..default()
                },
                ..default()
            },
            Collectible,
            RigidBody::Fixed,
            Collider::ball(RADIUS),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }

    info!("Collectibles spawned.");
}

pub fn collect_on_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    collectibles: Query<Entity, With<Collectible>>,
    mut stats: ResMut<CollectionStats>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (collectible_entity, _) = if collectibles.contains(*e1) {
                (*e1, *e2)
            } else if collectibles.contains(*e2) {
                (*e2, *e1)
            } else {
                continue;
            };

            commands.entity(collectible_entity).despawn();
            stats.collected += 1;
            info!("🎉 Collected! Total: {}", stats.collected);
        }
    }
}
