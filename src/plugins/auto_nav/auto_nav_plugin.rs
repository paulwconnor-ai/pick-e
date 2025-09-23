use super::{
    follow_path_system::{clear_debug_markers_system, follow_path_system},
    plan_frontier_path_system::plan_frontier_path_system,
    toggle_autonav_system::{toggle_autonav_system, AutoNavMode},
};
use bevy::prelude::*;

// ┌────────────────────────────────────────────────────────────────────────────┐
// │                            AUTO-NAV SYSTEM OVERVIEW                        │
// └────────────────────────────────────────────────────────────────────────────┘
//
// This module implements autonomous robot navigation using occupancy grid
// mapping, frontier-based exploration, local obstacle avoidance, and path
// following. It consists of four main systems:
//
// ▶ 1. `toggle_autonav_system` (mode.rs)
//    - Toggles AutoNav mode and exploration phase (WallSweep ↔ Fill) via keyboard.
//    - Controlled by `AutoNavMode` resource, which holds the current state.
//
// ▶ 2. `plan_frontier_path_system` (path_planning.rs)
//    - Triggers when AutoNav is enabled and no current path exists.
//    - Chooses the next goal cell based on current `Phase`.
//    - Uses a breadth-first frontier search + A* to generate a safe path.
//    - Attaches a `PathPlan` component to the hero containing that path.
//
// ▶ 3. `follow_path_system` (follow_path.rs)
//    - Runs each frame to follow the current `PathPlan`, if any.
//    - Converts next cell target to a heading and velocity command (`CmdVel`).
//    - Uses local avoidance to steer around nearby walls using a virtual cone.
//    - Stops or rotates in place if unsafe to proceed.
//
// ▶ 4. `stop_when_done_system` (done_check.rs)
//    - Detects when no unexplored frontier cells remain.
//    - If so, removes the `PathPlan` component and logs completion.
//
//
// ┌──────────────┐
// │ Key Resources│
// └──────────────┘
// - `AutoNavMode`: Stores whether AutoNav is enabled and what phase is active.
//
// ┌──────────────┐
// │ Key Components│
// └──────────────┘
// - `PathPlan`: Stores a list of cell positions to follow and the target cell.
// - `CmdVel`: Receives velocity commands (`linear`, `angular`) for the drive system.
// - `OccupancyGrid`: Provides the known state of the map (Free, Solid, Unknown).
//
// These systems work together to enable autonomous frontier exploration
// that prioritizes wall-following first (WallSweep), then interior fill (Fill).

pub struct AutoNavPlugin;

impl Plugin for AutoNavPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AutoNavMode>()
            .add_systems(PreUpdate, clear_debug_markers_system)
            .add_systems(Update, toggle_autonav_system)
            .add_systems(Update, plan_frontier_path_system)
            .add_systems(Update, follow_path_system);
    }
}
