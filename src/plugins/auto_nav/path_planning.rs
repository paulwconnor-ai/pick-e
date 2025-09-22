use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::bundles::hero::HeroController;
use crate::components::occupancy_grid::{CellState, OccupancyGrid};
use crate::plugins::auto_nav::constants::*;
use crate::plugins::auto_nav::mode::{AutoNavMode, Phase};

#[derive(Component)]
pub struct PathPlan {
    pub cells: Vec<IVec2>,
    pub target: IVec2,
}

pub fn plan_frontier_path_system(
    mut mode: ResMut<AutoNavMode>,
    mut commands: Commands,
    query: Query<
        (Entity, &GlobalTransform, &OccupancyGrid, Option<&PathPlan>),
        With<HeroController>,
    >,
) {
    if !mode.enabled {
        return;
    }

    for (entity, xform, grid, maybe_path) in query.iter() {
        if maybe_path.is_some() {
            continue; // already has a plan
        }

        let pos = xform.translation().truncate();
        let Some(start_cell) = grid.world_to_cell(pos) else {
            continue;
        };

        let target = match mode.phase {
            Phase::WallSweep => find_nearest_frontier_where(grid, start_cell, |c| {
                is_safe_cell(grid, c, SAFE_MARGIN_MIN)
                    && is_wall_band_cell(grid, c, SAFE_MARGIN_MIN, WALL_BAND_MAX)
            })
            .or_else(|| {
                mode.phase = Phase::Fill;
                info!("[AutoNav] No wall-band frontiers; switching to Fill.");
                find_nearest_frontier_where(grid, start_cell, |c| {
                    is_safe_cell(grid, c, SAFE_MARGIN_MIN)
                })
            }),
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
                    cells: path,
                    target: goal,
                });
                info!("[AutoNav:{:?}] Planned path to {:?}", mode.phase, goal);
            }
        }
    }
}

/* ---------------- Helpers ---------------- */

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

pub fn has_unknown_neighbor(grid: &OccupancyGrid, cell: IVec2) -> bool {
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

pub fn is_safe_cell(grid: &OccupancyGrid, cell: IVec2, safe_min: i32) -> bool {
    let d = distance_to_solid_or_edge(grid, cell, DIST_SCAN_MAX);
    d >= safe_min
}

fn is_wall_band_cell(grid: &OccupancyGrid, cell: IVec2, safe_min: i32, band_max: i32) -> bool {
    let d = distance_to_solid_or_edge(grid, cell, DIST_SCAN_MAX);
    d >= safe_min && d <= band_max
}

pub fn distance_to_solid_or_edge(grid: &OccupancyGrid, cell: IVec2, scan_max: i32) -> i32 {
    if !in_bounds(grid, cell) {
        return 0;
    }
    if grid.get_cell(cell) == Some(CellState::Solid) {
        return 0;
    }

    for r in 1..=scan_max {
        for dy in -r..=r {
            let dx = r - dy.abs();
            for sx in [-1, 1] {
                let c1 = cell + IVec2::new(sx * dx, dy);
                if !in_bounds(grid, c1) || grid.get_cell(c1) == Some(CellState::Solid) {
                    return r;
                }
            }
        }
    }

    // No solid or edge within scan range; treat as very safe
    scan_max + 1
}

fn in_bounds(grid: &OccupancyGrid, c: IVec2) -> bool {
    c.x >= 0 && c.y >= 0 && (c.x as usize) < grid.width && (c.y as usize) < grid.height
}

/* ---------------- A* ---------------- */

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
            other.f.cmp(&self.f)
        }
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open = BinaryHeap::new();
    let mut came = HashMap::new();
    let mut g_score = HashMap::new();

    open.push(Node {
        pos: start,
        g: 0,
        f: heuristic(start, goal),
    });
    g_score.insert(start, 0);

    while let Some(Node { pos, g, .. }) = open.pop() {
        if pos == goal {
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

            let dist = distance_to_solid_or_edge(grid, nb, DIST_SCAN_MAX);
            if policy.avoid_unsafe && dist < policy.safe_min {
                continue;
            }

            let mut step = 1;
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

fn heuristic(a: IVec2, b: IVec2) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
