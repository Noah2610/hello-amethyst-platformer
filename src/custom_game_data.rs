use amethyst::core::{ArcThreadPool, SystemBundle};
use amethyst::ecs::{Dispatcher, DispatcherBuilder, System};
use amethyst::prelude::*;
use amethyst::{DataInit, Error, Result};

pub mod prelude {
    pub use super::CustomGameData;
    pub use super::CustomGameDataBuilder;
    pub use super::GameState;
}

#[derive(Clone, Copy)]
pub enum GameState {
    Startup,
    Ingame,
    Paused,
}

pub struct CustomGameData<'a, 'b> {
    core_dispatcher:   Dispatcher<'a, 'b>,
    ingame_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    pub fn update(&mut self, world: &World, state: GameState) {
        use GameState::*;

        if let Startup = state {
            // self.startup_dispatcher.dispatch(&world.res);
        }
        if let Ingame = state {
            self.ingame_dispatcher.dispatch(&world.res);
        }
        if let Paused = state {
            // self.paused_dispatcher.dispatch(&world.res);
        }
        self.core_dispatcher.dispatch(&world.res);
    }
}

pub struct CustomGameDataBuilder<'a, 'b> {
    pub core:   DispatcherBuilder<'a, 'b>,
    pub ingame: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        Self {
            core:   DispatcherBuilder::new(),
            ingame: DispatcherBuilder::new(),
        }
    }

    pub fn with_base_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle
            .build(&mut self.core)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn with_ingame<S>(
        mut self,
        system: S,
        name: &str,
        dependencies: &[&str],
    ) -> Self
    where
        for<'c> S: System<'c> + Send + 'a, // NOTE: What the heck is this syntax??? `for<'a>` in a where clause???
    {
        self.ingame.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> Default for CustomGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        CustomGameDataBuilder::new()
    }
}

impl<'a, 'b> DataInit<CustomGameData<'a, 'b>>
    for CustomGameDataBuilder<'a, 'b>
{
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b> {
        // Get handle to the `ThreadPool`
        let pool = world.read_resource::<ArcThreadPool>().clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut ingame_dispatcher = self.ingame.with_pool(pool.clone()).build();
        core_dispatcher.setup(&mut world.res);
        ingame_dispatcher.setup(&mut world.res);

        CustomGameData {
            core_dispatcher,
            ingame_dispatcher,
        }
    }
}
