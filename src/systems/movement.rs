use bevy::input::gamepad::{GamepadAxis, GamepadAxisType, Gamepads};
use bevy::input::keyboard::KeyCode;
use bevy::input::Axis;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bundles::hero::HeroController;

const FORWARD_SPEED: f32 = 150.0;
const ROTATION_SPEED: f32 = 3.0;
const DEADZONE_X: f32 = 0.25;
const DEADZONE_Y: f32 = 0.25;

pub fn keyboard_gamepad_control_system(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut q: Query<(&mut Velocity, &Transform), With<HeroController>>,
) {
    for (mut velocity, transform) in &mut q {
        let mut thrust = 0.0;
        let mut ang = 0.0;

        // Keyboard
        if keys.pressed(KeyCode::KeyW) {
            thrust += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            thrust -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            ang += 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            ang -= 1.0;
        }

        // Gamepad (first connected)
        if let Some(gamepad) = gamepads.iter().next() {
            let lx = axes
                .get(GamepadAxis {
                    gamepad,
                    axis_type: GamepadAxisType::LeftStickX,
                })
                .unwrap_or(0.0);
            let ly = axes
                .get(GamepadAxis {
                    gamepad,
                    axis_type: GamepadAxisType::LeftStickY,
                })
                .unwrap_or(0.0);

            let lx = if lx.abs() < DEADZONE_X { 0.0 } else { lx };
            let ly = if ly.abs() < DEADZONE_Y { 0.0 } else { ly };

            thrust += (ly).clamp(-1.0, 1.0); // up on stick = forward
            ang += (-lx).clamp(-1.0, 1.0); // left = +CCW
        }

        thrust = thrust.clamp(-1.0, 1.0);
        ang = ang.clamp(-1.0, 1.0);

        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let dir = Vec2::new(angle.cos(), angle.sin());

        velocity.linvel = dir * thrust * FORWARD_SPEED;
        velocity.angvel = ang * ROTATION_SPEED;
    }
}
