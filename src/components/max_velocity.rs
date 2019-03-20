use super::component_prelude::*;

pub struct MaxVelocity {
    pub x: f32,
    pub y: f32,
}

impl MaxVelocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for MaxVelocity {
    type Storage = VecStorage<Self>;
}

impl From<(f32, f32)> for MaxVelocity {
    fn from(data: (f32, f32)) -> Self {
        Self::new(data.0, data.1)
    }
}
