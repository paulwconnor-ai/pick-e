use crate::bundles::hero::HeroController;
use crate::components::cmd_vel::CmdVel;
use crate::components::occupancy_grid::{CellState, OccupancyGrid};
use bevy::prelude::*;

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Phase {
    /// Prefer cells near walls within a safe band.
    #[default]
    WallSweep,
    /// Fill remaining interior.
    Fill,
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct AutoNavMode {
    pub enabled: bool,
    pub phase: Phase,
}

impl Default for AutoNavMode {
    fn default() -> Self {
        Self {
            enabled: true,
            phase: Phase::WallSweep,
        }
    }
}

#[derive(Component)]
pub struct PathPlan {
    pub cells: Vec<IVec2>,
    pub target: IVec2,
}

// CmdVel outputs (dimensionless, expected range [-1.0, 1.0])
// - linear maps to your downstream drive system's max forward speed
// - angular maps to your downstream system's max turn rate
const CMD_VEL_MAX_LIN: f32 = 0.85; // [-] fraction of max forward speed
const CMD_VEL_MAX_ANG: f32 = 1.0; // [-] fraction of max angular speed

// Distance threshold for "arrived at next cell center"
const CELL_RADIUS: f32 = 8.0; // [world units, e.g. meters in your Bevy world] (not cells)

// Safety / wall-band parameters (cell-based; convert to meters via grid.resolution)
const SAFE_MARGIN_MIN: i32 = 2; // [cells] minimum distance from walls/edges considered "safe"
const WALL_BAND_MAX: i32 = 4; // [cells] max distance from wall to still count as "near wall" (band upper bound)
const DIST_SCAN_MAX: i32 = 6; // [cells] search radius when estimating distance to nearest wall/edge

// Local avoidance sampling
const AVOID_SAMPLE_DEGS: [f32; 5] = [-60.0, -30.0, 0.0, 30.0, 60.0]; // [deg] headings tested around desired direction
const AVOID_LOOKAHEAD_STEPS: i32 = 6; // [count] how many sample points to check along a heading
const AVOID_STEP_SIZE_CELLS: f32 = 0.6; // [cells] spacing between those sample points (cells → meters via resolution)
const AVOID_REQUIRED_CLEARANCE: i32 = 2; // [cells] minimum clearance required in forward-cone check
const AVOID_FWD_CONE_DEG: f32 = 35.0; // [deg] half-angle of the "virtual bumper" cone ahead

// A* weighting (dimensionless costs)
const COST_NON_BAND_PENALTY: i32 = 4; // [-] extra step cost during WallSweep if step leaves the wall band

pub struct AutoNavPlugin;

impl Plugin for AutoNavPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AutoNavMode>().add_systems(
            Update,
            (
                toggle_autonav_system,
                plan_frontier_path_system,
                follow_path_system,
                stop_when_done_system,
            )
                .chain(),
        );
    }
}

fn toggle_autonav_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<AutoNavMode>,
    mut query: Query<&mut CmdVel, With<HeroController>>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        mode.enabled = !mode.enabled;
        info!(
            "[AutoNav] {} (phase: {:?})",
            if mode.enabled { "Enabled" } else { "Disabled" },
            mode.phase
        );

        if !mode.enabled {
            for mut cmd in query.iter_mut() {
                cmd.linear = 0.0;
                cmd.angular = 0.0;
            }
        }
    }

    // quick toggle for phase for debugging
    if keys.just_pressed(KeyCode::KeyN) {
        mode.phase = if mode.phase == Phase::WallSweep {
            Phase::Fill
        } else {
            Phase::WallSweep
        };
        info!("[AutoNav] Switched phase -> {:?}", mode.phase);
    }
}

/* ------------------------------ Planning ------------------------------ */

