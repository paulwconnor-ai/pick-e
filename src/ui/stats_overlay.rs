use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::text::{BreakLineOn, JustifyText};

#[derive(Component)]
pub struct StatsOverlayText;

pub struct StatsOverlayPlugin;

impl Plugin for StatsOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup_stats_ui)
            .add_systems(Update, update_stats_text);
    }
}

fn setup_stats_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    // Panel (background) node
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
                background_color: Color::rgba(0.05, 0.05, 0.10, 0.70).into(),
                ..default()
            },
            Name::new("StatsOverlay"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Loading stats...",
                            TextStyle {
                                font,
                                font_size: 16.0,
                                color: Color::rgba(0.90, 0.90, 0.90, 0.95),
                            },
                        )],
                        justify: JustifyText::Left, // <- alignment (0.13)
                        linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    ..default()
                },
                StatsOverlayText,
            ));
        });
}

fn update_stats_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<StatsOverlayText>>,
    time: Res<Time>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    let frame_time = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    let sim_time = time.elapsed();
    let minutes = sim_time.as_secs() / 60;
    let seconds = sim_time.as_secs() % 60;

    // placeholder values for now
    let occupied = 1240;
    let explored_pct = 82.7;
    let collected = 42;
    let total = 100;

    let mut text = query.single_mut();
    text.sections[0].value = format!(
        "Exploration\n  Occupied cells seen: {occupied}\n  % of map explored: {explored_pct:.1}%\n\
         Collection\n  Items collected: {collected} / {total}\n  Completion %: {comp:.1}%\n\
         Perf/Sim\n  Frame time: {ft:.1}ms\n  FPS: {fps:.0}\n  Sim time: {mins:02}:{secs:02}",
        comp = 100.0 * collected as f32 / total as f32,
        ft = frame_time,
        mins = minutes,
        secs = seconds,
    );
}
