use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::assets::{Completion, Loader, AssetStorage, ProgressCounter};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{
    Event, VirtualKeyCode, PosTex, Material, MaterialDefaults, Mesh, MeshHandle, Texture, Camera,
    Projection, TextureMetadata, TextureData
};
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

use crate::{WIDTH, HEIGHT};
use super::main_menu::MainMenu;

#[derive(Default)]
pub struct Loading {
    progress: ProgressCounter,
    entities: Vec<Entity>,
}

impl<'a, 'b> State<GameData<'a, 'b>> for Loading {
    fn on_start(&mut self, state_data: StateData<GameData<'a, 'b>>) {
        info!("loading...");

        let StateData { world, .. } = state_data;

        initialise_camera(world);
        initialise_ui(world, &mut self.progress);
        initialise_progress_bar(world);

        // TODO: Delete entities on stop.
    }

    fn handle_event(&mut self, _: StateData<GameData<'a, 'b>>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            return Trans::Quit
        }

        Trans::None
    }

    fn update(&mut self, state_data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        state_data.data.update(&state_data.world);

        info!("Loading {:?}/{:?}", self.progress.num_finished(), self.progress.num_assets());

        match self.progress.complete() {
            Completion::Failed => {
                error!("Failed loading assets: {:?}", self.progress.errors());
                Trans::Quit
            }
            Completion::Complete => {
                info!("Assets loaded, swapping state");
                Trans::Switch(Box::new(MainMenu::default()))
            }
            Completion::Loading => Trans::None,
        }
    }
}

fn initialise_ui(world: &mut World, progress: &mut ProgressCounter) -> Vec<Entity> {
    let mut entities = Vec::<Entity>::new();

    // FPS text.
    let font = world.read_resource::<Loader>().load(
        "fonts/Inconsolata-Regular.ttf",
        TtfFormat,
        (),
        &mut *progress,
        &world.read_resource(),
        );

    let entity = world
        .create_entity()
        .with(UiTransform::new(
                "fps_text".to_string(),
                Anchor::TopLeft,
                51.,
                26.,
                0.,
                100.,
                50.,
                0,
                ))
        .with(UiText::new(
                font.clone(),
                "N/A".to_string(),
                [1.0, 1.0, 1.0, 1.0],
                12.,
                ))
        .build();
    entities.push(entity);

    // Loading text.
    let font = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        (),
        &mut *progress,
        &world.read_resource(),
    );

    let entity = world
        .create_entity()
        .with(UiTransform::new(
                "loading_text".to_string(),
                Anchor::Middle,
                0.,
                0.,
                0.,
                200.,
                50.,
                0,
                ))
        .with(UiText::new(
                font.clone(),
                "Loading...".to_string(),
                [1.0, 1.0, 1.0, 1.0],
                25.,
                ))
        .build();
    entities.push(entity);

    entities
}

fn initialise_progress_bar(world: &mut World) -> Entity {
    let (mesh, material) = {
        let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();
        let tex_storage = world.read_resource::<AssetStorage<Texture>>();
        let mat_defaults = world.read_resource::<MaterialDefaults>();
        let loader = world.read_resource::<Loader>();

        let meta = TextureMetadata {
            sampler: None,
            mip_levels: Some(1),
            size: Some((1, 1)),
            dynamic: false,
            format: None,
            channel: None,
        };
        let texture_data = TextureData::Rgba([1.0, 1.0, 1.0, 1.0], meta);

        let white = loader.load_from_data(texture_data, (), &tex_storage);
        let material = Material {
            albedo: white,
            ..mat_defaults.0.clone()
        };

        let mesh: MeshHandle = loader.load_from_data(
            QUAD.to_vec().into(),
            (),
            &mesh_storage);

        (mesh, material)
    };

    let mut transform = Transform::default();
    transform.translation = Vector3::new(WIDTH/2., HEIGHT/2.-20., 0.);
    transform.scale = Vector3::new(WIDTH/2.-20., 10., 1.);

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(GlobalTransform::default())
        .with(transform)
        .build()
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            WIDTH,
            HEIGHT,
            0.0,
        ))).with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        )).build()
    // world.add_resource(ActiveCamera { entity: cam });
}

const QUAD: [PosTex; 6] = [
    PosTex { position: [-1.0, -1.0, 0.0], tex_coord: [0.0, 0.0] },
    PosTex { position: [1.0, -1.0, 0.0], tex_coord: [1.0, 0.0] },
    PosTex { position: [1.0, 1.0, 0.0], tex_coord: [1.0, 1.0] },
    PosTex { position: [1.0, 1.0, 0.0], tex_coord: [1.0, 1.0] },
    PosTex { position: [-1.0, 1.0, 0.0], tex_coord: [0.0, 1.0] },
    PosTex { position: [-1.0, -1.0, 0.0], tex_coord: [0.0, 0.0] },
];

/// Generate the vertices of a rectangle.
pub fn gen_rectangle_vertices(w: f32, h: f32) -> Vec<PosTex> {
    let data: Vec<PosTex> = vec![
        PosTex {
            position: [-w / 2., -h / 2., 0.],
            tex_coord: [0., 0.],
        },
        PosTex {
            position: [w / 2., -h / 2., 0.],
            tex_coord: [1., 0.],
        },
        PosTex {
            position: [w / 2., h / 2., 0.],
            tex_coord: [1., 1.],
        },
        PosTex {
            position: [w / 2., h / 2., 0.],
            tex_coord: [1., 1.],
        },
        PosTex {
            position: [-w / 2., h / 2., 0.],
            tex_coord: [0., 1.],
        },
        PosTex {
            position: [-w / 2., -h / 2., 0.],
            tex_coord: [0., 0.],
        },
    ];
    data
}
