use crate::components::collectible::CollectionStats;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::text::{BreakLineOn, JustifyText};

// ---------- components ----------
#[derive(Component)]
pub struct StatsOverlayText; // bottom-left perf panel

#[derive(Component)]
pub struct TopHudText; // top-center game HUD

// ---------- plugin ----------
pub struct StatsOverlayPlugin;

impl Plugin for StatsOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, (setup_top_hud, setup_stats_ui))
            .add_systems(Update, (update_top_hud, update_stats_text));
    }
}

// ---------- TOP HUD (gamey) ----------
fn setup_top_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    // bar container across the top
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    height: Val::Px(48.0),
                    padding: UiRect::horizontal(Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.3).into(),
                ..default()
            },
            Name::new("TopHUD"),
        ))
        .with_children(|parent| {
            // single compact line
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "⭐ 0/0   🗺️ 0.0%   ⏱ 00:00",
                            TextStyle {
                                font,
                                font_size: 28.0, // big, readable
                                color: Color::rgb(0.1, 0.1, 0.4),
                            },
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                },
                TopHudText,
            ));
        });
}

fn update_top_hud(
    mut q: Query<&mut Text, With<TopHudText>>,
    time: Res<Time>,
    mut stats: ResMut<CollectionStats>,
) {
    // TODO: replace these mocks with real data:
    // - explored_pct derived from your occupancy grid stats (cells visited, or similar)

    let collected = stats.collected;
    let total = stats.total;
    let explored_pct = 0;

    let sim = time.elapsed();
    let mins = (sim.as_secs() / 60) as u64;
    let secs = (sim.as_secs() % 60) as u64;

    let mut text = q.single_mut();
    text.sections[0].value = format!(
        "Collected {}/{}   Explored {:.1}%   Time {:02}:{:02}",
        collected, total, explored_pct, mins, secs
    );
}

// ---------- PERF PANEL (bottom-left, less verbose colors) ----------
fn setup_stats_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(12.0),
                    left: Val::Px(12.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    max_width: Val::Px(420.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.3).into(),
                ..default()
            },
            Name::new("StatsOverlay"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            // perf-only
                            "Perf/Sim\n  Frame time: --.-ms   FPS: --\n  Sim time: 00:00",
                            TextStyle {
                                font,
                                font_size: 16.0,
                                color: Color::rgb(0.1, 0.1, 0.4),
                            },
                        )],
                        justify: bevy::text::JustifyText::Left,
                        linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
                    },
                    ..default()
                },
                StatsOverlayText,
            ));
        });
}

fn update_stats_text(
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    mut query: Query<&mut Text, With<StatsOverlayText>>,
    time: Res<Time>,
) {
    let fps = diagnostics
        .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    let frame_time = diagnostics
        .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    let sim_time = time.elapsed();
    let minutes = sim_time.as_secs() / 60;
    let seconds = sim_time.as_secs() % 60;

    let mut text = query.single_mut();
    text.sections[0].value = format!(
        "Perf/Sim\n  Frame time: {:.1}ms   FPS: {:.0}\n  Sim time: {:02}:{:02}",
        frame_time, fps, minutes, seconds,
    );
}
