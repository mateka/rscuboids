use super::scoring::Score;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

struct PointsText;

fn update_points_text(score: Res<Score>, mut query: Query<&mut Text, With<PointsText>>) {
    for mut text in query.iter_mut() {
        text.value = format!("Points: {:>4}", score.score);
    }
}

struct FpsText;

fn update_fps_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

fn setup_ui(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(CameraUiBundle::default())
        // FPS text field
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(2.0),
                    right: Val::Percent(50.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "FPS: ".to_string(),
                font: asset_server.load("galaxy-monkey/galax___.ttf"),
                style: TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText)
        // Points text field
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(2.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "Points: ".to_string(),
                font: asset_server.load("galaxy-monkey/galax___.ttf"),
                style: TextStyle {
                    font_size: 36.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(PointsText);
}

#[derive(Default)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_ui.system())
            .add_system(update_fps_text.system())
            .add_system(update_points_text.system());
    }
}
