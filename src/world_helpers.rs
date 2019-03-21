use amethyst::ecs::World;
use amethyst::input::InputHandler;
use amethyst::shred::Fetch;

use crate::settings::Settings;

pub trait WorldHelpers {
    fn settings(&self) -> Settings;
    fn input(&self) -> Fetch<InputHandler<String, String>>;
}

impl WorldHelpers for World {
    fn settings(&self) -> Settings {
        self.read_resource::<Settings>().clone()
    }

    fn input(&self) -> Fetch<InputHandler<String, String>> {
        self.read_resource::<InputHandler<String, String>>()
    }
}
