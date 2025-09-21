use bevy::asset::{AssetMode, AssetPlugin};
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use bevy_rapier2d::prelude::*;

use crate::components::collectible::CollectionStats;
use crate::systems::collectibles::collect_on_collision;
use crate::systems::collectibles::spawn_collectibles;
use crate::systems::level::{setup_level_loading, spawn_level};
use crate::systems::robot::cmd_vel_drive::cmd_vel_to_velocity_system;
use crate::systems::robot::input_keyboard::keyboard_control_system;
use crate::systems::robot::lidar_sensor::{lidar_debug_draw_system, lidar_sensor_system};
use crate::systems::robot::occupancy_grid::{
    draw_occupancy_grid_system, update_occupancy_grid_system,
};
use crate::systems::startup::setup;

pub fn build_app() -> App {
    let mut app = App::new();

    // Use Unprocessed mode for direct asset loading (native or WASM via Trunk)
    let asset_plugin: AssetPlugin = AssetPlugin {
        file_path: "assets".into(),
        mode: AssetMode::Unprocessed,
        watch_for_changes_override: Some(false),
        ..default()
    };

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy-canvas".into()),
            ..default()
        }),
        ..default() // ← no fit_canvas_to_parent!
    };

    let plugins = DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(asset_plugin)
        .set(window_plugin);

    app.add_plugins(plugins);

    // Physics setup
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..default()
    });
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    // Game systems
    app.add_systems(Startup, setup);
    app.add_systems(Startup, setup_level_loading);
    app.add_systems(Update, spawn_level);
    app.add_systems(Update, keyboard_control_system);
    app.add_systems(Update, cmd_vel_to_velocity_system);

    app.add_systems(
        Update,
        (
            lidar_sensor_system,
            lidar_debug_draw_system.after(lidar_sensor_system),
        ),
    );

    app.add_systems(
        Update,
        (
            update_occupancy_grid_system,
            draw_occupancy_grid_system.after(update_occupancy_grid_system),
        ),
    );

    app.insert_resource(CollectionStats::default());
    app.add_systems(Update, collect_on_collision);

    app.add_systems(Startup, spawn_collectibles);

    app
}
