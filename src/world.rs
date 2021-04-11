use bevy::prelude::*;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

use super::{cuboids::Spawner, physics_layers, trap::Trap, PhysicsObjectSpawner};

fn setup_3d_world(
    mut commands: Commands,
    mut physics_config: ResMut<bevy_rapier2d::physics::RapierConfiguration>,
) {
    // camera
    let mut camera = PerspectiveCameraBundle::new_3d();
    camera.transform = Transform::from_xyz(0.0, 1.0, 300.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera);

    // light
    commands.spawn_bundle(LightBundle {
        light: Light {
            intensity: 10000000.0f32,
            range: 10000000.0f32,
            color: Color::rgb(1.0, 1.0, 1.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, -4.0, 50.0),
        ..Default::default()
    });

    physics_config.gravity = bevy_rapier2d::na::Vector2::new(0.0, 0.0)
}

pub struct Wall;

fn setup_walls(mut commands: Commands) {
    const POSITION: f32 = 110.0;

    let mut spawn_wall = |pos| {
        let body = RigidBodyBuilder::new_static().translation(pos, 0.0);
        let collider = ColliderBuilder::cuboid(1.0, 150.0).collision_groups(physics_layers::WALLS);
        commands.spawn_object(Wall, body, collider);
    };

    spawn_wall(-POSITION);
    spawn_wall(POSITION);
}

fn setup_traps(mut commands: Commands) {
    const POSITION: f32 = 100.0;

    let mut spawn_trap = |pos| {
        let body = RigidBodyBuilder::new_static().translation(0.0, pos);
        let collider = ColliderBuilder::cuboid(110.0, 1.0)
            .sensor(true)
            .collision_groups(physics_layers::TRAPS);
        commands.spawn_object(Trap, body, collider);
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
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_3d_world.system())
            .add_startup_system(setup_walls.system())
            .add_startup_system(setup_traps.system())
            .add_startup_system(setup_cubes_spawners.system());
    }
}
