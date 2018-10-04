use amethyst::assets::{AssetStorage, Loader};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{
    Event, VirtualKeyCode, PosTex, Mesh, MeshHandle, Texture, Material, MaterialDefaults, TextureHandle
};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ui::{UiCreator};
use amethyst::{GameData, SimpleState, StateData, Trans};
use amethyst::ecs::prelude::*;
use amethyst::core::cgmath::Vector3;
use amethyst::prelude::*;

#[derive(Default)]
pub struct Loading {}

impl<'a, 'b> SimpleState<'a, 'b> for Loading {
    fn on_start(&mut self, data: StateData<GameData>) {
        println!("Loading...");

        let world = data.world;

        // Progress bar.
        // let (texture, mesh) = {
        //     let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();
        //     let tex_storage = world.read_resource::<AssetStorage<Texture>>();
        //     let loader = world.read_resource::<Loader>();

        //     let texture = loader.load_from_data([1.0, 1.0, 1.0, 1.0].into(), (), &tex_storage);

        //     let mesh: MeshHandle = loader.load_from_data(
        //         QUAD.to_vec().into(),
        //         (),
        //         &mesh_storage);

        //     (texture, mesh)
        // };

        // world.register::<TextureHandle>();

        // let mut transform = Transform::default();
        // transform.scale = Vector3::new(20., 20., 20.);
        // world
        //     .create_entity()
        //     .with(mesh)
        //     .with(texture)
        //     .with(transform)
        //     .build();

        world.exec(|mut creator: UiCreator| {
            creator.create("ui/fps.ron", ());
            creator.create("ui/loading.ron", ());
        });
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: StateEvent<()>,
    ) -> SimpleTrans<'a, 'b> {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, state_data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
        state_data.data.update(&state_data.world);

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
