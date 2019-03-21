use super::super::Ingame;
use super::state_prelude::*;

pub struct Startup {
    loading_entity: Option<Entity>,
}

impl Startup {
    pub fn new() -> Self {
        Self {
            loading_entity: None,
        }
    }

    fn is_finished_loading(&self, data: &StateData<CustomGameData>) -> bool {
        // Finished loading spritesheet?
        let spritesheet_asset =
            data.world.read_resource::<AssetStorage<SpriteSheet>>();
        let spritesheet_handle =
            data.world.read_resource::<SpriteSheetHandle>();
        spritesheet_asset.get(&spritesheet_handle).is_some()
    }

    fn initialize_loading_text(
        &mut self,
        data: &mut StateData<CustomGameData>,
    ) {
        const FONT_SIZE: f32 = 50.0;
        const LOADING_TEXT: &str = "Loading...";

        let world = &mut data.world;

        let screen_size =
            data.data.display_config.dimensions.unwrap_or((1200, 800));

        let font = world.read_resource::<Loader>().load(
            resource("fonts/square.ttf"),
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        let transform = new_ui_transform(
            "loading",
            Anchor::Middle,
            (0.0, 0.0, 0.0, screen_size.0 as f32, screen_size.1 as f32, 0),
        );

        self.loading_entity = Some(
            world
                .create_entity()
                .with(transform)
                .with(UiText::new(
                    font,
                    LOADING_TEXT.to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    FONT_SIZE,
                ))
                .build(),
        );
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        // Loading font
        self.initialize_loading_text(&mut data);

        // Spritesheet
        let spritesheet_handle = load_spritesheet(&mut data.world);
        data.world.add_resource(spritesheet_handle);

        // Settings RON
        let settings = load_settings();
        dbg!(&settings);
        data.world.add_resource(settings);
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event)
                || data.world.input().action_is_down("quit").unwrap_or(false)
            {
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

        if self.is_finished_loading(&data) {
            // Remove loading text
            if let Some(entity) = self.loading_entity {
                data.world
                    .delete_entity(entity)
                    .expect("Should delete loading text entity");
            }
            // Switch state
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

fn load_settings() -> Settings {
    let settings_raw = read_file(resource("config/settings.ron"))
        .expect("Couldn't read settings.ron file");
    ron::Value::from_str(&settings_raw)
        .unwrap()
        .into_rust()
        .unwrap()
}

/// `UiTransform::new` wrapper
fn new_ui_transform<T: ToString>(
    name: T,
    anchor: Anchor,
    pos: (f32, f32, f32, f32, f32, i32),
) -> UiTransform {
    UiTransform::new(
        name.to_string(),
        anchor,
        pos.0, // x
        pos.1, // y
        pos.2, // z
        pos.3, // width
        pos.4, // height
        pos.5, // tab-order (?)
    )
}
