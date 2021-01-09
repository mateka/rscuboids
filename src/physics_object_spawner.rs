use bevy::prelude::*;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

pub trait PhysicsObjectSpawner {
    fn spawn_object(
        &mut self,
        bundle: impl Bundle + Send + Sync + 'static,
        body_builder: RigidBodyBuilder,
        collider_builder: ColliderBuilder,
    ) -> &mut Self;
}

impl PhysicsObjectSpawner for Commands {
    fn spawn_object(
        &mut self,
        bundle: impl Bundle + Send + Sync + 'static,
        body_builder: RigidBodyBuilder,
        collider_builder: ColliderBuilder,
    ) -> &mut Self {
        let entity_id = self.spawn(bundle).current_entity().unwrap();
        let user_data = entity_id.to_bits() as u128;
        self.with_bundle((
            body_builder.user_data(user_data),
            collider_builder.user_data(user_data),
        ))
    }
}
