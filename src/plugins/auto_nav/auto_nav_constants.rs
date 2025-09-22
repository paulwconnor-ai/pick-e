// CmdVel outputs (dimensionless, expected range [-1.0, 1.0])
pub const CMD_VEL_MAX_LIN: f32 = 0.85;
pub const CMD_VEL_MAX_ANG: f32 = 1.0;

// Safety / wall-band parameters
pub const SAFE_MARGIN_MIN: i32 = 2;
pub const WALL_BAND_MAX: i32 = 4;
pub const DIST_SCAN_MAX: i32 = 6;

// Local avoidance sampling
pub const AVOID_SAMPLE_DEGS: [f32; 5] = [-60.0, -30.0, 0.0, 30.0, 60.0];
pub const AVOID_LOOKAHEAD_STEPS: i32 = 6;
pub const AVOID_STEP_SIZE_CELLS: f32 = 0.6;
pub const AVOID_REQUIRED_CLEARANCE: i32 = 1;
pub const AVOID_FWD_CONE_DEG: f32 = 35.0;

// A* weighting
pub const COST_NON_BAND_PENALTY: i32 = 4;
