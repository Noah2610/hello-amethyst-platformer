use std::collections::HashMap;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::World;
use amethyst::renderer::{
    PngFormat,
    SpriteSheet,
    SpriteSheetFormat,
    SpriteSheetHandle,
    Texture,
    TextureMetadata,
};

use crate::resource_helpers::*;

pub mod prelude {
    pub use super::SpriteSheetHandles;
}

pub struct SpriteSheetHandles {
    spritesheet_handles: HashMap<String, SpriteSheetHandle>,
}

impl SpriteSheetHandles {
    pub fn insert<T: ToString>(&mut self, name: T, handle: SpriteSheetHandle) {
        self.spritesheet_handles.insert(name.to_string(), handle);
    }

    pub fn get<T: ToString>(&self, name: T) -> Option<SpriteSheetHandle> {
        self.spritesheet_handles
            .get(&name.to_string())
            .map(Clone::clone)
    }

    pub fn load<T: ToString>(&mut self, name: T, world: &World) {
        let name = name.to_string().replace(".png", "").replace(".ron", "");
        let filepath_png = resource(format!("textures/{}.png", name));
        let filepath_ron = resource(format!("textures/{}.ron", name));
        let handle = {
            let loader = world.read_resource::<Loader>();
            let texture_handle = {
                let texture_storage =
                    world.read_resource::<AssetStorage<Texture>>();
                loader.load(
                    filepath_png,
                    PngFormat,
                    TextureMetadata::srgb_scale(),
                    (),
                    &texture_storage,
                )
            };
            let spritesheet_store =
                world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                filepath_ron,
                SpriteSheetFormat,
                texture_handle,
                (),
                &spritesheet_store,
            )
        };

        self.insert(name, handle);
    }

    pub fn get_or_load(
        &mut self,
        name: &str,
        world: &World,
    ) -> SpriteSheetHandle {
        if let Some(handle) = self.get(name) {
            handle
        } else {
            self.load(name, world);
            self.get(name).unwrap()
        }
    }

    pub fn has_finished_loading_all(&self, world: &World) -> bool {
        let asset = world.read_resource::<AssetStorage<SpriteSheet>>();
        self.spritesheet_handles
            .values()
            .all(|handle| asset.get(handle).is_some())
    }
}

impl Default for SpriteSheetHandles {
    fn default() -> Self {
        Self {
            spritesheet_handles: HashMap::new(),
        }
    }
}
