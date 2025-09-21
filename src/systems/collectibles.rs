use crate::components::collectible::{Collectible, CollectionStats};
use bevy::prelude::*;
use bevy::render::texture::Image;
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[derive(Resource, Default)]
pub struct CollectibleFloodState {
    pub has_spawned: bool,
}

const COLLECTIBLE_RADIUS: f32 = 12.0;
const TEXTURE_SIZE: f32 = 256.0;
const DOWNSCALE_FACTOR: usize = 4;
const COLLECTIBLE_GRID_SIZE: f32 = 100.0;
const JITTER_FRACTION: f32 = 0.3;

pub fn flood_spawn_collectibles_from_map(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    level_assets: Res<crate::systems::level::LevelAssets>,
    mut state: ResMut<CollectibleFloodState>,
) {
    if state.has_spawned {
        return;
    }

    let Some(image) = images.get(&level_assets.background) else {
        return;
    };

    let full_w = image.size().x as usize;
    let full_h = image.size().y as usize;
    let stride = 4;
    let data = &image.data;

    let w = full_w / DOWNSCALE_FACTOR;
    let h = full_h / DOWNSCALE_FACTOR;

    let mut solid = vec![vec![false; w]; h];
    let mut visited = vec![vec![false; w]; h];

    for y in 0..h {
        for x in 0..w {
            let orig_x = x * DOWNSCALE_FACTOR;
            let orig_y = y * DOWNSCALE_FACTOR;
            let idx = (orig_y * full_w + orig_x) * stride;

            let r = data[idx];
            let g = data[idx + 1];
            let b = data[idx + 2];
            let a = data[idx + 3];

            solid[y][x] = crate::systems::level::is_clearly_blue(r, g, b, a);
        }
    }

    let tile_size = DOWNSCALE_FACTOR as f32;
    let origin_offset = crate::systems::level::compute_origin_offset(image, tile_size);

    // Find player spawn tile
    let mut start_x = None;
    let mut start_y = None;

    'outer: for y in 0..h {
        for x in 0..w {
            let world_x = origin_offset.x + (x as f32 * tile_size);
            let world_y = origin_offset.y - (y as f32 * tile_size);
            if (world_x - 0.0).abs() < tile_size && (world_y - 200.0).abs() < tile_size {
                start_x = Some(x);
                start_y = Some(y);
                break 'outer;
            }
        }
    }

    let (start_x, start_y) = match (start_x, start_y) {
        (Some(x), Some(y)) => (x, y),
        _ => {
            warn!("Could not locate player start tile for collectible flood!");
            return;
        }
    };

    let texture = asset_server.load("textures/collectible.png");
    let scale = (COLLECTIBLE_RADIUS * 2.0) / TEXTURE_SIZE;

    let mut rng = rand::thread_rng();
    let mut queue = vec![(start_x, start_y)];
    let mut placed_positions = std::collections::HashSet::<(i32, i32)>::new();

    while let Some((x, y)) = queue.pop() {
        if x >= w || y >= h || visited[y][x] || solid[y][x] {
            continue;
        }

        visited[y][x] = true;

        let world_x = origin_offset.x + (x as f32 * tile_size);
        let world_y = origin_offset.y - (y as f32 * tile_size);

        // Snap to grid
        let snapped_x = (world_x / COLLECTIBLE_GRID_SIZE).round() as i32;
        let snapped_y = (world_y / COLLECTIBLE_GRID_SIZE).round() as i32;

        if placed_positions.insert((snapped_x, snapped_y)) {
            let snapped_world_x = snapped_x as f32 * COLLECTIBLE_GRID_SIZE;
            let snapped_world_y = snapped_y as f32 * COLLECTIBLE_GRID_SIZE;

            // Add jitter (±30% of grid size)
            let jitter_range = COLLECTIBLE_GRID_SIZE * JITTER_FRACTION;
            let jitter_x = rng.gen_range(-jitter_range..=jitter_range);
            let jitter_y = rng.gen_range(-jitter_range..=jitter_range);

            let final_x = snapped_world_x + jitter_x;
            let final_y = snapped_world_y + jitter_y;

            // Convert final world pos → downsampled solid[][] indices
            let image_x = ((final_x - origin_offset.x) / tile_size).round() as isize;
            let image_y = ((origin_offset.y - final_y) / tile_size).round() as isize;

            if image_x < 0 || image_x >= w as isize || image_y < 0 || image_y >= h as isize {
                continue;
            }
            let check_radius_px = COLLECTIBLE_RADIUS;
            let check_radius_cells = (check_radius_px / tile_size).ceil() as isize;

            let mut overlaps_blue = false;
            for dy in -check_radius_cells..=check_radius_cells {
                for dx in -check_radius_cells..=check_radius_cells {
                    let cx = image_x + dx;
                    let cy = image_y + dy;

                    if cx < 0 || cx >= w as isize || cy < 0 || cy >= h as isize {
                        overlaps_blue = true;
                        break;
                    }

                    if solid[cy as usize][cx as usize] {
                        overlaps_blue = true;
                        break;
                    }
                }
                if overlaps_blue {
                    break;
                }
            }

            if overlaps_blue {
                continue;
            }

            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(final_x, final_y, 1.0),
                        scale: Vec3::splat(scale),
                        ..default()
                    },
                    ..default()
                },
                Collectible,
                RigidBody::Fixed,
                Collider::ball(COLLECTIBLE_RADIUS),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(Group::GROUP_2, Group::ALL),
            ));
        }

        // Flood 4 neighbors
        if x > 0 {
            queue.push((x - 1, y));
        }
        if x + 1 < w {
            queue.push((x + 1, y));
        }
        if y > 0 {
            queue.push((x, y - 1));
        }
        if y + 1 < h {
            queue.push((x, y + 1));
        }
    }

    state.has_spawned = true;
    info!("✅ Collectibles placed via flood-fill with jitter.");
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
