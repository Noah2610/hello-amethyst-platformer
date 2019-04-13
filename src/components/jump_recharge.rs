use super::component_prelude::*;

pub struct JumpRecharge;

impl Component for JumpRecharge {
    type Storage = NullStorage<Self>;
}

impl Default for JumpRecharge {
    fn default() -> Self {
        Self
    }
}