fn plan_frontier_path_system(
    mut mode: ResMut<AutoNavMode>,
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform, &OccupancyGrid), With<HeroController>>,
    has_path: Query<&PathPlan>,
) {
    if !mode.enabled || has_path.iter().next().is_some() {
        return;
    }

    for (entity, xform, grid) in query.iter() {
        let pos = xform.translation().truncate();
        let Some(start_cell) = grid.world_to_cell(pos) else {
            continue;
        };

        // pick a target depending on phase
        let target = match mode.phase {
            Phase::WallSweep => {
                find_nearest_frontier_where(grid, start_cell, |c| {
                    is_safe_cell(grid, c, SAFE_MARGIN_MIN)
                        && is_wall_band_cell(grid, c, SAFE_MARGIN_MIN, WALL_BAND_MAX)
                })
                .or_else(|| {
                    // if no band frontiers exist, switch phase and try Fill
                    mode.phase = Phase::Fill;
                    info!("[AutoNav] No wall-band frontiers; switching to Fill.");
                    find_nearest_frontier_where(grid, start_cell, |c| {
                        is_safe_cell(grid, c, SAFE_MARGIN_MIN)
                    })
                })
            }
            Phase::Fill => find_nearest_frontier_where(grid, start_cell, |c| {
                is_safe_cell(grid, c, SAFE_MARGIN_MIN)
            }),
        };

        if let Some(goal) = target {
            if let Some(path) = astar_with_policy(
                grid,
                start_cell,
                goal,
                PathPolicy {
                    avoid_unsafe: true,
                    prefer_band: mode.phase == Phase::WallSweep,
                    safe_min: SAFE_MARGIN_MIN,
                    band_max: WALL_BAND_MAX,
                },
            ) {
                commands.entity(entity).insert(PathPlan {
                    cells: path.clone(),
                    target: goal,
                });
                info!("[AutoNav:{:?}] Planned path to {:?}", mode.phase, goal);
            }
        }
    }
}

/* --------------------------- Path following --------------------------- */

fn follow_path_system(
    mode: Res<AutoNavMode>,
    mut query: Query<
        (&mut CmdVel, &mut PathPlan, &GlobalTransform, &OccupancyGrid),
        With<HeroController>,
    >,
) {
    if !mode.enabled {
        return;
    }

    for (mut cmd, mut path, xform, grid) in query.iter_mut() {
        let pos = xform.translation().truncate();

        let Some(next_cell) = path.cells.first() else {
            // no path? stop just in case
            cmd.linear = 0.0;
            cmd.angular = 0.0;
            continue;
        };
        let target_pos = grid.cell_to_world(*next_cell);

        // Desired direction to next cell center
        let to_target = target_pos - pos;
        let dist = to_target.length();

        // Arrive at this cell?
        if dist < CELL_RADIUS {
            path.cells.remove(0);
            if path.cells.is_empty() {
                cmd.linear = 0.0;
                cmd.angular = 0.0;
            }
            continue;
        }

        // Robot forward (Y-up sprite assumed)
        let forward = xform.up().truncate().normalize_or_zero();
        let desired = to_target.normalize_or_zero();

        // --- LOCAL AVOIDANCE ---
        // If forward cone has unsafe clearance, forbid forward motion this tick.
        let forward_clear_ok = heading_clear_ok(
            grid,
            pos,
            forward,
            AVOID_FWD_CONE_DEG,
            AVOID_REQUIRED_CLEARANCE,
        );

        // Try a small set of headings around 'desired' and pick the one with best clearance.
        let best_dir = pick_best_heading(grid, pos, desired);

        // Steering to chosen heading
        let angle = forward.angle_between(best_dir).clamp(0.0, CMD_VEL_MAX_ANG); // [0..pi]
        let cross = forward.perp_dot(best_dir); // sign

        // Rotate-in-place if we need big correction OR forward bumper says no-go.
        let rotate_only = angle > 0.70 || !forward_clear_ok;

        let ang_cmd = cross.signum() * angle;
        let lin_cmd = if rotate_only {
            0.0
        } else if angle > 0.35 {
            0.4 * CMD_VEL_MAX_LIN
        } else {
            // Scale by clearance in chosen direction (soften near walls)
            let clear = heading_clearance_cells(grid, pos, best_dir);
            let clear_scale = ((clear as f32) / 4.0).clamp(0.25, 1.0);
            CMD_VEL_MAX_LIN * clear_scale
        };

        cmd.linear = lin_cmd;
        cmd.angular = ang_cmd;
    }
}

/* ---------------------------- Done detection ---------------------------- */

