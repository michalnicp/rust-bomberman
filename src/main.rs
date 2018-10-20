extern crate gfx;
extern crate amethyst;

#[macro_use]
extern crate log;
extern crate env_logger;

use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage,
};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::fps_counter::FPSCounterBundle;
use amethyst::audio::AudioBundle;

mod states;
mod systems;

pub const WIDTH: f32 = 1024.;
pub const HEIGHT: f32 = 768.;

const CLEAR_COLOUR: [f32; 4] = [0., 0., 0., 1.];

fn main() -> amethyst::Result<()> {
    env_logger::init();

    let resources_path = format!("{}/resources", env!("CARGO_MANIFEST_DIR"));

    let display_config_path = format!("{}/display_config.ron", resources_path);
    let display_config = DisplayConfig::load(&display_config_path);

    let pipeline_builder = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target(CLEAR_COLOUR, 1.0)
            .with_pass(DrawFlat::<PosTex>::new())
            .with_pass(DrawUi::new()),
    );

    let game_data_builder = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(FPSCounterBundle::default())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipeline_builder, Some(display_config)))?
        .with(systems::DebugSystem, "debug_system", &[]);

    let mut game = Application::build(resources_path, states::Loading::default())?
        .with_resource(Music::default())
        .build(game_data_builder)?;
    game.run();

    Ok(())
}

use amethyst::audio::SourceHandle;

#[derive(Default)]
pub struct Music {
    source: Option<SourceHandle>,
}

impl Music {
    pub fn new(source: SourceHandle) -> Self {
        Music {
            source: Some(source),
        }
    }

    pub fn next(&self) -> Option<SourceHandle> {
        self.source.as_ref().map(Clone::clone)
    }
}
