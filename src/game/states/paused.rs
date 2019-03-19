use super::state_prelude::*;

pub struct Paused;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Paused {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        // Create paused UI
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, keys::QUIT) {
                Trans::Quit
            } else if is_key_down(&event, keys::PAUSE) {
                // Remove paused UI
                Trans::Pop
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
        data.data.update(&data.world, GameState::Paused);
        Trans::None
    }
}
