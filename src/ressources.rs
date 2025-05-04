use raylib::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Ressource {
    model: Arc<Mutex<Model>>,
    position: Vector3,
    rotation: Vector3,
    scale: f32,
    typ: String,
    amount: u32
}

impl Ressource {
    pub fn new(model: Arc<Mutex<Model>>, position: Vector3, rotation: Vector3, scale: f32, typ: String, amount: u32) -> Self {
        Ressource {
            model,
            position,
            rotation,
            scale,
            typ,
            amount
        }
    }

    pub fn get_model(&self) -> std::sync::MutexGuard<'_, Model> {
        self.model.lock().unwrap()
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }
}