use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::cmd_vel::CmdVel;

const FORWARD_SPEED: f32 = 150.0;
const ROTATION_SPEED: f32 = 3.0;

pub fn cmd_vel_to_velocity_system(mut q: Query<(&CmdVel, &mut Velocity, &Transform)>) {
    for (cmd, mut velocity, transform) in &mut q {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let forward = Vec2::new(angle.cos(), angle.sin());

        velocity.linvel = forward * cmd.linear * FORWARD_SPEED;
        velocity.angvel = cmd.angular * ROTATION_SPEED;
    }
}
