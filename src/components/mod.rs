mod jump_recharge;
mod player;

pub mod prelude {
    pub use deathframe::components::prelude::*;

    pub use super::JumpRecharge;
    pub use super::Player;
}

mod component_prelude {
    // NOTE: Quick storage type reference
    // DenseVecStorage: Reduced memory usage for LARGE components.
    // HashMapStorage:  "Best suited for rare components."
    // NullStorage:     Storage without data, used as a simple flag.
    // VecStorage:      Preferable for SMALL components (<= 16 bytes || <= 128 bits). For often used components.
    pub use amethyst::ecs::{
        Component,
        DenseVecStorage,
        HashMapStorage,
        NullStorage,
        Storage,
        VecStorage,
    };
}

pub use jump_recharge::JumpRecharge;
pub use player::Player;

pub use deathframe::components::add_component_to_entity_by_name;

use amethyst::ecs::EntityBuilder;
use amethyst::prelude::Builder;

pub fn add_component_to_entity_by_name_custom<'a, T>(
    mut entity: EntityBuilder<'a>,
    component_name: T,
) -> EntityBuilder<'a>
where
    T: ToString,
{
    match component_name.to_string().as_str() {
        "JumpRecharge" => entity = entity.with(jump_recharge::JumpRecharge),
        _ => (),
    }

    entity
}
