use super::state_prelude::*;
use super::Paused;
use crate::components::prelude::*;

pub struct Ingame;

impl Ingame {
    /// Register components (can be removed once systems using the components are in place)
    fn register_components(&self, world: &mut World) {
        world.register::<Player>();
        world.register::<Size>();
        world.register::<Velocity>();
        world.register::<Scale>();
    }

    fn initialize_camera(&self, world: &mut World) {
        use constants::VIEW_DIMENSIONS;
        let mut transform = Transform::default();
        transform.set_z(1.0);
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0,               // Left
                VIEW_DIMENSIONS.0, // Right
                0.0,               // Bottom (!)
                VIEW_DIMENSIONS.1, // Top    (!)
            )))
            .with(transform)
            .build();
    }

    fn initialize_player(&self, data: &mut StateData<CustomGameData>) {
        let mut transform = Transform::default();
        transform.set_xyz(0.0, 0.0, 0.0);

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
            .with(Player)
            .with(transform)
            .with(sprite_render)
            .with(Velocity::new(4.0, 0.0))
            .with(Size::from(constants::PLAYER_SIZE))
            .with(Scale)
            .build();
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        self.register_components(&mut data.world);

        self.initialize_camera(&mut data.world);
        self.initialize_player(&mut data);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, keys::QUIT) {
                Trans::Quit
            } else if is_key_down(&event, keys::PAUSE) {
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
