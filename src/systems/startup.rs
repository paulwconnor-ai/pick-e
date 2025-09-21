use crate::bundles::camera::camera_2d_bundle;
use crate::bundles::hero::hero_bundle;

use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Well hello there!!");
    commands.spawn(camera_2d_bundle());
    commands.spawn(hero_bundle(&asset_server));
}
