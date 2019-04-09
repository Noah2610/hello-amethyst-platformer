use amethyst::renderer::SpriteSheetHandle;

use crate::resource_helpers::*;

pub mod prelude {
    pub use super::SpriteSheetHandles;
    pub use super::SpriteSheetName;
}

#[derive(Clone, Copy)]
pub enum SpriteSheetName {
    Base,
    Player,
}

impl SpriteSheetName {
    fn base_name(&self) -> &str {
        use SpriteSheetName::*;
        match self {
            Base => "spritesheet_base",
            Player => "spritesheet_player",
        }
    }

    pub fn filepath_png(&self) -> String {
        resource(format!("textures/{}.png", self.base_name()))
    }

    pub fn filepath_ron(&self) -> String {
        resource(format!("textures/{}.ron", self.base_name()))
    }
}

pub struct SpriteSheetHandles {
    pub base:   Option<SpriteSheetHandle>,
    pub player: Option<SpriteSheetHandle>,
}

impl SpriteSheetHandles {
    pub fn insert(&mut self, name: SpriteSheetName, handle: SpriteSheetHandle) {
        use SpriteSheetName::*;
        match name {
            Base => self.base = Some(handle),
            Player => self.player = Some(handle),
        }
    }
}

impl Default for SpriteSheetHandles {
    fn default() -> Self {
        Self {
            base:   None,
            player: None,
        }
    }
}
