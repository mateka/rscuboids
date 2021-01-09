use bevy::prelude::*;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::ops::Range;

use super::assets::{Materials, Meshes, CUBOID_MESH_SIZE};
use super::physics_layers;
use super::PhysicsObjectSpawner;

#[derive(Debug)]
pub struct Cuboid {
    pub size: u8,
}

impl Default for Cuboid {
    fn default() -> Self {
        Self { size: 4 }
    }
}

fn spawn_cuboid(
    commands: &mut Commands,
    meshes: &Res<Meshes>,
    materials: &Res<Materials>,
    size: u8,
    position: Vec2,
    velocity: Vec2,
) {
    let extent = 0.5 * CUBOID_MESH_SIZE * size as f32;
    let body = RigidBodyBuilder::new_dynamic()
        .translation(position.x, position.y)
        .linvel(velocity.x, velocity.y);
    let collider = ColliderBuilder::cuboid(extent, extent)
        .restitution(1.5)
        .collision_groups(physics_layers::ALL);

    commands
        .spawn_object(
            PbrBundle {
                mesh: meshes.cuboid[&size].clone(),
                material: materials.cuboid[&size].clone(),
                ..Default::default()
            },
            body,
            collider,
        )
        .with(Cuboid { size });
}

#[derive(Debug)]
pub struct Spawner {
    /// Cooldown timer for this spawner to spawn cuboid.
    cooldown: Timer,
    /// Possible cuboid's sizes to generate
    size_range: Range<u8>,
    /// At which angles (in radians) cuboids will be spawned.
    angle_range: Range<u16>,
    /// Possible velocity vector's lengths.
    speed_range: Range<u8>,
}

impl Spawner {
    pub fn new(
        cooldown: Timer,
        angle_range: Option<Range<u16>>,
        size_range: Option<Range<u8>>,
        speed_range: Option<Range<u8>>,
    ) -> Self {
        Self {
            cooldown,
            angle_range: angle_range.unwrap_or(0..360),
            size_range: size_range.unwrap_or(1..4),
            speed_range: speed_range.unwrap_or(10..50),
        }
    }
}

impl Default for Spawner {
    fn default() -> Self {
        Spawner {
            cooldown: Timer::default(),
            size_range: 1..4,
            angle_range: 0..360,
            speed_range: 10..50,
        }
    }
}

fn spawner_system(
    commands: &mut Commands,
    mut spawners: Query<(&Transform, &mut Spawner)>,
    time: Res<Time>,
    meshes: Res<Meshes>,
    materials: Res<Materials>,
) {
    for (transform, mut spawner) in spawners.iter_mut() {
        // Advance time in spawner and skip spawning, if time has not elapsed
        if !spawner.cooldown.tick(time.delta_seconds()).just_finished() {
            continue;
        }

        // Pick random size and velocity
        let mut rng = StdRng::from_entropy();
        let size = rng.gen_range(spawner.size_range.clone());
        let angle = (rng.gen_range(spawner.angle_range.clone()) as f32).to_radians();
        let speed = rng.gen_range(spawner.speed_range.clone()) as f32;
        let movement_direction = Vec2::new(angle.cos(), angle.sin());
        let velocity = speed * movement_direction;
        let position = Vec2::new(transform.translation.x, transform.translation.y)
            + (size as f32) * movement_direction;

        spawn_cuboid(commands, &meshes, &materials, size, position, velocity);
    }
}

#[derive(Default)]
pub struct CuboidsPlugin;

impl Plugin for CuboidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawner_system.system());
    }
}
