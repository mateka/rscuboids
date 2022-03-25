use crate::cuboids::Cuboid;

use super::{
    assets::{Materials, Meshes},
    physics_layers,
};
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

#[derive(Debug, Component)]
pub struct Ship {
    pub lives: u8,
}

impl Ship {}

fn create_ship(mut commands: Commands, meshes: Res<Meshes>, materials: Res<Materials>) {
    const SHIP_SIZE: f32 = 8.0;
    let extent = 0.5 * SHIP_SIZE;
    let body = RigidBodyBundle {
        position: vector![0.0, -50.0].into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(extent, extent).into(),
        material: ColliderMaterial {
            restitution: 1.5,
            ..Default::default()
        }
        .into(),
        flags: ColliderFlags {
            collision_groups: physics_layers::ALL, // TODO: Do not collide with bullets
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    commands
        .spawn()
        .insert(Ship { lives: 3 })
        .insert_bundle(body)
        .insert_bundle(collider)
        .insert_bundle(PbrBundle {
            mesh: meshes.ship.clone(),
            material: materials.ship.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
}

fn ship_input_handling(
    keyboard_input: Res<Input<KeyCode>>,
    mut ships: Query<&mut RigidBodyVelocityComponent, With<Ship>>,
) {
    let mut speed = 0.0;

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        speed = -25.0;
    } else if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        speed = 25.0;
    }

    let velocity = vector![speed, 0.0];
    for mut body_handle in ships.iter_mut() {
        body_handle.0.linvel = velocity;
        body_handle.0.angvel = -speed * 5.0 / 25.0;
    }
}

fn collisions(
    mut commands: Commands,
    mut event_reader: EventReader<ContactEvent>,
    cuboids: Query<&Cuboid>,
    mut ships: Query<&mut Ship>,
) {
    let is_cuboid = |e: Entity| cuboids.get(e).is_ok();

    for event in event_reader.iter() {
        if let ContactEvent::Started(collider1, collider2) = event {
            let entity1 = collider1.entity();
            let entity2 = collider2.entity();

            if let Ok(mut ship) = ships.get_mut(entity1) {
                if is_cuboid(entity2) {
                    // TODO: mark cuboid for brekage, add end game system and do not check for collisions, when lives == 0
                    ship.lives -= 1;
                    commands.entity(entity2).despawn();
                    dbg!(format!("Hit: {:?} with {:?}", ship, entity2));
                }
            } else if let Ok(mut ship) = ships.get_mut(entity2) {
                if is_cuboid(entity1) {
                    // TODO: mark cuboid for brekage, add end game system and do not check for collisions, when lives == 0
                    ship.lives -= 1;
                    commands.entity(entity1).despawn();
                    dbg!(format!("Hit: {:?} with {:?}", ship, entity1));
                }
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct ShipSystem;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum ShipStagesSystem {
    Movement,
    CollisionsHandler,
}

#[derive(Default)]
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, create_ship)
            .add_system_set(
                SystemSet::new()
                    .label(ShipSystem)
                    .with_system(ship_input_handling)
                    .label(ShipStagesSystem::Movement)
                    .with_system(collisions)
                    .label(ShipStagesSystem::CollisionsHandler),
            );
    }
}
