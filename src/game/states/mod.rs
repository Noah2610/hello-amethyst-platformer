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
    pub use amethyst::ecs::{Entity, World};
    pub use amethyst::input::is_close_requested;
    pub use amethyst::prelude::*;
    pub use amethyst::renderer::{
        Camera as AmethystCamera,
        PngFormat,
        Projection,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
        Transparent,
        VirtualKeyCode,
    };
    pub use amethyst::ui::{
        Anchor as AmethystAnchor,
        TtfFormat,
        UiText,
        UiTransform,
    };
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use crate::custom_game_data::prelude::*;
    pub use crate::game::handles::prelude::*;
    pub use crate::resource_helpers::*;
    pub use crate::settings::prelude::*;
    pub use crate::world_helpers::*;
}

pub use ingame::Ingame;
pub use paused::Paused;
pub use startup::Startup;
