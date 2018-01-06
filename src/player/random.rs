use rand::{thread_rng, Rng};

use game::Game;
use player::InputHandler;

pub struct Random {}

impl InputHandler for Random {
    fn update_game(&self, game: &Game) {
        // Don't need to do anything
    }

    fn get_placement(&self, available_places: Vec<String>) -> String {
        thread_rng().choose(&available_places).unwrap().to_string()
    }

    fn get_move(&self, available_moves: Vec<(String, String)>) -> (String, String) {
        thread_rng().choose(&available_moves).unwrap().to_owned()
    }

    fn get_mill(&self, available_mills: Vec<String>) -> String {
        thread_rng().choose(&available_mills).unwrap().to_string()
    }
}
