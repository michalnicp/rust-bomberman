use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{Event, VirtualKeyCode};
use amethyst::ui::UiCreator;
use amethyst::{GameData, State, StateData, Trans};

#[derive(Default)]
pub struct Loading {}

impl<'a, 'b> State<GameData<'a, 'b>> for Loading {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        println!("Loading...");

        data.world.exec(|mut creator: UiCreator| {
            creator.create("ui/fps.ron", ());
            creator.create("ui/loading.ron", ());
        });
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            return Trans::Quit;
        }

        Trans::None
    }

    fn update(&mut self, state_data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        state_data.data.update(&state_data.world);

        Trans::None
    }
}
