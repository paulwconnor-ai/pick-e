use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct CmdVel {
    pub linear: f32,
    pub angular: f32,
}
