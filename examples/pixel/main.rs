extern crate amethyst;

use amethyst::assets::{Loader, AssetStorage};
use amethyst::core::cgmath::{Array, Deg, Euler, Quaternion, Rotation3, Vector3, Matrix4};
use amethyst::core::transform::{TransformBundle, GlobalTransform, Transform};
use amethyst::prelude::*;
use amethyst::renderer::*;
use amethyst::input::{is_close_requested, is_key_down};

const WIDTH: f32 = 1024.;
const HEIGHT: f32 = 768.;

const CLEAR_COLOUR: [f32; 4] = [0., 0., 0., 1.];

const QUAD: [PosTex; 6] = [
    PosTex { position: [-1.0, -1.0, 0.0], tex_coord: [0.0, 0.0] },
    PosTex { position: [1.0, -1.0, 0.0], tex_coord: [1.0, 0.0] },
    PosTex { position: [1.0, 1.0, 0.0], tex_coord: [1.0, 1.0] },
    PosTex { position: [1.0, 1.0, 0.0], tex_coord: [1.0, 1.0] },
    PosTex { position: [-1.0, 1.0, 0.0], tex_coord: [0.0, 1.0] },
    PosTex { position: [-1.0, -1.0, 0.0], tex_coord: [0.0, 0.0] },
];

struct Example;

impl<'a, 'b> State<GameData<'a, 'b>> for Example {
    fn on_start(&mut self, state_data: StateData<GameData<'a, 'b>>) {
        println!("starting...");

        let StateData { world, .. } = state_data;

        let mesh = {
            let loader = world.read_resource::<Loader>();
            let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();

            let mesh: MeshHandle = loader.load_from_data(
                QUAD.to_vec().into(),
                (),
                &mesh_storage);
            mesh
        };

        let white = {
            let loader = world.read_resource::<Loader>();
            // let texture_storage = world.read_resource::<AssetStorage<Texture>>();

            // let meta = TextureMetadata {
            //     sampler: None,
            //     mip_levels: Some(1),
            //     size: Some((1, 1)),
            //     dynamic: false,
            //     format: None,
            //     channel: None,
            // };
            // let texture_data = TextureData::Rgba([1.0, 1.0, 1.0, 1.0], meta);
            // let white = loader.load_from_data(texture_data, (), &tex_storage);

            let white = loader.load(
                "red.png",
                PngFormat,
                Default::default(),
                (),
                &world.read_resource(),
            );

            white
        };

        let mut transform = Transform::default();
        transform.translation = Vector3::new(WIDTH/2., HEIGHT/2.-20., 0.);

        transform.scale = Vector3::new(WIDTH/2.-20., 10., 1.);

        world
            .create_entity()
            .with(mesh)
            .with(white)
            .with(GlobalTransform::default())
            .with(transform)
            .build();
    }

    fn handle_event(&mut self, _: StateData<GameData<'a, 'b>>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            return Trans::Quit
        }

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources_path = format!("{}/resources", env!("CARGO_MANIFEST_DIR"));
    println!("resources path: {}", resources_path);

    let display_config_path = format!("{}/display_config.ron", resources_path);
    let display_config = DisplayConfig::load(&display_config_path);

    let pipeline_builder = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target(CLEAR_COLOUR, 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data_builder = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipeline_builder, Some(display_config)))?;

    let mut game = Application::build(resources_path, Example)?
        .build(game_data_builder)?;
    game.run();

    Ok(())
}
