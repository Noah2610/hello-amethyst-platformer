use super::state_prelude::*;
use super::Paused;
use crate::components::prelude::*;
use crate::geo::Vector;

pub struct Ingame;

impl Ingame {
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
        // self.register_components(&mut data.world);
        // self.initialize_platforms(&mut data);
        // self.initialize_player(&mut data);
        // self.load_map(&mut data);
        // self.initialize_camera(&mut data.world);
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
