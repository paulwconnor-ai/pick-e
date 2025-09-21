use crate::components::lidar::{LidarEmitter, LidarHit, LidarSensor};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::{LIDAR_ANGLE_STEP, LIDAR_MAX_RANGE_PX, LIDAR_SPIN_RATE_HZ};

#[cfg(debug_assertions)]
use crate::components::lidar::DebugHitInfo;

/// Emits a realistic LIDAR scan arc each frame (no accumulation).
pub fn lidar_sensor_system(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut query: Query<(&GlobalTransform, &mut LidarEmitter, Entity), With<LidarSensor>>,
) {
    for (transform, mut emitter, entity) in query.iter_mut() {
        let origin = transform.translation().truncate();
        let angle_delta = 360.0 * LIDAR_SPIN_RATE_HZ * time.delta_seconds();

        let start_angle = emitter.angle_cursor;
        let end_angle = start_angle + angle_delta;

        #[cfg(debug_assertions)]
        emitter.hits.clear();

        let mut angle = start_angle;
        while angle < end_angle {
            let angle_rad = angle.to_radians();
            let dir = Vec2::new(angle_rad.cos(), angle_rad.sin());

            use bevy_rapier2d::geometry::{CollisionGroups, Group};

            let ray_result = rapier_context.cast_ray(
                origin,
                dir,
                LIDAR_MAX_RANGE_PX,
                true,
                QueryFilter::default()
                    .exclude_collider(entity)
                    .groups(CollisionGroups::new(
                        Group::ALL,
                        Group::ALL ^ Group::GROUP_2,
                    )),
            );

            let distance = match ray_result {
                Some((_e, toi)) => toi,
                None => LIDAR_MAX_RANGE_PX,
            };

            #[cfg(debug_assertions)]
            emitter.hits.push(LidarHit {
                angle_deg: angle % 360.0,
                distance,
                debug: DebugHitInfo {
                    hit_point: origin + dir * distance,
                },
            });

            angle += LIDAR_ANGLE_STEP;
        }

        emitter.angle_cursor = end_angle % 360.0;
    }
}

/// Debug-draws the rays emitted this frame from each LIDAR
pub fn lidar_debug_draw_system(
    query: Query<(&GlobalTransform, &LidarEmitter), With<LidarSensor>>,
    mut gizmos: Gizmos,
) {
    #[cfg(debug_assertions)]
    for (transform, emitter) in query.iter() {
        let origin = transform.translation().truncate();

        for hit in &emitter.hits {
            gizmos.line_2d(origin, hit.debug.hit_point, Color::rgba(1.0, 0.2, 0.0, 0.5));
        }
    }
}
