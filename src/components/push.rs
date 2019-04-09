use super::component_prelude::*;

/// `Push` can push other `Pushable` entities,
/// when moving (with `Transform` and `Velocity`).
pub struct Push;

impl Component for Push {
    type Storage = NullStorage<Self>;
}

impl Default for Push {
    fn default() -> Self {
        Self
    }
}
