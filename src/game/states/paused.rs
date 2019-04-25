use super::state_prelude::*;

pub struct Paused;

impl<'a, 'b> State<CustomGameData<'a, 'b, DisplayConfig>, StateEvent>
    for Paused
{
    fn on_start(&mut self, data: StateData<CustomGameData<DisplayConfig>>) {
        // Create paused UI
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
                println!("UNpause");
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
        data: StateData<CustomGameData<DisplayConfig>>,
    ) -> Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent> {
        data.data.update(&data.world, "paused");
        Trans::None
    }
}
