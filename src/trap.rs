use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// use bevy_rapier2d::rapier::geometry::{ColliderHandle, ColliderSet};

#[derive(Debug, Default, Component)]
pub struct Trap;

fn traps(
    mut commands: Commands,
    mut intersection_events: EventReader<IntersectionEvent>,
    traps: Query<&Trap>,
) {
    let mut despawn_if_trap = |trap, other| {
        if traps.get(trap).is_ok() {
            commands.entity(other).despawn();
        }
    };
    for event in intersection_events.iter() {
        if event.intersecting {
            let entity_1 = event.collider1.entity();
            let entity_2 = event.collider2.entity();
            despawn_if_trap(entity_1, entity_2);
            despawn_if_trap(entity_2, entity_1);
        }
    }
}

#[derive(Default)]
pub struct TrapsPlugin;

impl Plugin for TrapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(traps);
    }
}
