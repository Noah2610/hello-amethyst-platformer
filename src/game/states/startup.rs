use super::super::Ingame;
use super::state_prelude::*;
use crate::components::prelude::*;
use crate::geo::Vector;

pub struct Startup {
    loading_entity: Option<Entity>,
    loaded_map:     bool,
    loaded_camera:  bool,
}

impl Startup {
    pub fn new() -> Self {
        Self {
            loading_entity: None,
            loaded_map:     false,
            loaded_camera:  false,
        }
    }

    fn is_finished_loading(&self, data: &StateData<CustomGameData>) -> bool {
        // Finished loading spritesheet(s)?
        let spritesheet_asset =
            data.world.read_resource::<AssetStorage<SpriteSheet>>();
        let spritesheet_handles =
            data.world.read_resource::<SpriteSheetHandles>();

        let has_base_handle =
            if let Some(base_handle) = &spritesheet_handles.base {
                spritesheet_asset.get(base_handle).is_some()
            } else {
                false
            };
        let has_player_handle =
            if let Some(player_handle) = &spritesheet_handles.player {
                spritesheet_asset.get(player_handle).is_some()
            } else {
                false
            };

        has_base_handle
            && has_player_handle
            && self.loaded_map
            && self.loaded_camera
    }

    /// Register components (can be removed once systems using the components are in place)
    fn register_components(&self, world: &mut World) {
        world.register::<Transparent>();
        world.register::<Solid>();
        world.register::<Collision>();
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

    fn initialize_camera(&mut self, world: &mut World) {
        let settings = world.read_resource::<Settings>().clone();

        let mut transform = Transform::default();
        transform.set_z(1.0);

        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0,                    // Left
                settings.camera_size.0, // Right
                0.0,                    // Bottom (!)
                settings.camera_size.1, // Top    (!)
            )))
            .with(transform)
            .with(Size::from(settings.camera_size))
            .with(InnerSize(Size::from(settings.camera_inner_size)))
            .with(Velocity::default())
            .with(Collision::new())
            .build();

        self.loaded_camera = true;
    }

    fn load_map(&mut self, data: &mut StateData<CustomGameData>) {
        use std::fs::File;
        use std::io::prelude::*;

        use crate::components;

        let map_filepath = resource("map.json");
        let mut file = File::open(map_filepath)
            .expect("Should open file for reading: map.json");
        let mut json_raw = String::new();
        file.read_to_string(&mut json_raw)
            .expect("Should read file content: map.json");
        let json = json::parse(&json_raw).expect("Could not parse JSON");

        const TILE_SIZE: (f32, f32) = (32.0, 32.0); // TODO: Read this data from tileset JSON file

        // OBJECTS
        for object_data in json["objects"].members() {
            if let (Some(obj_type), (Some(x), Some(y)), (Some(w), Some(h))) = (
                object_data["type"].as_str(),
                (
                    object_data["pos"]["x"].as_f32(),
                    object_data["pos"]["y"].as_f32(),
                ),
                (
                    object_data["size"]["w"].as_f32(),
                    object_data["size"]["h"].as_f32(),
                ),
            ) {
                match obj_type {
                    "Player" => {
                        self.initialize_player_with(data, (x, y), (w, h))
                    }
                    _ => (),
                }
            }
        }

        // TILES
        for tile_data in json["tiles"].members() {
            if let (Some(id), (Some(x), Some(y)), component_names) = (
                tile_data["id"].as_usize(),
                (
                    tile_data["pos"]["x"].as_f32(),
                    tile_data["pos"]["y"].as_f32(),
                ),
                tile_data["properties"]["components"].members(),
            ) {
                let mut pos = Transform::default();
                pos.set_xyz(x, y, 0.0);

                let sprite_render = {
                    let spritesheet_handle = data
                        .world
                        .read_resource::<SpriteSheetHandles>()
                        .base
                        .clone()
                        .expect(
                            "Base SpriteSheet should be loaded at this point",
                        );
                    SpriteRender {
                        sprite_sheet:  spritesheet_handle,
                        sprite_number: id,
                    }
                };

                let mut entity = data
                    .world
                    .create_entity()
                    .with(pos)
                    .with(Size::from(TILE_SIZE))
                    .with(ScaleOnce)
                    .with(sprite_render);

                for component_name in component_names {
                    entity = components::add_component_by_name(
                        entity,
                        component_name
                            .as_str()
                            .expect("Could not parse string JSON"),
                    );
                }

                entity.build();
            }
        }

        self.loaded_map = true;
    }

    fn initialize_player_with(
        &self,
        data: &mut StateData<CustomGameData>,
        pos: Vector,
        size: Vector,
    ) {
        let settings = data.world.settings();

        let mut transform = Transform::default();
        transform.set_xyz(pos.0, pos.1, 0.0);
        // transform.set_xyz(0.0, 0.0, 0.0);
        // let size = Size::from(settings.player_size);
        let size = Size::from(size);

        let sprite_render = {
            let spritesheet_handle = data
                .world
                .read_resource::<SpriteSheetHandles>()
                .player
                .clone()
                .expect("Player SpriteSheet should be loaded at this point");
            SpriteRender {
                sprite_sheet:  spritesheet_handle,
                sprite_number: 0,
            }
        };

        data.world
            .create_entity()
            .with(Player::with_speed(settings.player_speed))
            .with(transform)
            .with(sprite_render)
            .with(Transparent)
            .with(Velocity::default())
            .with(MaxVelocity::from(settings.player_max_velocity))
            .with(DecreaseVelocity::from(settings.player_decr_velocity))
            .with(size)
            .with(ScaleOnce)
            .with(Gravity::from(settings.player_gravity))
            .with(Solid)
            .with(Collision::new())
            .with(CheckCollision)
            .build();
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        // Register components
        self.register_components(&mut data.world);

        // Loading font
        self.initialize_loading_text(&mut data);

        // Spritesheet(s)
        load_spritesheets(&mut data.world);
        // let spritesheet_handle = load_spritesheet(&mut data.world);
        // data.world.add_resource(spritesheet_handle);

        // Update manually once, so the "Loading" text is displayed
        data.data.update(&data.world, GameState::Startup);

        // Settings RON
        let settings = load_settings();
        data.world.add_resource(settings);

        // Initialize entities
        self.load_map(&mut data);
        self.initialize_camera(&mut data.world);
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

fn load_spritesheets(world: &mut World) {
    const SPRITESHEET_NAMES: [SpriteSheetName; 2] =
        [SpriteSheetName::Base, SpriteSheetName::Player];

    let mut handles = SpriteSheetHandles::default();

    for &name in &SPRITESHEET_NAMES {
        let handle = {
            let loader = world.read_resource::<Loader>();
            let texture_handle = {
                let texture_storage =
                    world.read_resource::<AssetStorage<Texture>>();
                loader.load(
                    name.filepath_png(),
                    PngFormat,
                    TextureMetadata::srgb_scale(),
                    (),
                    &texture_storage,
                )
            };
            let spritesheet_store =
                world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                name.filepath_ron(),
                SpriteSheetFormat,
                texture_handle,
                (),
                &spritesheet_store,
            )
        };

        handles.insert(name, handle);
    }

    world.add_resource(handles);
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
