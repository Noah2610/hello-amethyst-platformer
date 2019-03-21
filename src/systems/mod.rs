mod control_player;
mod decrease_velocities;
mod gravity;
mod limit_velocities;
mod move_entities;
mod scale_sprites;

pub mod prelude {
    pub use super::ControlPlayerSystem;
    pub use super::DecreaseVelocitiesSystem;
    pub use super::GravitySystem;
    pub use super::LimitVelocitiesSystem;
    pub use super::MoveEntitiesSystem;
    pub use super::ScaleSpritesSystem;
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
pub use decrease_velocities::DecreaseVelocitiesSystem;
pub use gravity::GravitySystem;
pub use limit_velocities::LimitVelocitiesSystem;
pub use move_entities::MoveEntitiesSystem;
pub use scale_sprites::ScaleSpritesSystem;
