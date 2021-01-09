use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod assets;
pub mod cuboids;
mod physics_events;

pub use physics_events::{ContactEvent, ProximityEvent};

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(crate::physics_events::PhysicEventsPlugin::default());
        group.add(crate::assets::AssetsPlugin::default());
        group.add(crate::cuboids::CuboidsPlugin::default());
    }
}
