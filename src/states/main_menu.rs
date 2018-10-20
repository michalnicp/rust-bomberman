use gfx::texture::SamplerInfo;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::assets::{Completion, Loader, AssetStorage, ProgressCounter};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{
    Event, VirtualKeyCode, PosTex, Material, MaterialDefaults, Mesh, MeshHandle, Texture, Camera,
    Projection, TextureMetadata, TextureData, PngFormat
};
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

use crate::{WIDTH, HEIGHT};

#[derive(Default)]
pub struct MainMenu;

impl<'a, 'b> State<GameData<'a, 'b>> for MainMenu {
    fn on_start(&mut self, state_data: StateData<GameData>) {
        info!("Main menu started");

        // Create background
        initialise_background(state_data.world);
    }

    fn update(&mut self, state_data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        let StateData { world, data } = state_data;

        data.update(&world);

        Trans::None
    }
}

fn initialise_background(world: &mut World) -> Entity {
    let (mesh, background) = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();
        let mat_defaults = world.read_resource::<MaterialDefaults>();
        let loader = world.read_resource::<Loader>();

        let meta = TextureMetadata {
            sampler: SamplerInfo::new(FilterMethod::Trilinear, WrapMode::Clamp),
            mip_levels: Some(1),
            size: Some((1, 1)),
            dynamic: false,
            format: None,
            channel: None,
        };

        let background = Material {
            albedo: loader.load(
                "textures/logo.png",
                PngFormat,
                meta,
                (),
                &texture_storage,
            ),
            ..mat_defaults.0.clone()
        };
        // loader.load_from_data([0., 0., 1., 0.].into(), (), &world.read_resource())
        // loader.load_from_data([0.6, 0.4, 0.2, 1.].into(), (), &world.read_resource())

        let mesh = loader.load_from_data(
            QUAD.to_vec().into(),
            (),
            &mesh_storage,
        );

        (mesh, background)
    };

    let mut transform = Transform::default();
    transform.translation = Vector3::new(WIDTH/2., HEIGHT/2.-50., 0.);
    transform.scale = Vector3::new(100., 100., 1.);

    world
        .create_entity()
        .with(mesh)
        .with(background)
        .with(transform)
        .with(GlobalTransform::default())
        .build()
}

const QUAD: [PosTex; 6] = [
    PosTex { position: [-1.0, -1.0, 0.0], tex_coord: [0.0, 0.0] },
    PosTex { position: [1.0, -1.0, 0.0], tex_coord: [1.0, 0.0] },
    PosTex { position: [1.0, 1.0, 0.0], tex_coord: [1.0, 1.0] },
    PosTex { position: [1.0, 1.0, 0.0], tex_coord: [1.0, 1.0] },
    PosTex { position: [-1.0, 1.0, 0.0], tex_coord: [0.0, 1.0] },
    PosTex { position: [-1.0, -1.0, 0.0], tex_coord: [0.0, 0.0] },
];