fn stop_when_done_system(
    mut commands: Commands,
    mode: Res<AutoNavMode>,
    query: Query<(Entity, &OccupancyGrid, Option<&PathPlan>), With<HeroController>>,
) {
    if !mode.enabled {
        return;
    }

    for (entity, grid, path) in query.iter() {
        let mut has_frontier = false;
        for (cell, state) in grid.iter() {
            if state == CellState::Free
                && has_unknown_neighbor(grid, cell)
                && is_safe_cell(grid, cell, SAFE_MARGIN_MIN)
            {
                has_frontier = true;
                break;
            }
        }

        if !has_frontier {
            if path.is_some() {
                commands.entity(entity).remove::<PathPlan>();
            }
            info!("[AutoNav] All frontiers explored.");
        }
    }
}

/* --------------------------- Frontier helpers --------------------------- */

fn find_nearest_frontier_where(
    grid: &OccupancyGrid,
    start: IVec2,
    predicate: impl Fn(IVec2) -> bool,
) -> Option<IVec2> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if grid.get_cell(current) == Some(CellState::Free)
            && has_unknown_neighbor(grid, current)
            && predicate(current)
        {
            return Some(current);
        }

        for n in neighbors4(current) {
            if !visited.contains(&n) && grid.get_cell(n) == Some(CellState::Free) {
                visited.insert(n);
                queue.push_back(n);
            }
        }
    }

    None
}

fn has_unknown_neighbor(grid: &OccupancyGrid, cell: IVec2) -> bool {
    neighbors4(cell)
        .iter()
        .any(|&n| grid.get_cell(n) == Some(CellState::Unknown))
}

fn neighbors4(cell: IVec2) -> [IVec2; 4] {
    [
        cell + IVec2::X,
        cell - IVec2::X,
        cell + IVec2::Y,
        cell - IVec2::Y,
    ]
}

/* ----------------------- Safety / band classification ----------------------- */

fn is_safe_cell(grid: &OccupancyGrid, cell: IVec2, safe_min: i32) -> bool {
    let d = distance_to_solid_or_edge(grid, cell, DIST_SCAN_MAX);
    d >= safe_min
}

fn is_wall_band_cell(grid: &OccupancyGrid, cell: IVec2, safe_min: i32, band_max: i32) -> bool {
    let d = distance_to_solid_or_edge(grid, cell, DIST_SCAN_MAX);
    d >= safe_min && d <= band_max
}

/// Manhattan-like local distance to nearest SOLID or map EDGE (edges treated as solid).
/// Returns a value in [0..=scan_max], where 0 means touching.
fn distance_to_solid_or_edge(grid: &OccupancyGrid, cell: IVec2, scan_max: i32) -> i32 {
    // treat out-of-bounds as SOLID to keep margin from map edges
    if !in_bounds(grid, cell) {
        return 0;
    }
    if grid.get_cell(cell) == Some(CellState::Solid) {
        return 0;
    }

    let mut best = scan_max + 1;
    for r in 1..=scan_max {
        // diamond (Manhattan) ring scan for speed; square works too if you prefer
        for dy in -r..=r {
            let dx = r - dy.abs();
            for sx in [-1, 1] {
                let c1 = cell + IVec2::new(sx * dx, dy);
                if !in_bounds(grid, c1) || grid.get_cell(c1) == Some(CellState::Solid) {
                    return r;
                }
            }
        }
        best = best.min(r);
    }
    best
}

fn in_bounds(grid: &OccupancyGrid, c: IVec2) -> bool {
    c.x >= 0 && c.y >= 0 && (c.x as usize) < grid.width && (c.y as usize) < grid.height
}

/* --------------------------------- A* --------------------------------- */

#[derive(Clone, Copy)]
struct PathPolicy {
    avoid_unsafe: bool,
    prefer_band: bool,
    safe_min: i32,
    band_max: i32,
}

