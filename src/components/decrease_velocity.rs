use super::component_prelude::*;

pub struct DecreaseVelocity {
    pub x:               f32,
    pub y:               f32,
    pub should_decrease: bool,
}

impl DecreaseVelocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            should_decrease: true,
        }
    }
}

impl Component for DecreaseVelocity {
    type Storage = VecStorage<Self>;
}

impl From<(f32, f32)> for DecreaseVelocity {
    fn from(data: (f32, f32)) -> Self {
        Self::new(data.0, data.1)
    }
}