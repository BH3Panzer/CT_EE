use raylib::ffi::GetWorldToScreen;
use raylib::prelude::*;

use crate::unit::Unit;
use crate::ressources::Ressource;
use crate::ground_tile::GroundTile;

use rand::prelude::*;

use std::sync::Arc;
use std::sync::Mutex;

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

    pub fn dev_gen(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, shader: &Shader) {
        let food_model = Arc::new(Mutex::new(rl.load_model(thread, "src/assets/models/foodSpot.gltf").unwrap()));
        let tree_model = Arc::new(Mutex::new(rl.load_model(thread, "src/assets/models/arbre.gltf").unwrap()));
        let ground_model = Arc::new(Mutex::new(rl.load_model(thread, "src/assets/models/grass_ground.gltf").unwrap()));
        let rock_model = Arc::new(Mutex::new(rl.load_model(thread, "src/assets/models/rockSpot.gltf").unwrap()));
        for material in food_model.lock().unwrap().materials_mut() {
            material.shader = *shader.as_ref();
        }
        for material in tree_model.lock().unwrap().materials_mut() {
            material.shader = *shader.as_ref();
        }
        for material in ground_model.lock().unwrap().materials_mut() {
            material.shader = *shader.as_ref();
        }
        for material in rock_model.lock().unwrap().materials_mut() {
            material.shader = *shader.as_ref();
        }

        for x in 0..self.map_size as i32 * 8 {
            for y in 0..self.map_size as i32 * 8 {
                let rotation: f32;
                let rotation_chance: u8 = rand::rng().random_range(0..3);
                if rotation_chance == 0 {
                    rotation = 0.0;
                } else if rotation_chance == 1 {
                    rotation = 90.0;
                } else if rotation_chance == 2 {
                    rotation = 180.0;
                } else {
                    rotation = 270.0;
                }
                let ground_tile: GroundTile = GroundTile::new(Arc::clone(&ground_model), Vector3::new(x as f32 * 2.0, 0.0, y as f32 * 2.0), Vector3::new(0.0, rotation, 0.0), 2.0);
                self.add_tile(ground_tile);
            }
        }
        
        
        for _ in 0..5 {
            let rotation: f32;
            let rotation_chance: u8 = rand::rng().random_range(0..3);
            if rotation_chance == 0 {
                rotation = 0.0;
            } else if rotation_chance == 1 {
                rotation = 90.0;
            } else if rotation_chance == 2 {
                rotation = 180.0;            
            } else {
                rotation = 270.0;
            }
            
            let food_spot: Ressource = Ressource::new(Arc::clone(&food_model), Vector3::new(rand::rng().random_range(0.0..self.map_size * 16.0), 0.0, rand::rng().random_range(0.0..self.map_size * 16.0)), Vector3::new(0.0, rotation, 0.0), 1.0, "food".to_string(), 100);
            self.add_ressource(food_spot);
        }
        
        
        
        for _ in 0..500 {
            let rotation: u16 = rand::rng().random_range(0..360);
            let tree = Ressource::new(Arc::clone(&tree_model), Vector3::new(rand::rng().random_range(0.0..self.map_size * 16.0), 0.0, rand::rng().random_range(0.0..self.map_size * 16.0)), Vector3::new(0.0, rotation as f32, 0.0), 3.0, "tree".to_string(), 200);
            self.add_ressource(tree);
        }

        for _ in 0..3 {
            let rotation: f32;
            let rotation_chance: u8 = rand::rng().random_range(0..3);
            if rotation_chance == 0 {
                rotation = 0.0;
            } else if rotation_chance == 1 {
                rotation = 90.0;
            } else if rotation_chance == 2 {
                rotation = 180.0;
            } else {
                rotation = 270.0;
            }
            let rock_spot: Ressource = Ressource::new(Arc::clone(&rock_model), Vector3::new(rand::rng().random_range(0.0..self.map_size * 16.0), 0.0, rand::rng().random_range(0.0..self.map_size * 16.0)), Vector3::new(0.0, rotation, 0.0), 1.0, "rock".to_string(), 100);
            self.add_ressource(rock_spot);
        }
    }

    pub fn draw(&mut self, d3d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>, camera: &Camera3D, screen_width: i32, screen_height: i32) {
        let rotation_axis: Vector3 = Vector3::new(0.0, 1.0, 0.0);
        for tile in self.tiles.iter() {
            if unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(tile.get_position()), Into::into(camera)).x} <= screen_width as f32 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(tile.get_position()), Into::into(camera)).x} >= 0.0 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(tile.get_position()), Into::into(camera)).y} <= screen_height as f32 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(tile.get_position()), Into::into(camera)).y} >= 0.0 {
                d3d.draw_model_ex(&*tile.get_model(), tile.get_position(), rotation_axis, tile.get_rotation().y, Vector3::new(tile.get_scale(), tile.get_scale(), tile.get_scale()), Color::RAYWHITE);
            }
            
        }
        for ressource in self.ressources.iter() {
            if unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(ressource.get_position()), Into::into(camera)).x} <= screen_width as f32 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(ressource.get_position()), Into::into(camera)).x} >= 0.0 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(ressource.get_position()), Into::into(camera)).y} <= screen_height as f32 && unsafe {GetWorldToScreen(raylib::ffi::Vector3::from(ressource.get_position()), Into::into(camera)).y} >= 0.0 {
                d3d.draw_model_ex(&*ressource.get_model(), ressource.get_position(), rotation_axis, ressource.get_rotation().y, Vector3::new(ressource.get_scale(), ressource.get_scale(), ressource.get_scale()), Color::RAYWHITE);
            }
        }
    }
}