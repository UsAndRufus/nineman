use rand::{thread_rng, Rng};

use game::GameState;
use player::InputHandler;

pub struct Random {}

impl InputHandler for Random {
    fn update_game_state(&mut self, _game_state: GameState) {
        // Don't need to do anything
    }

    fn get_placement(&mut self, available_places: Vec<String>) -> String {
        thread_rng().choose(&available_places).unwrap().to_string()
    }

    fn get_move(&mut self, available_moves: Vec<(String, String)>) -> (String, String) {
        thread_rng().choose(&available_moves).unwrap().to_owned()
    }

    fn get_mill(&mut self, available_mills: Vec<String>) -> String {
        thread_rng().choose(&available_mills).unwrap().to_string()
    }

    fn to_string(&self) -> String {
        "Random InputHandler".to_string()
    }
}
