use amethyst::ecs::{Entities, Join, ReadStorage, Write};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{UiEvent, UiEventType, UiTransform};

use super::state_prelude::*;

pub struct Paused {
    ui_entities:  Vec<Entity>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
}

impl<'a, 'b> Paused {
    fn handle_keys(
        &self,
        data: &StateData<CustomGameData<DisplayConfig>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent>> {
        let input = data.world.input_manager();
        if input.is_up("quit") {
            Some(Trans::Quit)
        } else if input.is_down("pause") {
            Some(Trans::Pop)
        } else {
            None
        }
    }

    fn create_ui(
        &mut self,
        data: &mut StateData<CustomGameData<DisplayConfig>>,
    ) {
        self.ui_entities
            .push(data.world.exec(|mut creator: UiCreator| {
                let entity =
                    creator.create(resource("ui/pause_button.ron"), ());
                entity
            }));
    }

    fn delete_ui(
        &mut self,
        data: &mut StateData<CustomGameData<DisplayConfig>>,
    ) {
        data.world.delete_entities(&self.ui_entities).unwrap();
        self.ui_entities.clear();
    }

    fn handle_ui_events(
        &mut self,
        data: &mut StateData<CustomGameData<DisplayConfig>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent>> {
        data.world.exec(
            |(entities, mut events, ui_transforms): (
                Entities,
                Write<EventChannel<UiEvent>>,
                ReadStorage<UiTransform>,
            )| {
                if let Some(target_id) = (&entities, &ui_transforms)
                    .join()
                    .find_map(|(entity, ui_transform)| {
                        if ui_transform.id == "pause_button" {
                            Some(entity.id())
                        } else {
                            None
                        }
                    })
                {
                    let reader_id = self
                        .ui_reader_id
                        .get_or_insert_with(|| events.register_reader());
                    for event in events.read(reader_id) {
                        if event.event_type == UiEventType::ClickStop
                            && event.target.id() == target_id
                        {
                            return Some(Trans::Pop);
                        }
                    }
                }

                None
            },
        )
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, DisplayConfig>, StateEvent>
    for Paused
{
    fn on_start(&mut self, mut data: StateData<CustomGameData<DisplayConfig>>) {
        // Create paused UI
        self.create_ui(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<DisplayConfig>>) {
        // Delete paused UI
        self.delete_ui(&mut data);
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
        data.data.update(&data.world, "paused").unwrap();
        if let Some(trans) = self.handle_keys(&data) {
            return trans;
        }
        Trans::None
    }

    fn fixed_update(
        &mut self,
        mut data: StateData<CustomGameData<DisplayConfig>>,
    ) -> Trans<CustomGameData<'a, 'b, DisplayConfig>, StateEvent> {
        if let Some(trans) = self.handle_ui_events(&mut data) {
            return trans;
        }
        Trans::None
    }
}

impl Default for Paused {
    fn default() -> Self {
        Self {
            ui_entities:  Vec::new(),
            ui_reader_id: None,
        }
    }
}
