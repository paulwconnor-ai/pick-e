// ==========================
// Vacuum Bot World Config
// ==========================

/// Logical resolution of the world (in pixels).
/// This defines the render or simulation canvas size.
pub const LOGICAL_W: f32 = 1920.0;
pub const LOGICAL_H: f32 = 1080.0;

/// Approximate radius of the vacuum bot in pixels and real-world meters.
/// Many consumer bots are ~30–35 cm in diameter.
pub const HERO_RADIUS_PX: f32 = 32.0;
pub const HERO_RADIUS_METERS: f32 = 0.175; // ~35cm diameter

/// Global unit scale — how many meters per pixel.
/// Used to convert physical distances into world-space coordinates.
pub const METERS_PER_PIXEL: f32 = HERO_RADIUS_METERS / HERO_RADIUS_PX;

// ===================
// 🔦 LIDAR Parameters
// ===================

/// Simulated spin rate of the vacuum bot's LIDAR (in Hz).
/// Budget bots usually spin at 5–7 Hz.
/// 5 Hz = 1 full sweep every 200 ms.
pub const LIDAR_SPIN_RATE_HZ: f32 = 5.0;

/// Angular resolution of the LIDAR in degrees.
/// Many low-cost sensors emit beams every 1–5 degrees.
/// Higher values = lower resolution = less CPU usage.
pub const LIDAR_ANGLE_STEP: f32 = 2.0;

/// Maximum sensor range in real-world meters.
/// Budget vacuums typically range up to 4–6 meters at best.
pub const LIDAR_MAX_RANGE_METERS: f32 = 5.0;

/// Max LIDAR range in world pixels, based on METERS_PER_PIXEL.
pub const LIDAR_MAX_RANGE_PX: f32 = LIDAR_MAX_RANGE_METERS / METERS_PER_PIXEL;
