use super::component_prelude::*;

/// `Pushable` can be pushed by other `Push` entities.
pub struct Pushable;

impl Component for Pushable {
    type Storage = NullStorage<Self>;
}

impl Default for Pushable {
    fn default() -> Self {
        Self
    }
}
