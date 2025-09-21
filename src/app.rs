use bevy::asset::{AssetMode, AssetPlugin};
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_rapier2d::prelude::*;

use crate::systems::level::{setup_level_loading, spawn_level};
use crate::systems::robot::cmd_vel_drive::cmd_vel_to_velocity_system;
use crate::systems::robot::input_keyboard::keyboard_control_system;
use crate::systems::startup::setup;

pub fn build_app() -> App {
    let mut app = App::new();

    // Core plugins: Image nearest filtering, embedded assets (for WASM), and asset loading
    let mut default_plugins = DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
            file_path: "assets".into(),
            mode: AssetMode::Processed, // required to support .meta files
            watch_for_changes_override: Some(false),
            ..default()
        });

    // WASM: Embed assets at compile time instead of fetching from network
    #[cfg(target_arch = "wasm32")]
    {
        default_plugins = default_plugins
            .build()
            .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin::default());
    }

    app.add_plugins(default_plugins);

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

    app
}
