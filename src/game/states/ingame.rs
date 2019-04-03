use super::state_prelude::*;
use super::Paused;
use crate::components::prelude::*;

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
                0.0,                  // Left
                settings.view_size.0, // Right
                0.0,                  // Bottom (!)
                settings.view_size.1, // Top    (!)
            )))
            .with(transform)
            .build();
    }

    fn initialize_player(&self, data: &mut StateData<CustomGameData>) {
        let settings = data.world.settings();

        let mut transform = Transform::default();
        transform.set_xyz(
            settings.view_size.0 * 0.5,
            settings.view_size.1 * 0.5,
            0.0,
        );

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
            .with(Size::from(settings.player_size))
            .with(ScaleOnce)
            .with(Gravity::from(settings.player_gravity))
            .with(Solid)
            .with(Collision::new())
            .build();
    }

    fn initialize_platforms(&self, data: &mut StateData<CustomGameData>) {
        let settings = data.world.settings();

        let mut transform_one = Transform::default();
        transform_one.set_xyz(
            settings.view_size.0 * 0.5,
            settings.view_size.1 * 0.5 - 100.0,
            0.0,
        );
        let mut transform_two = Transform::default();
        transform_two.set_xyz(
            settings.view_size.0 * 0.7,
            settings.view_size.1 * 0.5 + 25.0,
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

        for _ in 0 .. 100 {
            data.world
                .create_entity()
                .with(transform_two.clone())
                .with(Size::from(size_two))
                .with(ScaleOnce)
                .with(Solid)
                // .with(sprite_render.clone())
                .with(Collision::new())
                .build();
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        self.register_components(&mut data.world);

        self.initialize_camera(&mut data.world);
        self.initialize_platforms(&mut data);
        self.initialize_player(&mut data);
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
