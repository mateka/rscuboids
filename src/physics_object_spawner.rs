use bevy::{
    ecs::{component::Component, system::EntityCommands},
    prelude::*,
};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

pub trait PhysicsObjectSpawner<'a> {
    fn spawn_object(
        &mut self,
        component: impl Component,
        body_builder: RigidBodyBuilder,
        collider_builder: ColliderBuilder,
    ) -> EntityCommands<'a, '_>;
}

impl<'a> PhysicsObjectSpawner<'a> for Commands<'a> {
    fn spawn_object(
        &mut self,
        component: impl Component,
        body_builder: RigidBodyBuilder,
        collider_builder: ColliderBuilder,
    ) -> EntityCommands<'a, '_> {
        let mut entity = self.spawn();
        let user_data = entity.id().to_bits() as u128;
        entity
            .insert(component)
            .insert(body_builder.user_data(user_data))
            .insert(collider_builder.user_data(user_data));
        entity
    }
}
