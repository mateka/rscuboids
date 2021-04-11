use super::scoring::Score;
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
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct UISystem;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum UISystemLabels {
    UpdateFPS,
    UpdateScore,
}

#[derive(Default)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup_ui)
            .add_system_set(
                SystemSet::new()
                    .label(UISystem)
                    .with_system(update_fps_text.label(UISystemLabels::UpdateFPS))
                    .with_system(
                        update_points_text
                            .label(UISystemLabels::UpdateScore)
                            .after(UISystemLabels::UpdateFPS),
                    ),
            );
    }
}
