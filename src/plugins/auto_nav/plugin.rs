use bevy::prelude::*;
use super::{
    mode::{toggle_autonav_system, AutoNavMode},
    path_planning::plan_frontier_path_system,
    follow_path::follow_path_system,
    done_check::stop_when_done_system,
};

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
