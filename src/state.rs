
pub mod loopholdertrait;
mod gamestate;

use std::collections::HashMap;

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::state::gamestate::GameState;


pub enum State<'a> {
    Game(GameState<'a>),
}

pub fn get_state_map<'a> (texture_creator: &'a TextureCreator<WindowContext>) -> HashMap<String, State<'a>> {

    let mut state_map: HashMap<String, State> = HashMap::new();

    let game_state = GameState::new(&texture_creator).unwrap();

    state_map.insert(
        String::from("game"),
        State::Game(game_state),
    );

    state_map

}
