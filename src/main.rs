use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier2d::physics::RapierPhysicsPlugin;
use bevy_rapier2d::render::RapierRenderPlugin;

use rscuboids::GamePlugins;

#[bevy_main]
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "rsCuboids".to_string(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(GamePlugins)
        .add_startup_system(setup.system())
        .add_system(text_update_system.system())
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS: ".to_string(),
                font: asset_server.load("galaxy-monkey/galax___.ttf"),
                style: TextStyle {
                    font_size: 32.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}
