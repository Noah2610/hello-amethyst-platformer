extern crate deathframe;

extern crate amethyst;
#[macro_use]
extern crate amethyst_editor_sync;
extern crate json;
extern crate regex;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate tap;

mod game;
mod resource_helpers;
mod settings;
mod world_helpers;

mod components;
mod systems;

pub use deathframe::geo;

use amethyst::audio::AudioBundle;
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
use amethyst_editor_sync::SyncEditorBundle;
use tap::*;

use deathframe::custom_game_data::prelude::*;

use components::prelude as comps;
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
            .with_pass(DrawUi::new()), // NOTE: "It's recommended this be your last pass."
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

    // TODO: Temporary.
    use deathframe::handlers::AudioHandles;
    let audio_bundle =
        AudioBundle::new(|audio: &mut AudioHandles| audio.get("music"));

    // amethyst_editor_sync bundle
    use comps::*;
    let editor_bundle = SyncEditorBundle::default()
        // Register the default types from the engine.
        .tap(SyncEditorBundle::sync_default_types)
        // Register the components and resources specified above.
        .tap(|bundle| {
            read_components!(
                bundle,
                Camera,
                CheckCollision,
                Collision,
                DecreaseVelocity,
                Gravity,
                InnerSize,
                MaxVelocity,
                Parallax,
                Push,
                Pushable,
                ScaleOnce,
                Size,
                Solid,
                Velocity,
                JumpRecharge,
                Player,
            )
        })
        .tap(|bundle| sync_resources!(bundle, settings::Settings));

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
        .with_core_bundle(editor_bundle)?
        .with_core_bundle(audio_bundle)?
        .with_core(InputManagerSystem, "input_manager_system", &[
            "input_system",
        ])?
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
        )?
        .with("ingame", AnimationSystem, "animation_system", &[])?;
    Ok(game_data)
}
