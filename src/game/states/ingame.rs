use super::state_prelude::*;
use super::Paused;

pub struct Ingame;

impl<'a, 'b> Ingame {
    fn handle_keys(
        &self,
        data: &StateData<CustomGameData<DisplayConfig>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent>> {
        let input = data.world.input_manager();
        if input.is_up("quit") {
            Some(Trans::Quit)
        } else if input.is_down("pause") {
            Some(Trans::Push(Box::new(Paused::default())))
        } else {
            None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, DisplayConfig>, StateEvent>
    for Ingame
{
    fn on_start(&mut self, data: StateData<CustomGameData<DisplayConfig>>) {
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData<DisplayConfig>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
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
        data: StateData<CustomGameData<DisplayConfig>>,
    ) -> Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent> {
        data.data.update(&data.world, "ingame").unwrap();
        if let Some(trans) = self.handle_keys(&data) {
            return trans;
        }
        Trans::None
    }
}
