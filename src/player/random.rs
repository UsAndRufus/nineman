use rand::{thread_rng, Rng};

use game::GameState;
use game::Ply;
use player::InputHandler;

pub struct Random {
    pub player_id: i8,
}

impl InputHandler for Random {
    fn give_new_game_state(&mut self, _game_state: GameState) {
        // Don't need to do anything
    }

    fn get_placement(&mut self, available_places: Vec<Ply>) -> Ply {

        match thread_rng().choose(&available_places) {
            Some(ply) => ply.to_owned(),
            None => panic!("In placement phase and no places to choose from")
        }
    }

    fn get_move(&mut self, available_moves: Vec<Ply>) -> Ply {
        match thread_rng().choose(&available_moves) {
            Some(ply) => ply.to_owned(),
            None => panic!("In move phase and no moves to choose from")
        }
    }

    fn get_mill(&mut self, available_mills: Vec<Ply>) -> Ply {
        match thread_rng().choose(&available_mills) {
            Some(ply) => ply.to_owned(),
            None => panic!("In mill phase and no mills to choose from")
        }
    }

    fn to_string(&self) -> String {
        "Random InputHandler".to_string()
    }

    fn set_player_id(&mut self, player_id: i8) {
        self.player_id = player_id;
    }
}
