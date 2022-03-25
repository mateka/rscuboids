use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod physics_layers {
    use bevy_rapier2d::prelude::InteractionGroups;

    pub const WALLS: InteractionGroups =
        InteractionGroups::new(0b0000_0000_0000_0001, 0b0000_0000_0000_0001);
    pub const TRAPS: InteractionGroups =
        InteractionGroups::new(0b0000_0000_0000_0010, 0b0000_0000_0000_0010);
    pub const ALL: InteractionGroups = InteractionGroups::all();
}

mod assets;
mod cuboids;
mod scoring;
mod ship;
mod trap;
mod ui;
mod world;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(crate::assets::AssetsPlugin::default())
            .add(crate::scoring::ScoringPlugin::default())
            .add(crate::ui::UiPlugin::default())
            .add(crate::trap::TrapsPlugin::default())
            .add(crate::cuboids::CuboidsPlugin::default())
            .add(crate::world::WorldPlugin::default())
            .add(crate::ship::ShipPlugin::default());
    }
}
