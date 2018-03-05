mod input_handler;
mod human;
mod random;
mod player_state;

use std::fmt;

pub use self::input_handler::InputHandler;
pub use self::human::Human;
pub use self::random::Random;
pub use self::player_state::PlayerState;

use game::GameState;

pub struct Player {
    pub name: String,
    pub id: i8,
    input_handler: Box<InputHandler>,
}

impl Player {
    pub fn new(name: String, id: i8, input_handler: Box<InputHandler>) -> Self {
        Player { name: name, id: id, input_handler: input_handler }
    }

    pub fn mill(&mut self, available_mills: Vec<String>) -> String {
        self.input_handler.get_mill(available_mills)
    }

    pub fn give_new_game_state(&mut self, game_state: GameState) {
        self.input_handler.give_new_game_state(game_state);
    }

    pub fn set_input_handler_player_id(&mut self, player_id: i8) {
        self.input_handler.set_player_id(player_id);
    }

    // NB used to be assert!(!self.is_placement()) before PlayerState
    pub fn get_move(&mut self, available_moves: Vec<(String, String)>) -> (String, String) {
        self.input_handler.get_move(available_moves)
    }

    // NB used to be assert!(!self.is_placement()) before PlayerState
    pub fn get_placement(&mut self, available_places: Vec<String>) -> String {
        self.input_handler.get_placement(available_places)
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        return write!(f, "Player {} ({})", self.id, self.name);

    }
}
