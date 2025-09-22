use crate::bundles::hero::HeroController;
use crate::components::cmd_vel::CmdVel;
use crate::components::occupancy_grid::OccupancyGrid;
use crate::plugins::auto_nav::constants::*;
use crate::plugins::auto_nav::mode::AutoNavMode;
use crate::plugins::auto_nav::path_planning::PathPlan;
use bevy::prelude::*;

pub fn follow_path_system(
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
            cmd.linear = 0.0;
            cmd.angular = 0.0;
            continue;
        };
        let target_pos = grid.cell_to_world(*next_cell);
        let to_target = target_pos - pos;
        let dist = to_target.length();

        if dist < CELL_RADIUS {
            path.cells.remove(0);
            if path.cells.is_empty() {
                cmd.linear = 0.0;
                cmd.angular = 0.0;
            }
            continue;
        }

        let forward = xform.up().truncate().normalize_or_zero();
        let desired = to_target.normalize_or_zero();
        let forward_clear_ok = heading_clear_ok(
            grid,
            pos,
            forward,
            AVOID_FWD_CONE_DEG,
            AVOID_REQUIRED_CLEARANCE,
        );

        let best_dir = pick_best_heading(grid, pos, desired);
        let angle = forward.angle_between(best_dir).clamp(0.0, CMD_VEL_MAX_ANG);
        let cross = forward.perp_dot(best_dir);
        let rotate_only = angle > 0.70 || !forward_clear_ok;

        let ang_cmd = cross.signum() * angle;
        let lin_cmd = if rotate_only {
            0.0
        } else if angle > 0.35 {
            0.4 * CMD_VEL_MAX_LIN
        } else {
            let clear = heading_clearance_cells(grid, pos, best_dir);
            let clear_scale = ((clear as f32) / 4.0).clamp(0.25, 1.0);
            CMD_VEL_MAX_LIN * clear_scale
        };

        cmd.linear = lin_cmd;
        cmd.angular = ang_cmd;
    }
}

fn heading_clear_ok(
    grid: &OccupancyGrid,
    pos: Vec2,
    forward: Vec2,
    cone_deg: f32,
    min_clear: i32,
) -> bool {
    for off in [-cone_deg, -cone_deg * 0.5, 0.0, cone_deg * 0.5, cone_deg] {
        let dir = rot_deg(forward, off);
        let c = heading_clearance_cells(grid, pos, dir);
        if c < min_clear {
            return false;
        }
    }
    true
}

fn heading_clearance_cells(grid: &OccupancyGrid, pos: Vec2, dir: Vec2) -> i32 {
    let mut min_clear = i32::MAX;
    let step_world = AVOID_STEP_SIZE_CELLS * grid.resolution;

    for i in 1..=AVOID_LOOKAHEAD_STEPS {
        let p = pos + dir * (i as f32) * step_world;
        if let Some(c) = grid.world_to_cell(p) {
            let d = crate::plugins::auto_nav::path_planning::distance_to_solid_or_edge(
                grid,
                c,
                DIST_SCAN_MAX,
            );
            min_clear = min_clear.min(d);
        } else {
            return 0;
        }
    }

    if min_clear == i32::MAX {
        0
    } else {
        min_clear
    }
}

fn pick_best_heading(grid: &OccupancyGrid, pos: Vec2, desired: Vec2) -> Vec2 {
    let mut best_dir = desired;
    let mut best_score = -1_000_000f32;

    for off in AVOID_SAMPLE_DEGS {
        let dir = rot_deg(desired, off);
        let clear = heading_clearance_cells(grid, pos, dir) as f32;
        let align = desired.dot(dir).clamp(0.0, 1.0);
        let score = clear * 3.0 + align * 1.0;

        if score > best_score {
            best_score = score;
            best_dir = dir;
        }
    }

    best_dir
}

fn rot_deg(v: Vec2, deg: f32) -> Vec2 {
    let rad = deg.to_radians();
    let (s, c) = rad.sin_cos();
    Vec2::new(c * v.x - s * v.y, s * v.x + c * v.y).normalize_or_zero()
}
