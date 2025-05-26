use std::f32::consts::PI;
use rand::prelude::*;
use bevy::{prelude::*};


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_terrain);
    }
}


#[derive(Component, Debug)]
struct GroundTile;

fn create_tiles(asset_server: Res<AssetServer>) -> Vec<(GroundTile, Transform, SceneRoot)> {
    let mut rng = rand::rng();
    let mut rotation_chance: u8;
    let mut rotation: f32;
    let scene_handle: Handle<Scene> = asset_server.load("models/grass_ground.gltf#Scene0");
    let width: u8 = 50;
    let height: u8 = 50;
    let mut tiles: Vec<(GroundTile, Transform, SceneRoot)> = Vec::with_capacity(width as usize * height as usize);
    let scene_root = &SceneRoot(scene_handle);
    for y in 0..width {
        for x in 0..height {
            rotation_chance = rng.random_range(0..3);
            if rotation_chance == 0 {
                rotation = 0.;
            } else if rotation_chance == 1 {
                rotation = PI/2.;
            } else if rotation_chance == 2 {
                rotation = PI;
            } else if rotation_chance == 3 {
                rotation = (PI/2.) * 3.;
            } else {
                println!("RNG error in map gen, rotation");
                rotation = 0.;
            }
            tiles.push((
                GroundTile,
                Transform::from_xyz(x as f32 * 4., 0., y as f32 * 4.).with_scale(Vec3 {x: 4., y: 4., z: 4.}).with_rotation(Quat::from_axis_angle(Vec3 { x: 0., y: 1., z: 0. }, rotation)),
                scene_root.clone()
            ));
        }
    }
    return tiles;
}


fn generate_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_batch(create_tiles(asset_server));
}