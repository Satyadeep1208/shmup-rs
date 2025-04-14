
mod sdlsetup;
mod state;

use std::time::Duration;

use sdlsetup::setup_and_get_refs;
use state::{State, LoopHolder, get_state_map};


pub fn main() -> Result<(), String> {

    let mut sdl_refs = setup_and_get_refs().unwrap();

    let mut state_map = get_state_map(&sdl_refs.texture_creator);

    let State::Game(state) = state_map.get_mut("game").unwrap();

    let quit = String::from("quit");

    loop {

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        if let Err(text) = state.apploop(&mut sdl_refs.event_pump, &mut sdl_refs.canvas) {

            if text == quit {
                break;
            }

        }

    }

    Ok(())

}
