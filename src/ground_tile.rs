use raylib::prelude::*;

pub struct GroundTile {
    pub model: Model,
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: f32,
}

impl GroundTile {
    pub fn new(model: Model, position: Vector3, rotation: Vector3, scale: f32) -> Self {
        GroundTile {
            model,
            position,
            rotation,
            scale,
        }
    }

    pub fn get_model(&self) -> &Model {
        &self.model
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }
}