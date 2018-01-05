use rand::{thread_rng, Rng};

use player::InputHandler;

pub struct RandomInput {}

impl InputHandler for RandomInput {
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
