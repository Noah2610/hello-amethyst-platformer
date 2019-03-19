use super::state_prelude::*;
use super::Paused;

pub struct Ingame;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<CustomGameData>) {

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
