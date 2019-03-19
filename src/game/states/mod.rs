mod ingame;
mod paused;
mod startup;

pub mod prelude {
    pub use super::Ingame;
    pub use super::Paused;
    pub use super::Startup;
}

mod state_prelude {
    pub use amethyst::assets::{AssetStorage, Loader};
    pub use amethyst::ecs::World;
    pub use amethyst::input::{is_close_requested, is_key_down};
    pub use amethyst::renderer::{
        PngFormat,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
        VirtualKeyCode,
    };
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use super::super::constants;
    pub use super::super::constants::keys;
    pub use crate::custom_game_data::prelude::*;
    pub use crate::resource_helpers::*;
}

pub use ingame::Ingame;
pub use paused::Paused;
pub use startup::Startup;
