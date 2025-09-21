use crate::components::lidar::LidarEmitter;
use crate::components::occupancy_grid::{CellState, OccupancyGrid};
use crate::constants::OCCUPANCY_ASSUMED_MAX_LIDAR_RANGE_PX;

use bevy::prelude::*;

/// Updates the occupancy grid using LIDAR hits (per-entity)
pub fn update_occupancy_grid_system(
    mut query: Query<(&GlobalTransform, &LidarEmitter, &mut OccupancyGrid)>,
) {
    for (transform, emitter, mut grid) in query.iter_mut() {
        let origin = transform.translation().truncate();

        for hit in emitter.hits.iter() {
            let angle_rad = hit.angle_deg.to_radians();
            let dir = Vec2::new(angle_rad.cos(), angle_rad.sin());
            let max_distance = hit.distance;

            // Sample along the beam
            let steps = (max_distance / grid.resolution).ceil() as usize;
            for step in 0..=steps {
                let distance = step as f32 * grid.resolution;
                let point = origin + dir * distance;

                let grid_x = ((point.x - grid.origin.x) / grid.resolution).floor() as isize;
                let grid_y = ((point.y - grid.origin.y) / grid.resolution).floor() as isize;

                if grid_x < 0 || grid_y < 0 {
                    continue;
                }

                let (x, y) = (grid_x as usize, grid_y as usize);

                // Mark final point as solid
                if step == steps {
                    if hit.distance < OCCUPANCY_ASSUMED_MAX_LIDAR_RANGE_PX {
                        grid.set(x, y, CellState::Solid);
                    }
                // Else: we don't assume anything — not Solid, not Free
                } else {
                    grid.set(x, y, CellState::Free);
                }
            }
        }
    }
}

/// Draws known cells in the occupancy grid as colored boxes
pub fn draw_occupancy_grid_system(mut gizmos: Gizmos, query: Query<&OccupancyGrid>) {
    #[cfg(debug_assertions)]
    for grid in query.iter() {
        for y in 0..grid.height {
            for x in 0..grid.width {
                let state = grid.get(x, y).unwrap_or(CellState::Unknown);
                if state == CellState::Unknown {
                    continue;
                }

                const OCCUPANCY_GRID_Z: f32 = 10.0;

                let center = Vec3::new(
                    grid.origin.x + (x as f32 + 0.5) * grid.resolution,
                    grid.origin.y + (y as f32 + 0.5) * grid.resolution,
                    OCCUPANCY_GRID_Z, // Z-layer: push in front of sprites
                );

                let color = match state {
                    CellState::Free => Color::rgba(0.1, 1.0, 0.1, 0.1), // green
                    CellState::Solid => Color::rgba(1.0, 0.0, 0.0, 0.5), // red
                    CellState::Unknown => continue,
                };

                gizmos.rect(center, Quat::IDENTITY, Vec2::splat(grid.resolution), color);
            }
        }
    }
}
