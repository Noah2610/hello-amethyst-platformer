use amethyst::audio::AudioSink;

use super::state_prelude::*;
use super::Ingame;
use crate::components::prelude::*;
use map_loader::MapLoader;

mod map_loader;

mod constants {
    pub const CAMERA_Z: f32 = 10.0;
    pub const BACKGROUND_Z: f32 = -1.0;
    pub const FOREGROUND_Z: f32 = 0.0;
    pub const FORE_FOREGROUND_Z: f32 = 0.5;
    pub const PROPERTY_Z_KEY: &str = "z";
    // pub const TILE_SIZE: (f32, f32) = (32.0, 32.0); // TODO: Read this data from tileset JSON file
    pub const TILE_SIZE: (f32, f32) = (16.0, 16.0); // TODO: Read this data from tileset JSON file

}

pub struct Startup {
    loading_entity: Option<Entity>,
    map_loader:     MapLoader,
}

impl Startup {
    pub fn new() -> Self {
        Self {
            loading_entity: None,
            map_loader:     MapLoader::new(),
        }
    }

    fn is_finished_loading(
        &self,
        data: &StateData<CustomGameData<DisplayConfig>>,
    ) -> bool {
        let spritesheet_handles =
            data.world.read_resource::<SpriteSheetHandles>();
        let texture_handles = data.world.read_resource::<TextureHandles>();
        let audio_handles = data.world.read_resource::<AudioHandles>();

        spritesheet_handles.has_finished_loading_all(&data.world)
            && texture_handles.has_finished_loading_all(&data.world)
            && audio_handles.has_finished_loading_all(&data.world)
            && self.map_loader.is_finished()
    }

    /// Register components (can be removed once systems using the components are in place)
    fn register_components(&self, world: &mut World) {
        world.register::<Transparent>();
        world.register::<Solid>();
        world.register::<Collision>();
    }

    fn initialize_loading_text(
        &mut self,
        data: &mut StateData<CustomGameData<DisplayConfig>>,
    ) {
        const FONT_SIZE: f32 = 50.0;
        const LOADING_TEXT: &str = "Loading...";

        let world = &mut data.world;

        let screen_size = data
            .data
            .custom
            .clone()
            .unwrap()
            .dimensions
            .unwrap_or((1200, 800));

        let font = world.read_resource::<Loader>().load(
            resource("fonts/square.ttf"),
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        let transform = new_ui_transform(
            "loading",
            AmethystAnchor::Middle,
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

    fn initialize_audio(
        &self,
        data: &mut StateData<CustomGameData<DisplayConfig>>,
    ) {
        {
            let mut sink = data.world.write_resource::<AudioSink>();
            sink.set_volume(0.5);
        }
        let mut audio_handles = AudioHandles::default();
        audio_handles.load(resource("audio/music/music.ogg"), &mut data.world);
        audio_handles
            .load(resource("audio/sfx/player_jump.ogg"), &mut data.world);

        data.world.add_resource(audio_handles);
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, DisplayConfig>, StateEvent>
    for Startup
{
    fn on_start(&mut self, mut data: StateData<CustomGameData<DisplayConfig>>) {
        // Register components
        self.register_components(&mut data.world);

        // Loading font
        self.initialize_loading_text(&mut data);

        // Audio
        self.initialize_audio(&mut data);

        // Update manually once, so the "Loading" text is displayed
        data.data.update(&data.world, "startup").unwrap();

        // Spritesheets and Textures
        data.world.add_resource(SpriteSheetHandles::default());
        data.world.add_resource(TextureHandles::default());

        // Settings RON
        let settings = load_settings();
        data.world.add_resource(settings);

        // Load map
        self.map_loader.load_map("map.json");
        self.map_loader.build(&mut data);
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData<DisplayConfig>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
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
        data: StateData<CustomGameData<DisplayConfig>>,
    ) -> Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent> {
        data.data.update(&data.world, "startup").unwrap();

        if self.is_finished_loading(&data) {
            // Create new Ingame state first
            let ingame = Box::new(Ingame);
            // Remove loading text
            if let Some(entity) = self.loading_entity {
                data.world
                    .delete_entity(entity)
                    .expect("Should delete loading text entity");
            }
            // Switch state
            return Trans::Switch(ingame);
        }

        Trans::None
    }
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
    anchor: AmethystAnchor,
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
