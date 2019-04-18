extern crate deathframe;

extern crate amethyst;
extern crate json;
extern crate regex;
extern crate ron;
extern crate serde;

mod game;
mod resource_helpers;
mod settings;
mod world_helpers;

mod components;
mod systems;

pub use deathframe::geo;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ColorMask,
    DepthMode,
    DisplayConfig,
    DrawFlat2D,
    Pipeline,
    RenderBundle,
    Stage,
    ALPHA,
    REPLACE,
};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::fps_counter::FPSCounterBundle;
use amethyst::{LogLevelFilter, LoggerConfig};

use deathframe::custom_game_data::prelude::*;

use resource_helpers::*;
use systems::prelude::*;

fn main() -> amethyst::Result<()> {
    start_logger();

    let game_data = build_game_data()?;

    let mut game: amethyst::CoreApplication<CustomGameData<DisplayConfig>> =
        Application::new("./", game::Startup::new(), game_data)?;
    game.run();

    Ok(())
}

fn start_logger() {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });
}

fn build_game_data<'a, 'b>(
) -> amethyst::Result<CustomGameDataBuilder<'a, 'b, DisplayConfig>> {
    // Display config
    let display_config = DisplayConfig::load(&resource("config/display.ron"));

    // Pipeline
    let pipeline = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.2, 0.2, 0.2, 1.0], 10.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                // NOTE: I have no idea what this `DepthMode` does, as it isn't documented,
                //       but sprite ordering via their z positions only works with this `DepthMode` variant.
                Some(DepthMode::LessEqualWrite),
            ))
            .with_pass(DrawUi::new()),
    );

    // Bundles
    let transform_bundle = TransformBundle::new();
    let render_bundle =
        RenderBundle::new(pipeline, Some(display_config.clone()))
            .with_sprite_sheet_processor()
            .with_sprite_visibility_sorting(&["transform_system"]);
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(&resource("config/bindings.ron"))?;
    let ui_bundle = UiBundle::<String, String>::new();
    let fps_bundle = FPSCounterBundle;

    // Create GameDataBuilder
    let game_data = CustomGameData::<DisplayConfig>::new()
        .dispatcher("startup")?
        .dispatcher("ingame")?
        .dispatcher("paused")?
        .custom(display_config)
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(render_bundle)?
        .with_core_bundle(input_bundle)?
        .with_core_bundle(ui_bundle)?
        .with_core_bundle(fps_bundle)?
        .with_core(ScaleSpritesSystem, "scale_sprites_system", &[])?
        .with_core(DebugSystem::default(), "debug_system", &[])?
        .with("ingame", ControlPlayerSystem, "control_player_system", &[])?
        .with("ingame", GravitySystem, "gravity_system", &[])?
        .with(
            "ingame",
            LimitVelocitiesSystem,
            "limit_velocities_system",
            &["control_player_system", "gravity_system"],
        )?
        .with("ingame", MoveEntitiesSystem, "move_entities_system", &[
            "control_player_system",
            "gravity_system",
            "limit_velocities_system",
        ])?
        .with("ingame", CameraSystem, "camera_system", &[
            "move_entities_system",
        ])?
        .with("ingame", ParallaxSystem, "parallax_system", &[
            "move_entities_system",
            "camera_system",
        ])?
        .with("ingame", CollisionSystem, "collision_system", &[
            "move_entities_system",
        ])?
        .with(
            "ingame",
            DecreaseVelocitiesSystem,
            "decrease_velocities_system",
            &[
                "control_player_system",
                "gravity_system",
                "limit_velocities_system",
                "move_entities_system",
            ],
        )?;
    Ok(game_data)
}
