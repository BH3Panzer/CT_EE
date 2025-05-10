use std::sync::{Arc, Mutex};

use raylib::prelude::*;

pub struct Unit {
    model: Arc<Mutex<Model>>,
    position: Vector3,
    rotation: Vector3,
    scale: Vector3,
    typ: String,
}

impl Unit {
    pub fn new(model: Arc<Mutex<Model>>, position: Vector3, rotation: Vector3, scale: Vector3, typ: String) -> Self {
        Unit {
            model,
            position,
            rotation,
            scale,
            typ,
        }
    }
}