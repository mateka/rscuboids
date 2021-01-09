use super::{Proximity, ProximityEvent};
use bevy::prelude::*;
use bevy_rapier2d::rapier::geometry::{ColliderHandle, ColliderSet};

#[derive(Debug, Default)]
pub struct Trap;

fn traps(
    commands: &mut Commands,
    mut event_reader: Local<EventReader<ProximityEvent>>,
    proximity_events: Res<Events<ProximityEvent>>,
    colliders: Res<ColliderSet>,
    traps: Query<&Trap>,
) {
    let collider_index_to_entity = |idx: ColliderHandle| {
        let collider = &colliders[idx];
        assert!(collider.user_data != 0);
        Entity::from_bits(collider.user_data as u64)
    };
    let mut despawn_if_trap = |trap, other| {
        if traps.get(trap).is_ok() {
            commands.despawn(other);
        }
    };

    for proximity_event in event_reader.iter(&proximity_events) {
        if proximity_event.new_status == Proximity::Intersecting {
            let entity_1 = collider_index_to_entity(proximity_event.collider1);
            let entity_2 = collider_index_to_entity(proximity_event.collider2);
            despawn_if_trap(entity_1, entity_2);
            despawn_if_trap(entity_2, entity_1);
        }
    }
}

#[derive(Default)]
pub struct TrapsPlugin;

impl Plugin for TrapsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(traps.system());
    }
}
