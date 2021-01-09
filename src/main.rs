use bevy::prelude::*;
use bevy_rapier2d::physics::RapierPhysicsPlugin;
use bevy_rapier2d::render::RapierRenderPlugin;

use rscuboids::GamePlugins;

#[bevy_main]
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "rsCuboids".to_string(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_plugins(GamePlugins)
        .run();
}
