use super::state_prelude::*;
use super::Paused;
use crate::components::prelude::*;
use crate::geo::Vector;

pub struct Ingame;

impl Ingame {
    /// Register components (can be removed once systems using the components are in place)
    fn register_components(&self, world: &mut World) {
        world.register::<Transparent>();
        world.register::<Solid>();
        world.register::<Collision>();
    }

    fn initialize_camera(&self, world: &mut World) {
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
    }

    fn load_map(&self, data: &mut StateData<CustomGameData>) {
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
                    let spritesheet_handle =
                        data.world.read_resource::<SpriteSheetHandle>();
                    SpriteRender {
                        sprite_sheet:  spritesheet_handle.clone(),
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
        // let size = Size::from(settings.player_size);
        let size = Size::from(size);

        let sprite_render = {
            let spritesheet_handle =
                data.world.read_resource::<SpriteSheetHandle>();
            SpriteRender {
                sprite_sheet:  spritesheet_handle.clone(),
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

    fn initialize_platforms(&self, data: &mut StateData<CustomGameData>) {
        let settings = data.world.settings();

        let mut transform_one = Transform::default();
        transform_one.set_xyz(
            settings.camera_size.0 * 0.5,
            settings.camera_size.1 * 0.5 - 100.0,
            0.0,
        );
        let mut transform_two = Transform::default();
        transform_two.set_xyz(
            settings.camera_size.0 * 0.7,
            settings.camera_size.1 * 0.5 + 25.0,
            0.0,
        );
        let size_one = (200.0, 64.0);
        let size_two = (64.0, 200.0);

        let sprite_render = {
            let spritesheet_handle =
                data.world.read_resource::<SpriteSheetHandle>();
            SpriteRender {
                sprite_sheet:  spritesheet_handle.clone(),
                sprite_number: 1,
            }
        };

        data.world
            .create_entity()
            .with(transform_one)
            .with(Size::from(size_one))
            .with(ScaleOnce)
            .with(Solid)
            .with(sprite_render.clone())
            .with(Collision::new())
            .build();

        data.world
            .create_entity()
            .with(transform_two.clone())
            .with(Size::from(size_two))
            .with(ScaleOnce)
            .with(Solid)
            .with(sprite_render.clone())
            .with(Collision::new())
            .build();

        let mut transform = Transform::default();
        transform.set_xyz(100.0, 200.0, 0.0);
        data.world
            .create_entity()
            .with(transform)
            .with(Size::from(size_two))
            .with(ScaleOnce)
            .with(Solid)
            .with(sprite_render.clone())
            .with(Collision::new())
            .build();

        let mut transform = Transform::default();
        transform.set_xyz(settings.camera_size.0, 0.0, 0.0);
        data.world
            .create_entity()
            .with(transform)
            .with(Size::new(3200.0, 200.0))
            .with(ScaleOnce)
            .with(Solid)
            .with(sprite_render.clone())
            .with(Collision::new())
            .build();

        let mut transform = Transform::default();
        transform.set_xyz(settings.camera_size.0 * 2.0, 0.0, 0.0);
        data.world
            .create_entity()
            .with(transform)
            .with(Size::new(70.0, 1600.0))
            .with(ScaleOnce)
            .with(Solid)
            .with(sprite_render.clone())
            .with(Collision::new())
            .build();

        let mut transform = Transform::default();
        transform.set_xyz(settings.camera_size.0 * 2.0 + 100.0, 0.0, 0.0);
        data.world
            .create_entity()
            .with(transform)
            .with(Size::new(50.0, 1600.0))
            .with(ScaleOnce)
            .with(Solid)
            .with(sprite_render.clone())
            .with(Collision::new())
            .build();

        // for _ in 0 .. 100 {
        //     data.world
        //         .create_entity()
        //         .with(transform_two.clone())
        //         .with(Size::from(size_two))
        //         .with(ScaleOnce)
        //         .with(Solid)
        //         // .with(sprite_render.clone())
        //         .with(Collision::new())
        //         .build();
        // }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        self.register_components(&mut data.world);

        self.initialize_camera(&mut data.world);
        // self.initialize_platforms(&mut data);
        // self.initialize_player(&mut data);
        self.load_map(&mut data);
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            let input = data.world.input();
            if is_close_requested(&event)
                || input.action_is_down("quit").unwrap_or(false)
            {
                Trans::Quit
            } else if input.action_is_down("pause").unwrap_or(false) {
                Trans::Push(Box::new(Paused))
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
        data.data.update(&data.world, GameState::Ingame);
        Trans::None
    }
}
