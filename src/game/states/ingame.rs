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
    }

    fn initialize_player(&self, data: &mut StateData<CustomGameData>) {
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
            .with(sprite_render)
            .build();
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        self.register_components(&mut data.world);

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
