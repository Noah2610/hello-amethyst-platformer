mod control_player;
mod debug;

pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::ControlPlayerSystem;
    pub use super::DebugSystem;
}

mod system_prelude {
    pub use amethyst::assets::AssetStorage;
    pub use amethyst::core::timing::Time;
    pub use amethyst::ecs::world::Index;
    pub use amethyst::ecs::{
        Entities,
        Entity,
        Join,
        Read,
        ReadExpect,
        ReadStorage,
        System,
        Write,
        WriteExpect,
        WriteStorage,
    };
    pub use amethyst::input::InputHandler;
    pub use amethyst::renderer::{
        SpriteRender,
        SpriteSheet,
        SpriteSheetHandle,
    };

    pub use crate::components::prelude::*;
    pub use crate::settings::prelude::*;
}

pub use control_player::ControlPlayerSystem;
pub use debug::DebugSystem;
