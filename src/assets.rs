use bevy::prelude::*;
use std::collections::HashMap;

pub const CUBOID_MESH_SIZE: f32 = 3.75;

pub struct Meshes {
    /// Map from cuboid size to mesh
    pub cuboid: HashMap<u8, Handle<Mesh>>,
}

pub struct Materials {
    /// Map from cuboid size to material
    pub cuboid: HashMap<u8, Handle<StandardMaterial>>,
}

fn generate_assets(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Meshes {
        cuboid: (1..10)
            .map(|s| {
                (
                    s,
                    meshes.add(Mesh::from(shape::Cube {
                        size: CUBOID_MESH_SIZE * s as f32,
                    })),
                )
            })
            .collect::<HashMap<_, _>>(),
    });
    commands.insert_resource(Materials {
        cuboid: (1..10)
            .map(|s| {
                (
                    s,
                    materials.add(StandardMaterial {
                        albedo: Color::rgb(0.65, 0.6, 0.6),
                        ..Default::default()
                    }),
                )
            })
            .collect::<HashMap<_, _>>(),
    });
}

#[derive(Default)]
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(generate_assets.system());
    }
}
