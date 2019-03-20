use super::component_prelude::*;

pub struct Scale;

impl Component for Scale {
    type Storage = NullStorage<Self>;
}

impl Default for Scale {
    fn default() -> Self {
        Self
    }
}
