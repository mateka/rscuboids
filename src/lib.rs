use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod physics_layers {
    use bevy_rapier2d::rapier::geometry::InteractionGroups;

    pub const WALLS: InteractionGroups =
        InteractionGroups::new(0b0000_0000_0000_0001, 0b0000_0000_0000_0001);
    pub const TRAPS: InteractionGroups =
        InteractionGroups::new(0b0000_0000_0000_0010, 0b0000_0000_0000_0010);
    pub const ALL: InteractionGroups = InteractionGroups::all();
}

pub mod assets;
pub mod cuboids;
mod physics_events;
mod physics_object_spawner;
pub mod trap;
pub mod world;

pub use physics_events::{ContactEvent, Proximity, ProximityEvent};
pub use physics_object_spawner::PhysicsObjectSpawner;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(crate::physics_events::PhysicEventsPlugin::default())
            .add(crate::assets::AssetsPlugin::default())
            .add(crate::trap::TrapsPlugin::default())
            .add(crate::cuboids::CuboidsPlugin::default())
            .add(crate::world::WorldPlugin::default());
    }
}
