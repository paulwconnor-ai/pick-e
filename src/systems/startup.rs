use crate::bundles::camera::camera_2d_bundle;
use crate::bundles::hero::hero_bundle;
use crate::systems::level::spawn_level;

use bevy::log::info;
use bevy::prelude::*;
use std::env;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(camera_2d_bundle());
    commands.spawn(hero_bundle(&asset_server));

    spawn_level(&mut commands, &asset_server);
}
