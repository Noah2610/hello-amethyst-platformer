use std::fs::File;
use std::io::prelude::*;

use amethyst::ecs::world::Index;
use deathframe::geo::{Anchor, Vector};
use json::JsonValue;

use super::super::state_prelude::*;
use super::constants::*;
use crate::components::prelude::*;

struct SpriteData {
    pub spritesheet_path: String,
    pub sprite_id:        usize,
}

struct TextureData {}

enum Graphic {
    Sprite(SpriteData),
    Texture(TextureData),
}

struct EntityData {
    pub pos:        Vector,
    pub size:       Vector,
    pub properties: JsonValue,
    pub graphic:    Option<Graphic>,
}

pub struct MapLoader {
    camera_id:     Option<Index>,
    player_id:     Option<Index>,
    player_data:   Option<EntityData>,
    tiles_data:    Vec<EntityData>,
    parallax_data: Vec<EntityData>,
}

impl MapLoader {
    pub fn new() -> Self {
        Self {
            camera_id:     None,
            player_id:     None,
            player_data:   None,
            tiles_data:    Vec::new(),
            parallax_data: Vec::new(),
        }
    }

    /// Returns `true` if everything has finished loading and building properly.
    pub fn is_finished(&self) -> bool {
        self.player_id.is_some() && self.camera_id.is_some()
    }

    /// Start loading the map data from the given map filename.
    pub fn load_map<T>(&mut self, filename: T)
    where
        T: ToString,
    {
        let map_filepath = resource(&filename.to_string());
        let mut file = File::open(map_filepath)
            .expect("Should open file for reading: map.json");
        let mut json_raw = String::new();
        file.read_to_string(&mut json_raw)
            .expect("Should read file content: map.json");
        let json = json::parse(&json_raw).expect("Could not parse JSON");

        // OBJECTS
        self.load_objects(&json["objects"]);

        // TILES
        self.load_tiles(&json["tiles"]);

        // self.loaded_map = true;
    }

    /// Builds the loaded data using the given `StateData`.
    pub fn build<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        self.build_player(data);
        self.build_camera(data);
        self.build_tiles(data);
        self.build_parallax(data);
    }

    fn load_objects(&mut self, json: &JsonValue) {
        for object_data in json.members() {
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
                    "Player" => {
                        self.player_data = Some(EntityData {
                            pos:        (x, y).into(),
                            size:       (w, h).into(),
                            properties: properties.clone(),
                            graphic:    None,
                        })
                    }
                    "Parallax" => self.parallax_data.push(EntityData {
                        pos:        (x, y).into(),
                        size:       (w, h).into(),
                        properties: properties.clone(),
                        graphic:    None,
                    }),
                    _ => (),
                }
            }
        }
    }

    fn load_tiles(&mut self, json: &JsonValue) {
        for tile_data in json.members() {
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
                let spritesheet_path =
                    resource(format!("textures/{}.png", tileset_name));

                self.tiles_data.push(EntityData {
                    pos:        (x, y).into(),
                    size:       TILE_SIZE.into(), // TODO: Read tile size from json
                    properties: properties.clone(),
                    graphic:    Some(Graphic::Sprite(SpriteData {
                        spritesheet_path: spritesheet_path,
                        sprite_id:        id,
                    })),
                });
            }
        }
    }

    fn build_player<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        if let Some(EntityData {
            pos,
            size,
            properties,
            graphic: _,
        }) = &self.player_data
        {
            let settings = data.world.settings();

            let mut transform = Transform::default();
            transform.set_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY]
                    .as_f32()
                    .unwrap_or(FORE_FOREGROUND_Z),
            ); // NOTE: Draw player above foreground elements
            let size = Size::from(*size);

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
                .with(Push)
                .build();
            self.player_id = Some(player.id());
        }
    }

    fn build_camera<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        let settings = data.world.settings();

        let mut transform = Transform::default();
        transform.set_z(CAMERA_Z);

        let mut camera = Camera::new()
            .base_speed({ settings.camera.base_speed })
            .deadzone({ settings.camera.deadzone });

        if let Some(player_id) = self.player_id {
            camera = camera.follow(player_id);
        }

        let entity = data
            .world
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
    }

    fn build_tiles<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        for EntityData {
            pos,
            size,
            properties,
            graphic,
        } in &self.tiles_data
        {
            let mut transform = Transform::default();
            transform.set_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(FOREGROUND_Z),
            );

            let sprite_render_opt =
                if let Some(Graphic::Sprite(sprite_data)) = graphic {
                    let sprite_render = {
                        let spritesheet_handle = data
                            .world
                            .write_resource::<SpriteSheetHandles>()
                            .get_or_load(
                                &sprite_data.spritesheet_path,
                                &data.world,
                            );
                        SpriteRender {
                            sprite_sheet:  spritesheet_handle,
                            sprite_number: sprite_data.sprite_id,
                        }
                    };
                    Some(sprite_render)
                } else {
                    None
                };

            let mut entity = data
                .world
                .create_entity()
                .with(transform)
                .with(Size::from(*size))
                .with(ScaleOnce)
                .with(Transparent);

            if let Some(sprite_render) = sprite_render_opt {
                entity = entity.with(sprite_render);
            }

            for component_name in properties["components"].members() {
                let component_name_str = component_name
                    .as_str()
                    .expect("Could not parse string JSON");
                entity = crate::components::add_component_to_entity_by_name(
                    entity,
                    component_name_str,
                );
                entity =
                    crate::components::add_component_to_entity_by_name_custom(
                        entity,
                        component_name_str,
                    );
            }

            entity.build();
        }
    }

    fn build_parallax<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        let bg_dir = resource("textures/bg");

        for EntityData {
            pos,
            size,
            properties,
            graphic: _,
        } in &self.parallax_data
        {
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
                            parallax = parallax.speed_mult(
                                parse_string_to_vector(val.as_str().expect(
                                    "Couldn't parse JsonValue as string",
                                )),
                            );
                        }
                        ("offset", _) => {
                            parallax = parallax.offset(parse_string_to_vector(
                                val.as_str().expect(
                                    "Couldn't parse JsonValue as string",
                                ),
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
                    .with(Size::from(*size))
                    .with(Velocity::default())
                    .with(ScaleOnce)
                    .with(Transparent)
                    .with(parallax.build());

                entity.build();
            }
        }
    }
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
