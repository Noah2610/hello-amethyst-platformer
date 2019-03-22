use super::component_prelude::*;

pub struct ScaleOnce;

impl Component for ScaleOnce {
    type Storage = NullStorage<Self>;
}

impl Default for ScaleOnce {
    fn default() -> Self {
        Self
    }
}
