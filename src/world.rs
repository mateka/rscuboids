use bevy::app::StartupStage;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{cuboids::Spawner, physics_layers, trap::Trap};

fn setup_3d_world(
    mut commands: Commands,
    mut physics_config: ResMut<bevy_rapier2d::physics::RapierConfiguration>,
) {
    // camera
    let mut camera = PerspectiveCameraBundle::new_3d();
    camera.transform = Transform::from_xyz(0.0, 1.0, 150.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera);

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 10000000.0f32,
            range: 10000000.0f32,
            color: Color::rgb(1.0, 1.0, 1.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, -4.0, 50.0),
        ..Default::default()
    });

    physics_config.gravity = vector![0.0, 0.0]
}

#[derive(Debug, Component)]
pub struct Wall;

fn setup_walls(mut commands: Commands) {
    const POSITION: f32 = 110.0;

    let mut spawn_wall = |pos| {
        let body = RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: vector![pos, 0.0].into(),
            ..Default::default()
        };
        let collider = ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 150.0).into(),
            flags: ColliderFlags {
                collision_groups: physics_layers::WALLS,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };
        commands
            .spawn()
            .insert(Wall)
            .insert_bundle(body)
            .insert_bundle(collider);
    };

    spawn_wall(-POSITION);
    spawn_wall(POSITION);
}

fn setup_traps(mut commands: Commands) {
    const POSITION: f32 = 100.0;

    let mut spawn_trap = |pos| {
        let body = RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: vector![0.0, pos].into(),
            ..Default::default()
        };
        let collider = ColliderBundle {
            collider_type: ColliderType::Sensor.into(),
            shape: ColliderShape::cuboid(110.0, 1.0).into(),
            flags: ColliderFlags {
                collision_groups: physics_layers::TRAPS,
                active_events: ActiveEvents::INTERSECTION_EVENTS,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };
        commands
            .spawn()
            .insert(Trap)
            .insert_bundle(body)
            .insert_bundle(collider);
    };

    spawn_trap(-POSITION);
    spawn_trap(POSITION);
}

fn setup_cubes_spawners(mut commands: Commands) {
    const POSITION: f32 = 75.0;

    let mut spawn_spawner = |pos, seconds| {
        commands.spawn_bundle((
            Spawner::new(
                Timer::from_seconds(seconds, true),
                Some(240..300),
                None,
                None,
            ),
            Transform::from_translation(Vec3::new(pos, 70.0, 0.0)),
        ));
    };

    spawn_spawner(-POSITION, 3.13);
    spawn_spawner(0.0, 1.5);
    spawn_spawner(POSITION, 2.79);
}

#[derive(Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_3d_world)
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_walls)
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_traps)
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_cubes_spawners);
    }
}
