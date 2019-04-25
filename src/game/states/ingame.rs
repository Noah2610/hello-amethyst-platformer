use super::state_prelude::*;
use super::Paused;
use crate::components::prelude::*;
use crate::geo::Vector;

pub struct Ingame;

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
            let input = data.world.input();
            if is_close_requested(&event)
                || input.action_is_down("quit").unwrap_or(false)
            {
                Trans::Quit
            } else if input.action_is_down("pause").unwrap_or(false) {
                println!("PAUSE");
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
        data: StateData<CustomGameData<DisplayConfig>>,
    ) -> Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent> {
        data.data.update(&data.world, "ingame");
        Trans::None
    }
}
