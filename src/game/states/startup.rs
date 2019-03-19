use super::super::Ingame;
use super::state_prelude::*;
use super::Paused;

pub struct Startup;

impl Startup {
    fn is_spritesheet_loaded(&self) -> bool {
        false
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        let spritesheet_handle = load_spritesheet(&mut data.world);
        data.world.add_resource(spritesheet_handle);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, keys::QUIT) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, GameState::Startup);

        if self.is_spritesheet_loaded() {
            return Trans::Switch(Box::new(Ingame));
        }

        Trans::None
    }
}

fn load_spritesheet(world: &mut World) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            resource("textures/spritesheet.png"),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let spritesheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        resource("textures/spritesheet.ron"),
        SpriteSheetFormat,
        texture_handle,
        (),
        &spritesheet_store,
    )
}
