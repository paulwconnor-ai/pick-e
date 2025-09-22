use bevy::prelude::*;
use crate::components::cmd_vel::CmdVel;
use crate::bundles::hero::HeroController;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Phase {
    /// Prefer cells near walls within a safe band.
    #[default]
    WallSweep,
    /// Fill remaining interior.
    Fill,
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct AutoNavMode {
    pub enabled: bool,
    pub phase: Phase,
}

impl Default for AutoNavMode {
    fn default() -> Self {
        Self {
            enabled: true,
            phase: Phase::WallSweep,
        }
    }
}

pub fn toggle_autonav_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<AutoNavMode>,
    mut query: Query<&mut CmdVel, With<HeroController>>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        mode.enabled = !mode.enabled;
        info!(
            "[AutoNav] {} (phase: {:?})",
            if mode.enabled { "Enabled" } else { "Disabled" },
            mode.phase
        );

        if !mode.enabled {
            for mut cmd in query.iter_mut() {
                cmd.linear = 0.0;
                cmd.angular = 0.0;
            }
        }
    }

    // quick toggle for phase for debugging
    if keys.just_pressed(KeyCode::KeyN) {
        mode.phase = if mode.phase == Phase::WallSweep {
            Phase::Fill
        } else {
            Phase::WallSweep
        };
        info!("[AutoNav] Switched phase -> {:?}", mode.phase);
    }
}
