use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::systems::startup::setup;

pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        file_path: "assets".into(),
        ..Default::default()
    }));

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_systems(Startup, setup);

    app
}
