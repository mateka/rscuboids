use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

/// This example spawns a large number of cubes, each with its own changing position and material
/// This is intended to be a stress test of bevy's ability to render many objects with different properties
/// For the best results, run it in release mode: ```cargo run --example spawner --release
/// NOTE: Bevy still has a number of optimizations to do in this area. Expect the performance here to go way up in the future
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(move_cubes.system())
        .add_system(text_update_system.system())
        .run();
}

fn move_cubes(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, &Handle<StandardMaterial>)>,
) {
    for (mut transform, material_handle) in query.iter_mut() {
        let material = materials.get_mut(material_handle).unwrap();
        transform.translation += Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds();
        material.albedo =
            Color::BLUE * Vec3::splat((3.0 * time.seconds_since_startup() as f32).sin());
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, -4.0, 5.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 150.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
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

    let mut rng = StdRng::from_entropy();
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    for _ in 0..10000 {
        commands.spawn(PbrBundle {
            mesh: cube_handle.clone(),
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                ),
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(
                rng.gen_range(-50.0..50.0),
                rng.gen_range(-50.0..50.0),
                0.0,
            )),
            ..Default::default()
        });
    }
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
