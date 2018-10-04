use amethyst::core::Time;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::core::cgmath::Vector3;
use amethyst::assets::{Loader, AssetStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{Event, VirtualKeyCode, PosTex, Material, MaterialDefaults, Mesh, MeshHandle, Texture, Shape};
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::utils::fps_counter::FPSCounter;

#[derive(Default)]
pub struct Loading {
    fps: Option<Entity>,
}

impl<'a, 'b> State<GameData<'a, 'b>> for Loading {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        println!("Loading...");

        let StateData { world, .. } = data;

        {
            // FPS text.
            let font = world.read_resource::<Loader>().load(
                "fonts/Inconsolata-Regular.ttf",
                TtfFormat,
                (),
                (),
                &world.read_resource(),
            );

            let fps = world
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
            self.fps = Some(fps);

            // Loading text.
            let font = world.read_resource::<Loader>().load(
                "fonts/square.ttf",
                TtfFormat,
                (),
                (),
                &world.read_resource(),
            );
            world
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
        }

        // Progress bar.
        let (mesh, material) = {
            let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();
            let tex_storage = world.read_resource::<AssetStorage<Texture>>();
            let mat_defaults = world.read_resource::<MaterialDefaults>();
            let loader = world.read_resource::<Loader>();

            let white = loader.load_from_data([1.0, 1.0, 1.0, 1.0].into(), (), &tex_storage);
            let material = Material {
                albedo: white,
                ..mat_defaults.0.clone()
            };

            // let mesh: MeshHandle = loader.load_from_data(
            //     QUAD.to_vec().into(),
            //     (),
            //     &mesh_storage);
            let mesh: MeshHandle = loader.load_from_data(
                Shape::Sphere(32, 32).generate::<Vec<PosTex>>(None),
                (),
                &mesh_storage,
            );

            (mesh, material)
        };

        let mut transform = Transform::default();
        transform.translation = Vector3::new(5., 0., -2.0);
        transform.scale = Vector3::new(5., 5., 5.);

        world
            .create_entity()
            .with(mesh)
            .with(material)
            .with(transform)
            .with(GlobalTransform::default())
            .build();
    }

    fn handle_event(
        &mut self,
        _: StateData<GameData<'a, 'b>>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            return Trans::Quit
        }

        Trans::None
    }

    fn update(&mut self, state_data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>> {
        let StateData { world, data } = state_data;

        if let Some(fps_entity) = self.fps {
            let frame_number = world.read_resource::<Time>().frame_number();
            let fps = world.read_resource::<FPSCounter>().sampled_fps();

            if frame_number % 20 == 0 {
                if let Some(fps_text) = world.write_storage::<UiText>().get_mut(fps_entity) {
                    fps_text.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }

        data.update(world);
        Trans::None
    }
}

const QUAD: [PosTex; 6] = [
    PosTex {
        position: [-0.5, 0.5, 0.],
        tex_coord: [0., 0.],
    },
    PosTex {
        position: [0.5, 0.5, 0.],
        tex_coord: [1., 0.],
    },
    PosTex {
        position: [-0.5, -0.5, 0.],
        tex_coord: [0., 1.],
    },
    PosTex {
        position: [0.5, -0.5, 0.],
        tex_coord: [1., 1.],
    },
    PosTex {
        position: [-0.5, -0.5, 0.],
        tex_coord: [0., 1.],
    },
    PosTex {
        position: [0.5, 0.5, 0.],
        tex_coord: [1., 0.],
    },
];
