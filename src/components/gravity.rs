use super::component_prelude::*;

#[derive(Deserialize)]
pub struct Gravity {
    pub x: f32,
    pub y: f32,
}

impl Gravity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for Gravity {
    type Storage = VecStorage<Self>;
}

impl From<(f32, f32)> for Gravity {
    fn from(data: (f32, f32)) -> Self {
        Self::new(data.0, data.1)
    }
}
