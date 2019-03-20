extern crate amethyst;

mod custom_game_data;
mod game;
mod resource_helpers;

mod components;
mod systems;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig,
    DrawFlat2D,
    Pipeline,
    RenderBundle,
    Stage,
};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::{LogLevelFilter, LoggerConfig};

pub use resource_helpers::*;

use custom_game_data::prelude::*;
use systems::prelude::*;

fn main() -> amethyst::Result<()> {
    start_logger();

    let game_data = build_game_data()?;

    let mut game = Application::new("./", game::Startup::new(), game_data)?;
    game.run();

    Ok(())
}

fn start_logger() {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });
}

fn build_game_data<'a, 'b>() -> amethyst::Result<CustomGameDataBuilder<'a, 'b>>
{
    // Display config
    let display_config = DisplayConfig::load(&resource("config/display.ron"));

    // Pipeline
    let pipeline = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    // Bundles
    let render_bundle =
        RenderBundle::new(pipeline, Some(display_config.clone()))
            .with_sprite_sheet_processor();
    let transform_bundle = TransformBundle::new();
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(&resource("config/bindings.ron"))?;
    let ui_bundle = UiBundle::<String, String>::new();

    // Create GameDataBuilder
    let game_data = CustomGameDataBuilder::default()
        .with_display_config(display_config)
        .with_base_bundle(render_bundle)?
        .with_base_bundle(transform_bundle)?
        .with_base_bundle(input_bundle)?
        .with_base_bundle(ui_bundle)?
        .with_core(ScaleSpritesSystem, "scale_sprites_system", &[])
        .with_ingame(ControlPlayerSystem, "control_player_system", &[])
        .with_ingame(MoveEntitiesSystem, "move_entities_system", &[
            "control_player_system",
        ]);
    Ok(game_data)
}
