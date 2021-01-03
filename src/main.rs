use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier2d::physics::RapierPhysicsPlugin;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use rand::{rngs::StdRng, Rng, SeedableRng};

use bevy_rapier2d::render::RapierRenderPlugin;

#[bevy_main]
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "rsCuboids".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_system(setup_world.system())
        .add_startup_system(setup_cubes.system())
        .add_system(text_update_system.system())
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, -4.0, 5.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -15.0, 150.0))
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
}

fn setup_world(
    commands: &mut Commands,
    mut physics_config: ResMut<bevy_rapier2d::physics::RapierConfiguration>,
) {
    physics_config.gravity = bevy_rapier2d::na::Vector2::new(0.0, 0.0);

    let ceiling_body = RigidBodyBuilder::new_static().translation(0.0, 65.0);
    let ceiling_collider = ColliderBuilder::cuboid(1000.0, 1.0);
    commands.spawn((ceiling_body, ceiling_collider));

    let floor_body = RigidBodyBuilder::new_static().translation(0.0, -61.0);
    let floor_collider = ColliderBuilder::cuboid(1000.0, 1.0);
    commands.spawn((floor_body, floor_collider));

    let left_body = RigidBodyBuilder::new_static().translation(-105.0, 0.0);
    let left_collider = ColliderBuilder::cuboid(1.0, 1000.0);
    commands.spawn((left_body, left_collider));

    let right_body = RigidBodyBuilder::new_static().translation(105.0, 0.0);
    let right_collider = ColliderBuilder::cuboid(1.0, 1000.0);
    commands.spawn((right_body, right_collider));
}

fn setup_cubes(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = StdRng::from_entropy();
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
    for _ in 0..50 {
        let position = Vec3::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0), 0.0);
        let body = RigidBodyBuilder::new_dynamic()
            .translation(position.x, position.y)
            .linvel(rng.gen_range(-30.0..30.0), rng.gen_range(-30.0..30.0));
        let collider = ColliderBuilder::cuboid(1.0, 1.0).restitution(1.5);
        commands
            .spawn(PbrBundle {
                mesh: cube_handle.clone(),
                material: materials.add(StandardMaterial {
                    albedo: Color::rgb(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .with_bundle((body, collider));
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
