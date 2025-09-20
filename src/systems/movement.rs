use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bundles::hero::HeroController;

const FORWARD_SPEED: f32 = 150.0;
const ROTATION_SPEED: f32 = 3.0;

pub fn keyboard_control_system(
    keys: Res<ButtonInput<KeyCode>>, // <- was Input<KeyCode>
    mut q: Query<(&mut Velocity, &Transform), With<HeroController>>,
) {
    for (mut velocity, transform) in &mut q {
        let mut thrust = 0.0;
        let mut angvel = 0.0;

        if keys.pressed(KeyCode::KeyW) {
            thrust += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            thrust -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            angvel += 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            angvel -= 1.0;
        }

        // forward vector from rotation (Z is out of screen in 2D)
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let dir = Vec2::new(angle.cos(), angle.sin());

        velocity.linvel = dir * thrust * FORWARD_SPEED;
        velocity.angvel = angvel * ROTATION_SPEED;
    }
}
