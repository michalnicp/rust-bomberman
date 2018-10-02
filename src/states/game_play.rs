use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{Event, VirtualKeyCode};
use amethyst::{State, StateData, Trans};

pub struct GamePlayState;

impl<'a, 'b> State<GameData<'a, 'b>> for GamePlayState {
    fn on_start(&mut self, _: StateData<GameData>) {
        info!("Starting game.");
    }

    fn on_stop(&mut self, _: StateData<GameData>) {
        info!("Exiting game.");
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, state_data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        state_data.data.update(&state_data.world);
        Trans::None
    }
}
