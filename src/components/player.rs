use super::component_prelude::*;

pub struct Player {
    pub speed:               (f32, f32),
    pub is_jump_button_down: bool,
}

impl Player {
    pub fn with_speed(speed: (f32, f32)) -> Self {
        Self {
            speed,
            is_jump_button_down: false,
        }
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
