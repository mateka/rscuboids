use crate::physics_events::PhysicEventsSystem;

use super::IntersectionEvent;
use bevy::prelude::*;
use bevy_rapier2d::rapier::geometry::{ColliderHandle, ColliderSet};

#[derive(Debug, Default)]
pub struct Trap;

fn traps(
    mut commands: Commands,
    mut event_reader: EventReader<IntersectionEvent>,
    colliders: Res<ColliderSet>,
    traps: Query<&Trap>,
) {
    let collider_index_to_entity = |idx: ColliderHandle| {
        let collider = &colliders[idx];
        Entity::from_bits(collider.user_data as u64)
    };
    let mut despawn_if_trap = |trap, other| {
        if traps.get(trap).is_ok() {
            commands.entity(other).despawn();
        }
    };

    for event in event_reader.iter() {
        if event.intersecting {
            let entity_1 = collider_index_to_entity(event.collider1);
            let entity_2 = collider_index_to_entity(event.collider2);
            despawn_if_trap(entity_1, entity_2);
            despawn_if_trap(entity_2, entity_1);
        }
    }
}

#[derive(Default)]
pub struct TrapsPlugin;

impl Plugin for TrapsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(traps.system().after(PhysicEventsSystem));
    }
}
