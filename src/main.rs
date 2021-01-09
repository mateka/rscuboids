use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier2d::physics::RapierPhysicsPlugin;
use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder,
    geometry::{ColliderBuilder, ColliderSet},
};
use bevy_rapier2d::render::RapierRenderPlugin;

use rscuboids::{cuboids::Spawner, ContactEvent, GamePlugins};

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
        .add_startup_system(setup_world.system())
        .add_startup_system(setup_cube_spawners.system())
        .add_system(text_update_system.system())
        .add_system(print_colision_started_events.system())
        .add_system(print_colision_stopped_events.system())
        .run();
}

fn print_colision_started_events(
    mut state: Local<EventReader<ContactEvent>>,
    my_events: Res<Events<ContactEvent>>,
    colliders: Res<ColliderSet>,
) {
    for contact_event in state.iter(&my_events) {
        match contact_event {
            ContactEvent::Started(c1, c2) => {
                let collider_1 = &colliders[*c1];
                let collider_2 = &colliders[*c2];

                if collider_1.user_data != 0 {
                    let eid = Entity::from_bits(collider_1.user_data as u64);
                    println!("STARTED: C1 is a cuboid with Entity ID {:?}", eid);
                }
                if collider_2.user_data != 0 {
                    let eid = Entity::from_bits(collider_2.user_data as u64);
                    println!("STARTED: C2 is a cuboid with Entity ID {:?}", eid);
                }
            }
            ContactEvent::Stopped(_, _) => {}
        }
    }
}

fn print_colision_stopped_events(
    mut state: Local<EventReader<ContactEvent>>,
    my_events: Res<Events<ContactEvent>>,
    colliders: Res<ColliderSet>,
) {
    for contact_event in state.iter(&my_events) {
        match contact_event {
            ContactEvent::Stopped(c1, c2) => {
                let collider_1 = &colliders[*c1];
                let collider_2 = &colliders[*c2];

                if collider_1.user_data != 0 {
                    let eid = Entity::from_bits(collider_1.user_data as u64);
                    println!("STOPED: C1 is a cuboid with Entity ID {:?}", eid);
                }
                if collider_2.user_data != 0 {
                    let eid = Entity::from_bits(collider_2.user_data as u64);
                    println!("STOPED: C2 is a cuboid with Entity ID {:?}", eid);
                }
            }
            ContactEvent::Started(_, _) => {}
        }
    }
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

    // let floor_body = RigidBodyBuilder::new_static().translation(0.0, -61.0);
    let floor_body = RigidBodyBuilder::new_static().translation(0.0, -50.0);
    let floor_collider = ColliderBuilder::cuboid(1000.0, 1.0).sensor(true); //.user_data(data);
    commands.spawn((floor_body, floor_collider));

    let left_body = RigidBodyBuilder::new_static().translation(-105.0, 0.0);
    let left_collider = ColliderBuilder::cuboid(1.0, 1000.0);
    commands.spawn((left_body, left_collider));

    let right_body = RigidBodyBuilder::new_static().translation(105.0, 0.0);
    let right_collider = ColliderBuilder::cuboid(1.0, 1000.0);
    commands.spawn((right_body, right_collider));
}

fn setup_cube_spawners(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 2.0,
        ..Default::default()
    }));
    let material = materials.add(StandardMaterial {
        albedo: Color::rgb(0.75, 0.6, 0.0),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(50.0, -15.0, 0.0)),
            ..Default::default()
        })
        .with(Spawner::new(
            Timer::from_seconds(1.0, true),
            Some(180..360),
            None,
            None,
        ));

    commands
        .spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(Vec3::new(-50.0, 15.0, 0.0)),
            ..Default::default()
        })
        .with(Spawner::new(
            Timer::from_seconds(1.0, true),
            Some(0..180),
            None,
            None,
        ));
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
