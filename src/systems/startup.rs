use crate::bundles::camera::camera_2d_bundle;
use crate::bundles::hero::hero_bundle;
use bevy::log::info;
use bevy::prelude::*;
use std::env;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Running Pick.e from {:?}", env::current_dir().unwrap());

    commands.spawn(camera_2d_bundle());
    commands.spawn(hero_bundle(&asset_server));
}
