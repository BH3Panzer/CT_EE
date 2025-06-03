
use bevy::{prelude::*};


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_terrain);
    }
}


#[derive(Component)]
pub struct Map {
    pub name: String
}

fn generate_terrain(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let plane_mesh = meshes.add(Cuboid::new(10. * 4., 0.0001, 10. * 4.));
    let color = materials.add(StandardMaterial::from_color(Color::srgb(0.2, 0.95, 0.3)));
    commands.spawn(
        (
            Map { name: "devTest".to_string() },
            Transform::from_xyz(5. * 4., 0., 5. * 4.),
            Mesh3d(plane_mesh.clone()),
            MeshMaterial3d(color)
        )
    );
}