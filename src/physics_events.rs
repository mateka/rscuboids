use bevy::prelude::*;
use bevy_rapier2d::physics::EventQueue;
pub use bevy_rapier2d::rapier::geometry::{ContactEvent, Proximity, ProximityEvent};

fn proximity_events_producer(
    events: Res<EventQueue>,
    mut proximity_events: ResMut<Events<ProximityEvent>>,
) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        proximity_events.send(proximity_event);
    }
}

fn contact_events_producer(
    events: Res<EventQueue>,
    mut contact_events: ResMut<Events<ContactEvent>>,
) {
    while let Ok(contact_event) = events.contact_events.pop() {
        contact_events.send(contact_event);
    }
}

/// Iterate over bevy_rapier2d events and emits proper bevy events
#[derive(Default)]
pub struct PhysicEventsPlugin;

impl Plugin for PhysicEventsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ContactEvent>()
            .add_event::<ProximityEvent>()
            .add_system(proximity_events_producer.system())
            .add_system(contact_events_producer.system());
    }
}
