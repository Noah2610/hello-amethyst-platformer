// TODO: UNUSED, CLEANUP!

use super::system_prelude::*;

pub struct PauseSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl<'a> System<'a> for PauseSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, mut events: Self::SystemData) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| events.register_reader());

        for event in events.read(reader_id) {
            dbg!(event);
        }
    }
}

impl Default for PauseSystem {
    fn default() -> Self {
        Self { reader_id: None }
    }
}
