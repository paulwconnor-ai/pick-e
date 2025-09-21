use bevy::asset::{AssetMode, AssetPlugin};
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use bevy_rapier2d::prelude::*;

use crate::components::collectible::CollectionStats;
use crate::systems::collectibles::{
    collect_on_collision, flood_spawn_collectibles_from_map, CollectibleFloodState,
};
use crate::systems::level::{setup_level_loading, spawn_level};
use crate::systems::robot::auto_nav::AutoNavPlugin;
use crate::systems::robot::cmd_vel_drive::cmd_vel_to_velocity_system;
use crate::systems::robot::input_keyboard::keyboard_control_system;
use crate::systems::robot::lidar_sensor::{lidar_debug_draw_system, lidar_sensor_system};
use crate::systems::robot::occupancy_grid::{
    draw_occupancy_grid_system, update_occupancy_grid_system,
};
use crate::systems::startup::setup;
use crate::ui::stats_overlay::StatsOverlayPlugin;

pub fn build_app() -> App {
    let mut app = App::new();

    // Asset and window plugins
    let asset_plugin = AssetPlugin {
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
        ..default()
    };

    let plugins = DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(asset_plugin)
        .set(window_plugin);

    app.add_plugins(plugins);

    app.add_plugins(StatsOverlayPlugin);

    // Physics
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..default()
    });
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    // Game setup systems (run once at startup)
    app.add_systems(Startup, setup);

    // In Startup:
    app.add_systems(Startup, setup_level_loading);

    // In Update:
    app.add_systems(Update, spawn_level);
    app.add_systems(Update, flood_spawn_collectibles_from_map.after(spawn_level));

    // Player input + movement
    app.add_systems(Update, keyboard_control_system);
    app.add_systems(Update, cmd_vel_to_velocity_system);

    app.add_plugins(AutoNavPlugin);

    // Sensors
    app.add_systems(
        Update,
        (
            lidar_sensor_system,
            lidar_debug_draw_system.after(lidar_sensor_system),
        ),
    );

    // Occupancy grid
    app.add_systems(
        Update,
        (
            update_occupancy_grid_system,
            draw_occupancy_grid_system.after(update_occupancy_grid_system),
        ),
    );

    // Collectibles: counter + collision detection
    app.insert_resource(CollectionStats::default());
    app.add_systems(Update, collect_on_collision);
    app.insert_resource(CollectibleFloodState::default());

    app
}
