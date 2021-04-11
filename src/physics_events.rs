use bevy::prelude::*;
use bevy_rapier2d::physics::EventQueue;
pub use bevy_rapier2d::rapier::geometry::{ContactEvent, IntersectionEvent};

fn contact_events_producer(events: Res<EventQueue>, mut contact_events: EventWriter<ContactEvent>) {
    while let Ok(contact_event) = events.contact_events.pop() {
        contact_events.send(contact_event);
    }
}

fn intersection_events_producer(
    events: Res<EventQueue>,
    mut intersection_events: EventWriter<IntersectionEvent>,
) {
    while let Ok(intersection_event) = events.intersection_events.pop() {
        intersection_events.send(intersection_event);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct PhysicEventsSystem;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum PhysicEventsStagesSystem {
    Intersection,
    Contact,
}

/// Iterate over bevy_rapier2d events and emits proper bevy events
#[derive(Default)]
pub struct PhysicEventsPlugin;

impl Plugin for PhysicEventsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ContactEvent>()
            .add_event::<IntersectionEvent>()
            .add_system_set(
                SystemSet::new()
                    .label(PhysicEventsSystem)
                    .with_system(
                        intersection_events_producer
                            .system()
                            .label(PhysicEventsStagesSystem::Intersection),
                    )
                    .with_system(
                        contact_events_producer
                            .system()
                            .label(PhysicEventsStagesSystem::Contact)
                            .after(PhysicEventsStagesSystem::Intersection),
                    ),
            );
    }
}
