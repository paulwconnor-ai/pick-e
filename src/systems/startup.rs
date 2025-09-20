use crate::bundles::camera::camera_2d_bundle;
use crate::bundles::hero::hero_bundle;
use bevy::log::info;
use bevy::prelude::*;
use std::env;
use std::path::Path;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Running from {:?}", env::current_dir().unwrap());

    info!("📷 Spawning camera...");
    commands.spawn(camera_2d_bundle());
    commands.spawn(hero_bundle(&asset_server));
}
