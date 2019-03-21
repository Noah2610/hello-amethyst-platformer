use super::component_prelude::*;

pub struct Solid;

impl Component for Solid {
    type Storage = NullStorage<Self>;
}

impl Default for Solid {
    fn default() -> Self {
        Self
    }
}
