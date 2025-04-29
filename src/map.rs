use raylib::prelude::*;

use crate::unit::Unit;
use crate::ressources::Ressource;

use rand::prelude::*;

pub struct Map {
    ressources: Vec<Ressource>,
    units: Vec<Unit>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            ressources: Vec::new(),
            units: Vec::new(),
        }
    }

    pub fn add_ressource(&mut self, ressource: Ressource) {
        self.ressources.push(ressource);
    }

    pub fn dev_gen(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let rock_spot_model: Model = rl.load_model(thread, "src/assets/models/rockSpot.gltf").unwrap();
        
        self.add_ressource(Ressource::new(rock_spot_model, Vector3::new(-2.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 0.0), 1.0, "rock".to_string(), 100));
        
        for i in 0..5 {
            let food_spot_model: Model = rl.load_model(thread, "src/assets/models/foodSpot.gltf").unwrap();
            self.add_ressource(Ressource::new(food_spot_model, Vector3::new(rand::rng().gen_range(-50.0..50.0), 0.0, rand::rng().gen_range(-50.0..50.0)), Vector3::new(0.0, 0.0, 0.0), 1.0, "food".to_string(), 100));
        }
        
        
        for i in 0..500 {
            let tree_model: Model = rl.load_model(thread, "src/assets/models/arbre.gltf").unwrap();
            self.add_ressource(Ressource::new(tree_model, Vector3::new(rand::rng().gen_range(-50.0..50.0), 0.0, rand::rng().gen_range(-50.0..50.0)), Vector3::new(0.0, 0.0, 0.0), 2.0, "tree".to_string(), 100));
        }
    }

    pub fn draw(&mut self, d3d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>) {
        for ressource in self.ressources.iter() {
            d3d.draw_model(ressource.get_model(), ressource.get_position(), ressource.get_scale(), Color::RAYWHITE);
        }
    }
}