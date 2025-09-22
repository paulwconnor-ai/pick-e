use crate::bundles::hero::HeroController;
use crate::components::cmd_vel::CmdVel;
use crate::components::occupancy_grid::{CellState, OccupancyGrid};
use crate::plugins::auto_nav::constants::*;
use crate::plugins::auto_nav::mode::AutoNavMode;
use crate::plugins::auto_nav::path_planning::{has_unknown_neighbor, is_safe_cell, PathPlan};
use bevy::prelude::*;

pub fn stop_when_done_system(
    mut commands: Commands,
    mode: Res<AutoNavMode>,
    mut query: Query<
        (Entity, &mut CmdVel, &OccupancyGrid, Option<&PathPlan>),
        With<HeroController>,
    >,
) {
    if !mode.enabled {
        return;
    }

    for (entity, mut cmd, grid, path) in query.iter_mut() {
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
            // Stop motion and clear plan
            cmd.linear = 0.0;
            cmd.angular = 0.0;
            if path.is_some() {
                commands.entity(entity).remove::<PathPlan>();
            }
            info!("[AutoNav] All frontiers explored.");
        }
    }
}
