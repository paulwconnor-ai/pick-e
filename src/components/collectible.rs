use bevy::prelude::*;

#[derive(Component)]
pub struct Collectible;

#[derive(Resource, Default)]
pub struct CollectionStats {
    pub collected: usize,
    pub total: usize,
}
