use bevy::{ecs::schedule::ReportExecutionOrderAmbiguities, prelude::*};
use bevy_rapier2d::prelude::*;

use rscuboids::GamePlugins;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(WindowDescriptor {
            title: "rsCuboids".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(GamePlugins)
        .run();
}
