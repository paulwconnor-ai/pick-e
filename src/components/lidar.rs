use bevy::prelude::*;

/// Marker for entities with a LIDAR sensor attached
#[derive(Component)]
pub struct LidarSensor;

/// One angle-distance pair from a LIDAR scan (realistic sensor output)
#[derive(Debug, Clone)]
pub struct LidarHit {
    pub angle_deg: f32,
    pub distance: f32,

    /// For visualisation only (in debug mode)
    #[cfg(debug_assertions)]
    pub debug: DebugHitInfo,
}

/// For debug visualisation only (not included in release builds)
#[cfg(debug_assertions)]
#[derive(Debug, Clone)]
pub struct DebugHitInfo {
    pub hit_point: Vec2,
}

/// Tracks partial sweep state — emits current-frame rays only
#[derive(Component, Debug, Clone, Default)]
pub struct LidarEmitter {
    pub angle_cursor: f32,

    /// This frame’s emitted rays (cleared each frame)
    #[cfg(debug_assertions)]
    pub hits: Vec<LidarHit>,
}
