use amethyst::core::{Time, Transform, GlobalTransform};

use amethyst::ecs::prelude::{Read, ReadExpect, System, ReadStorage, WriteStorage, Join};
use amethyst::ui::{UiFinder, UiText};
use amethyst::utils::fps_counter::FPSCounter;
use amethyst::renderer::{ScreenDimensions, Camera, ActiveCamera, Texture};
use amethyst::assets::{AssetStorage, Loader};

pub struct DebugSystem;

impl<'a> System<'a> for DebugSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, UiText>,
        Read<'a, FPSCounter>,
        UiFinder<'a>,
        ReadExpect<'a, ScreenDimensions>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, GlobalTransform>,
        Read<'a, AssetStorage<Texture>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, mut ui_text, fps_counter, finder, screen_dimensions, cameras, transforms, texture_storage) = data;

        // Update the fps every 20 frames, the default sample size for fps counter.
        if time.frame_number() % 20 == 0 {
            if let Some(fps_entity) = finder.find("fps_text") {
                if let Some(fps_text) = ui_text.get_mut(fps_entity) {
                    let fps = fps_counter.sampled_fps();
                    debug!("fps: {:.*}", 2, fps);
                    fps_text.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }

        debug!("screen dimensions: ({:?}, {:?})", screen_dimensions.width(), screen_dimensions.height());
    }
}
