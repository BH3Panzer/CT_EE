use raylib::ffi::{GetWorldToScreen, GetWorldToScreen2D};
use raylib::prelude::*;

use crate::unit::Unit;
use crate::ressources::Ressource;
use crate::ground_tile::GroundTile;

use rand::prelude::*;

pub struct Map {
    ressources: Vec<Ressource>,
    units: Vec<Unit>,
    tiles: Vec<GroundTile>,
    map_size: f32
}

impl Map {
    pub fn new(map_size: f32) -> Self {
        Map {
            ressources: Vec::new(),
            units: Vec::new(),
            tiles: Vec::new(),
            map_size: map_size
        }
    }

    pub fn add_ressource(&mut self, ressource: Ressource) {
        self.ressources.push(ressource);
    }

    pub fn add_tile(&mut self, tile: GroundTile) {
        self.tiles.push(tile);
    }

    pub fn dev_gen(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {

        for x in 0..self.map_size as i32 * 8 {
            for y in 0..self.map_size as i32 * 8 {
                let ground_tile_model: Model = rl.load_model(thread, "src/assets/models/grass_ground.gltf").unwrap();
                let ground_tile: GroundTile = GroundTile::new(ground_tile_model, Vector3::new(x as f32 * 2.0, 0.0, y as f32 * 2.0), Vector3::new(0.0, 0.0, 0.0), 2.0);
                self.add_tile(ground_tile);
            }
        }

        let rock_spot_model: Model = rl.load_model(thread, "src/assets/models/rockSpot.gltf").unwrap();
        
        let rock_ressource: Ressource = Ressource::new(rock_spot_model, Vector3::new(2.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 0.0), 1.0, "rock".to_string(), 100);
        
        self.add_ressource(rock_ressource);
        
        for _ in 0..5 {
            let food_spot_model: Model = rl.load_model(thread, "src/assets/models/foodSpot.gltf").unwrap();
            let food_spot: Ressource = Ressource::new(food_spot_model, Vector3::new(rand::rng().random_range(0.0..self.map_size * 16.0), 0.0, rand::rng().random_range(0.0..self.map_size * 16.0)), Vector3::new(0.0, 0.0, 0.0), 1.0, "food".to_string(), 100);
            self.add_ressource(food_spot);
        }
        
        
        for _ in 0..500 {
            let tree_model: Model = rl.load_model(thread, "src/assets/models/arbre.gltf").unwrap();
            let tree: Ressource = Ressource::new(tree_model, Vector3::new(rand::rng().random_range(0.0..self.map_size * 16.0), 0.0, rand::rng().random_range(0.0..self.map_size * 16.0)), Vector3::new(0.0, 0.0, 0.0), 2.0, "tree".to_string(), 100);
            self.add_ressource(tree);
        }
    }

    pub fn draw(&mut self, d3d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>, camera: &Camera3D, screen_width: i32, screen_height: i32) {
        for tile in self.tiles.iter() {
            if unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(tile.get_position()), Into::into(camera)).x} <= screen_width as f32 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(tile.get_position()), Into::into(camera)).x} >= 0.0 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(tile.get_position()), Into::into(camera)).y} <= screen_height as f32 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(tile.get_position()), Into::into(camera)).y} >= 0.0 {
                d3d.draw_model(tile.get_model(), tile.get_position(), tile.get_scale(), Color::RAYWHITE);
            }
            
        }
        for ressource in self.ressources.iter() {
            if unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(ressource.get_position()), Into::into(camera)).x} <= screen_width as f32 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(ressource.get_position()), Into::into(camera)).x} >= 0.0 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(ressource.get_position()), Into::into(camera)).y} <= screen_height as f32 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(ressource.get_position()), Into::into(camera)).y} >= 0.0 {
                d3d.draw_model(ressource.get_model(), ressource.get_position(), ressource.get_scale(), Color::RAYWHITE);
            }
        }
    }
}