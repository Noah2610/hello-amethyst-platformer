use super::component_prelude::*;

pub struct Player {
    pub speed: (f32, f32),
}

impl Player {
    pub fn with_speed(speed: (f32, f32)) -> Self {
        Self { speed }
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
