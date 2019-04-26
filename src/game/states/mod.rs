mod ingame;
mod paused;
// TODO: Temporary pub.
pub mod startup;

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
        DisplayConfig,
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
        UiCreator,
        UiText,
        UiTransform,
    };
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use deathframe::custom_game_data::prelude::*;
    pub use deathframe::handlers::prelude::*;
    pub use deathframe::input_manager::InputManager;

    pub use crate::game::handlers::prelude::*;
    pub use crate::resource_helpers::*;
    pub use crate::settings::prelude::*;
    pub use crate::world_helpers::*;
}

pub use ingame::Ingame;
pub use paused::Paused;
pub use startup::Startup;
