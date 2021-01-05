use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod assets;
pub mod cuboids;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(crate::assets::AssetsPlugin::default());
        group.add(crate::cuboids::CuboidsPlugin::default());
    }
}
