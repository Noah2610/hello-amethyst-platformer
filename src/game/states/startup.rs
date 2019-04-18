use amethyst::ecs::world::Index;
use json::JsonValue;

use super::super::Ingame;
use super::state_prelude::*;
use crate::components::prelude::*;
use deathframe::geo::{Anchor, Vector};

const CAMERA_Z: f32 = 10.0;
const BACKGROUND_Z: f32 = -1.0;
const FOREGROUND_Z: f32 = 0.0;
const FORE_FOREGROUND_Z: f32 = 0.5;
const PROPERTY_Z_KEY: &str = "z";

pub struct Startup {
    player_id:      Option<Index>,
    camera_id:      Option<Index>,
    loading_entity: Option<Entity>,
    parallax_data:  Vec<(Vector, Vector, JsonValue)>,
    loaded_map:     bool,
    loaded_camera:  bool,
}

impl Startup {
    pub fn new() -> Self {
        Self {
            player_id:      None,
            camera_id:      None,
            loading_entity: None,
            parallax_data:  Vec::new(),
            loaded_map:     false,
            loaded_camera:  false,
        }
    }

    fn is_finished_loading(&self, data: &StateData<CustomGameData>) -> bool {
        let spritesheet_handles =
            data.world.read_resource::<SpriteSheetHandles>();
        let texture_handles = data.world.read_resource::<TextureHandles>();

        spritesheet_handles.has_finished_loading_all(&data.world)
            && texture_handles.has_finished_loading_all(&data.world)
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

    fn initialize_camera(&mut self, world: &mut World) {
        let settings = world.read_resource::<Settings>().clone();

        let mut transform = Transform::default();
        transform.set_z(CAMERA_Z);

        let mut camera = Camera::new()
            .base_speed({ settings.camera.base_speed })
            .deadzone({ settings.camera.deadzone });

        if let Some(player_id) = self.player_id {
            camera = camera.follow(player_id);
        }

        let entity = world
            .create_entity()
            .with(AmethystCamera::from(Projection::orthographic(
                0.0,                    // Left
                settings.camera.size.0, // Right
                0.0,                    // Bottom (!)
                settings.camera.size.1, // Top    (!)
            )))
            .with(camera.build())
            .with(transform)
            .with(Size::from(settings.camera.size))
            .with(InnerSize(Size::from(settings.camera.inner_size)))
            .with(Velocity::default())
            .with(Collision::new())
            .build();

        self.camera_id = Some(entity.id());
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
            if let (
                Some(obj_type),
                (Some(x), Some(y)),
                (Some(w), Some(h)),
                properties,
            ) = (
                object_data["type"].as_str(),
                (
                    object_data["pos"]["x"].as_f32(),
                    object_data["pos"]["y"].as_f32(),
                ),
                (
                    object_data["size"]["w"].as_f32(),
                    object_data["size"]["h"].as_f32(),
                ),
                &object_data["properties"],
            ) {
                match obj_type {
                    "Player" => self.initialize_player_with(
                        data,
                        (x, y).into(),
                        (w, h).into(),
                        properties.clone(),
                    ),
                    "Parallax" => self.parallax_data.push((
                        (x, y).into(),
                        (w, h).into(),
                        properties.clone(),
                    )),
                    _ => (),
                }
            }
        }

        // TILES
        for tile_data in json["tiles"].members() {
            if let (
                Some(id),
                (Some(x), Some(y)),
                properties,
                Some(tileset_name),
            ) = (
                tile_data["id"].as_usize(),
                (
                    tile_data["pos"]["x"].as_f32(),
                    tile_data["pos"]["y"].as_f32(),
                ),
                &tile_data["properties"],
                tile_data["ts"].as_str(),
            ) {
                let mut pos = Transform::default();
                pos.set_xyz(
                    x,
                    y,
                    properties[PROPERTY_Z_KEY].as_f32().unwrap_or(FOREGROUND_Z),
                );

                let spritesheet_path =
                    resource(format!("textures/{}.png", tileset_name));
                let sprite_render = {
                    let spritesheet_handle = data
                        .world
                        .write_resource::<SpriteSheetHandles>()
                        .get_or_load(spritesheet_path, &data.world);
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
                    .with(sprite_render)
                    .with(Transparent);

                for component_name in properties["components"].members() {
                    let component_name_str = component_name
                        .as_str()
                        .expect("Could not parse string JSON");
                    entity = components::add_component_to_entity_by_name(
                        entity,
                        component_name_str,
                    );
                    entity = components::add_component_to_entity_by_name_custom(
                        entity,
                        component_name_str,
                    );
                }

                entity.build();
            }
        }

        self.loaded_map = true;
    }

    fn initialize_player_with(
        &mut self,
        data: &mut StateData<CustomGameData>,
        pos: Vector,
        size: Vector,
        properties: JsonValue,
    ) {
        let settings = data.world.settings();

        let mut transform = Transform::default();
        transform.set_xyz(
            pos.0,
            pos.1,
            properties[PROPERTY_Z_KEY]
                .as_f32()
                .unwrap_or(FORE_FOREGROUND_Z),
        ); // NOTE: Draw player above foreground elements
        let size = Size::from(size);

        let spritesheet_path = resource("textures/spritesheet_player.png");
        let sprite_render = {
            let spritesheet_handle = data
                .world
                .write_resource::<SpriteSheetHandles>()
                .get_or_load(spritesheet_path, &data.world);
            SpriteRender {
                sprite_sheet:  spritesheet_handle,
                sprite_number: 0,
            }
        };

        let player = data
            .world
            .create_entity()
            .with(
                Player::new()
                    .acceleration(settings.player.acceleration)
                    .run_acceleration(settings.player.run_acceleration)
                    .max_velocity(settings.player.max_velocity)
                    .run_max_velocity(settings.player.run_max_velocity)
                    .build(),
            )
            .with(transform)
            .with(sprite_render)
            .with(Transparent)
            .with(Velocity::default())
            .with(MaxVelocity::from(settings.player.max_velocity))
            .with(DecreaseVelocity::from(settings.player.decr_velocity))
            .with(size)
            .with(ScaleOnce)
            .with(Gravity::from(settings.player.gravity))
            .with(Solid)
            .with(Collision::new())
            .with(CheckCollision)
            .with(Pushable)
            .build();
        self.player_id = Some(player.id());
    }

    fn initialize_parallax_with(
        &self,
        data: &mut StateData<CustomGameData>,
        pos: Vector,
        size: Vector,
        properties: &JsonValue,
    ) {
        let bg_dir = resource("textures/bg");

        let settings = data.world.settings();

        if let Some(camera_id) = self.camera_id {
            // Load bg image texture
            let texture_handle_opt = if let Some((_, bg_filename)) =
                properties.entries().find(|(key, _)| key == &"image")
            {
                let mut texture_handles =
                    data.world.write_resource::<TextureHandles>();
                let filepath = format!(
                    "{}/{}",
                    bg_dir,
                    bg_filename.as_str().expect(
                        "Couldn't parse background image filename as str"
                    )
                );
                Some(texture_handles.get_or_load(filepath, data.world))
            } else {
                None
            };

            // Create entity
            let mut entity = data.world.create_entity();
            let mut parallax = Parallax::new()
                .follow(camera_id)
                .follow_anchor(Anchor::BottomLeft);

            for (key, val) in properties.entries() {
                match (key, &texture_handle_opt) {
                    ("speed_mult", _) => {
                        parallax = parallax.speed_mult(parse_string_to_vector(
                            val.as_str()
                                .expect("Couldn't parse JsonValue as string"),
                        ));
                    }
                    ("offset", _) => {
                        parallax = parallax.offset(parse_string_to_vector(
                            val.as_str()
                                .expect("Couldn't parse JsonValue as string"),
                        ))
                    }
                    ("image", Some(texture_handle)) => {
                        entity = entity.with(texture_handle.clone())
                    }
                    _ => (),
                }
            }

            // Add transform and size to entity
            let mut transform = Transform::default();
            transform.set_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(BACKGROUND_Z),
            ); // NOTE: Draw parallax backgrounds behind foreground
            entity = entity
                .with(transform)
                .with(Size::from(size))
                .with(Velocity::default())
                .with(ScaleOnce)
                .with(Transparent)
                .with(parallax.build());

            entity.build();
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        // Register components
        self.register_components(&mut data.world);

        // Loading font
        self.initialize_loading_text(&mut data);

        // Spritesheets and Textures
        data.world.add_resource(SpriteSheetHandles::default());
        data.world.add_resource(TextureHandles::default());

        // Update manually once, so the "Loading" text is displayed
        data.data.update(&data.world, GameState::Startup);

        // Settings RON
        let settings = load_settings();
        data.world.add_resource(settings);

        // Initialize entities
        self.load_map(&mut data);
        self.initialize_camera(&mut data.world);
        // Now initialize parallax backgrounds
        for pdata in &self.parallax_data {
            self.initialize_parallax_with(
                &mut data, pdata.0, pdata.1, &pdata.2,
            );
        }
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

fn parse_string_to_vector<T>(string: T) -> Vector
where
    T: ToString,
{
    let string = string.to_string();
    let vec = string
        .split(",")
        .map(|s| {
            s.trim()
                .parse::<f32>()
                .expect(&format!("Couldn't parse string as f32: '{:?}'", s))
        })
        .collect::<Vec<f32>>();
    if vec.len() == 2 {
        (vec[0], vec[1]).into()
    } else {
        panic!(format!(
            "Given string does not have exactly two fields for Vector (x, y): \
             '{:?}'",
            string
        ));
    }
}