fn astar_with_policy(
    grid: &OccupancyGrid,
    start: IVec2,
    goal: IVec2,
    policy: PathPolicy,
) -> Option<Vec<IVec2>> {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Node {
        pos: IVec2,
        g: i32,
        f: i32,
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            // min-heap by reversing
            other.f.cmp(&self.f)
        }
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open = BinaryHeap::new();
    let mut came: HashMap<IVec2, IVec2> = HashMap::new();
    let mut g_score: HashMap<IVec2, i32> = HashMap::new();

    open.push(Node {
        pos: start,
        g: 0,
        f: heuristic(start, goal),
    });
    g_score.insert(start, 0);

    while let Some(Node { pos, g, .. }) = open.pop() {
        if pos == goal {
            // reconstruct path
            let mut path = vec![pos];
            let mut cur = pos;
            while let Some(&prev) = came.get(&cur) {
                path.push(prev);
                cur = prev;
            }
            path.reverse();
            return Some(path);
        }

        for nb in neighbors4(pos) {
            if grid.get_cell(nb) != Some(CellState::Free) {
                continue;
            }

            // Safety: keep away from walls/edges
            let dist = distance_to_solid_or_edge(grid, nb, DIST_SCAN_MAX);
            if policy.avoid_unsafe && dist < policy.safe_min {
                continue;
            }

            // step cost
            let mut step = 1;

            // Prefer staying in the wall band during WallSweep
            if policy.prefer_band {
                if !(dist >= policy.safe_min && dist <= policy.band_max) {
                    step += COST_NON_BAND_PENALTY;
                }
            }

            let tentative = g + step;
            if tentative < *g_score.get(&nb).unwrap_or(&i32::MAX) {
                came.insert(nb, pos);
                g_score.insert(nb, tentative);

                let f = tentative + heuristic(nb, goal);
                open.push(Node {
                    pos: nb,
                    g: tentative,
                    f,
                });
            }
        }
    }

    None
}

#[inline]
fn heuristic(a: IVec2, b: IVec2) -> i32 {
    // Manhattan works with 4-neighborhood
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[inline]
fn rot_deg(v: Vec2, deg: f32) -> Vec2 {
    let rad = deg.to_radians();
    let (s, c) = rad.sin_cos();
    Vec2::new(c * v.x - s * v.y, s * v.x + c * v.y).normalize_or_zero()
}

/// Is the space within a forward cone safe enough? (virtual bumper)
fn heading_clear_ok(
    grid: &OccupancyGrid,
    pos: Vec2,
    forward: Vec2,
    cone_deg: f32,
    min_clear: i32,
) -> bool {
    // check a few rays within the cone
    for off in [-cone_deg, -cone_deg * 0.5, 0.0, cone_deg * 0.5, cone_deg] {
        let dir = rot_deg(forward, off);
        let c = heading_clearance_cells(grid, pos, dir);
        if c < min_clear {
            return false;
        }
    }
    true
}

/// Evaluate how many "cells" of clearance along a heading (min distance-to-wall across samples).
fn heading_clearance_cells(grid: &OccupancyGrid, pos: Vec2, dir: Vec2) -> i32 {
    let mut min_clear = i32::MAX;

    // convert step from cells to meters using grid.resolution
    let step_world = AVOID_STEP_SIZE_CELLS * grid.resolution;

    for i in 1..=AVOID_LOOKAHEAD_STEPS {
        let p = pos + dir * (i as f32) * step_world;
        if let Some(c) = grid.world_to_cell(p) {
            let d = distance_to_solid_or_edge(grid, c, DIST_SCAN_MAX);
            min_clear = min_clear.min(d);
        } else {
            // out of bounds counts as zero clearance
            return 0;
        }
    }

    if min_clear == i32::MAX {
        0
    } else {
        min_clear
    }
}

/// Choose the best heading among a small set around 'desired', maximizing clearance,
/// but keep it near the desired direction to reduce zig-zag.
fn pick_best_heading(grid: &OccupancyGrid, pos: Vec2, desired: Vec2) -> Vec2 {
    let mut best_dir = desired;
    let mut best_score = -1_000_000f32;

    for off in AVOID_SAMPLE_DEGS {
        let dir = rot_deg(desired, off);
        // clearance score in "cells"
        let clear = heading_clearance_cells(grid, pos, dir) as f32;

        // penalize large deviation from desired to keep progress
        let align = desired.dot(dir).clamp(0.0, 1.0); // [0..1]
        let score = clear * 3.0 + align * 1.0;

        if score > best_score {
            best_score = score;
            best_dir = dir;
        }
    }

    best_dir
}
