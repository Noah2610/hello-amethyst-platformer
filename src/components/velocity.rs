use super::component_prelude::*;

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
