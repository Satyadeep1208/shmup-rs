
pub mod loopholdertrait;
mod game;

use std::collections::HashMap;

use sdl2::render::Texture;

use crate::state::game::Game;


pub enum State<'a> {
    GameState(Game<'a>),
}

pub fn get_state_map<'a> (texture_map: &'a HashMap<String, Texture>) -> HashMap<String, State<'a>> {

    let mut state_map: HashMap<String, State> = HashMap::new();

    let game = Game::new(&texture_map).unwrap();

    state_map.insert(
        "game".to_string(),
        State::GameState(game),
    );

    state_map

}
