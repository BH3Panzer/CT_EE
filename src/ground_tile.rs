use std::sync::Arc;
use std::sync::Mutex;

use raylib::prelude::*;

pub struct GroundTile {
    pub model: Arc<Mutex<Model>>,
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: f32,
}

impl GroundTile {
    pub fn new(model: Arc<Mutex<Model>>, position: Vector3, rotation: Vector3, scale: f32) -> Self {
        GroundTile {
            model,
            position,
            rotation,
            scale,
        }
    }

    pub fn get_model(&self) -> std::sync::MutexGuard<'_, Model> {
        self.model.lock().unwrap()
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }

    pub fn get_rotation(&self) -> &Vector3 {
        &self.rotation
    }
}