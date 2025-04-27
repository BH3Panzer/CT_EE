use raylib::prelude::*;

pub struct Ressource {
    model: Model,
    position: Vector3,
    rotation: Vector3,
    scale: f32,
    typ: String,
    amount: u32
}

impl Ressource {
    pub fn new(model: Model, position: Vector3, rotation: Vector3, scale: f32, typ: String, amount: u32) -> Self {
        Ressource {
            model,
            position,
            rotation,
            scale,
            typ,
            amount
        }
    }

    pub fn get_model(&self) -> &Model {
        &self.model
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }
}