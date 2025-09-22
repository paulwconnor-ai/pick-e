use crate::bundles::hero::HeroController;
use crate::components::cmd_vel::CmdVel;

use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::prelude::*;

pub fn keyboard_control_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CmdVel, With<HeroController>>,
) {
    for mut cmd_vel in &mut query {
        let mut linear_speed: f32 = 0.0;
        let mut angular_speed: f32 = 0.0;

        // WASD controls
        if keys.pressed(KeyCode::KeyW) {
            linear_speed += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            linear_speed -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            angular_speed += 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            angular_speed -= 1.0;
        }

        cmd_vel.linear = linear_speed.clamp(-1.0, 1.0);
        cmd_vel.angular = angular_speed.clamp(-1.0, 1.0);
    }
}
