use super::component_prelude::*;
use crate::geo::Side;

use amethyst::ecs::Entity;

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

    pub fn on_collide(&mut self, entity: &Entity, side: Side) {

    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
