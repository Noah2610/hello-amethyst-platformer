use super::component_prelude::*;

#[derive(Serialize, Deserialize)]
pub struct JumpRecharge;

impl Component for JumpRecharge {
    type Storage = NullStorage<Self>;
}

impl Default for JumpRecharge {
    fn default() -> Self {
        Self
    }
}
