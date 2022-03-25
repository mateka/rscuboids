use super::scoring::Score;
use super::ship::Ship;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Debug, Component)]
struct PointsText;

fn update_points_text(score: Res<Score>, mut query: Query<&mut Text, With<PointsText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Points: {:>4}", score.score).to_string();
    }
}

#[derive(Debug, Component)]
struct FpsText;

fn update_fps_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = format!("FPS: {:.2}", average).to_string();
            }
        }
    }
}

#[derive(Debug, Component)]
struct LivesText;

fn update_lives_text(ship_query: Query<&Ship>, mut text_query: Query<&mut Text, With<LivesText>>) {
    let ship = ship_query.single();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Lives: {}", ship.lives).to_string();
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        // FPS text field
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(2.0),
                    right: Val::Percent(50.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "FPS: ".to_string(),
                TextStyle {
                    font: asset_server.load("galaxy-monkey/galax___.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(FpsText);
    commands
        // Points text field
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(2.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Points: ".to_string(),
                TextStyle {
                    font: asset_server.load("galaxy-monkey/galax___.ttf"),
                    font_size: 36.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(PointsText);
    commands
        // Lives text field
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(2.0),
                    right: Val::Percent(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Lives: ".to_string(),
                TextStyle {
                    font: asset_server.load("galaxy-monkey/galax___.ttf"),
                    font_size: 36.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(LivesText);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct UiSystem;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum UiSystemLabels {
    Score,
    Lives,
    Fps,
}

#[derive(Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup_ui)
            .add_system_set(
                SystemSet::new()
                    .label(UiSystem)
                    .with_system(update_fps_text.label(UiSystemLabels::Fps))
                    .with_system(
                        update_points_text
                            .label(UiSystemLabels::Score)
                            .after(UiSystemLabels::Fps),
                    )
                    .with_system(
                        update_lives_text
                            .label(UiSystemLabels::Lives)
                            .after(UiSystemLabels::Score),
                    ),
            );
    }
}
