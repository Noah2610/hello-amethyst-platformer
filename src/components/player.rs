use super::component_prelude::*;

pub struct Player;

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
