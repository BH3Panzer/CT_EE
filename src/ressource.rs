use bevy::{prelude::*};
use rand::{self, Rng};

pub struct RessourcePlugin;

impl Plugin for RessourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_ressources);
    }
}
#[derive(Component)]
struct Ressource {
    ressource_type: RessourceType,
    quantity: u32
}

#[derive(Component)]
enum RessourceType {
    Wood,
    Food
}

fn generate_ressources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_handle_tree: Handle<Scene> = asset_server.load("models/arbre.gltf#Scene0");
    let scene_handle_food: Handle<Scene> = asset_server.load("models/foodSpot.gltf#Scene0");
    let scene_root_tree = &SceneRoot(scene_handle_tree);
    let scene_root_food = &SceneRoot(scene_handle_food);
    let mut rng = rand::rng();
    
    for _ in 0..20000 {
        commands.spawn((
            Ressource {
                ressource_type: RessourceType::Wood,
                quantity: 100
            },
            Transform::from_xyz(rng.random_range(0..(150 * 4)) as f32, 0., rng.random_range(0..(150 * 4)) as f32).with_rotation(Quat::from_rotation_y(rng.random_range(0.0..6.28))),
            scene_root_tree.clone()
        ));
    }

    for _ in 0..1000 {
        commands.spawn((
            Ressource {
                ressource_type: RessourceType::Food,
                quantity: 2000
            },
            Transform::from_xyz(rng.random_range(0..(150 * 4)) as f32, 0., rng.random_range(0..(150 * 4)) as f32).with_scale(Vec3::new(0.5, 0.5, 0.5)),
            scene_root_food.clone()
        ));
    }
}