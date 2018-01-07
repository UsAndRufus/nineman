mod input_handler;
mod human;
mod random;

use std::cell::Cell;
use std::fmt;

pub use self::input_handler::InputHandler;
pub use self::human::Human;
pub use self::random::Random;
use game::GameState;

pub const WIN_SCORE: i8 = 7;
pub const STARTING_SCORE: i8 = 0;
pub const STARTING_PIECES: i8 = 9;

pub struct Player {
    pub name: String,
    pub id: i8,
    input_handler: Box<InputHandler>,
    score: Cell<i8>,
    pieces_left_to_place: Cell<i8>,
}

impl Player {
    pub fn new(name: String, id: i8, input_handler: Box<InputHandler>) -> Self {
        Player { name: name, id: id, input_handler: input_handler,
                score: Cell::new(STARTING_SCORE),
                pieces_left_to_place: Cell::new(STARTING_PIECES) }
    }

    pub fn mill(&mut self, available_mills: Vec<String>) -> String {
        self.input_handler.get_mill(available_mills)
    }

    pub fn update_game_state(&mut self, game_state: GameState) {
        self.input_handler.update_game_state(game_state);
    }

    pub fn set_input_handler_player_id(&mut self, player_id: i8) {
        self.input_handler.set_player_id(player_id);
    }

    pub fn is_placement(&self) -> bool {
        self.pieces_left_to_place.get() > 0
    }

    pub fn place_piece(&self) {
        if self.is_placement() {
            self.pieces_left_to_place.set(self.pieces_left_to_place.get() - 1);
        } else {
            panic!("Tried to place piece when it was not placement!");
        }
    }

    pub fn increment_score(&self) {
        self.score.set(self.score.get() + 1);
    }

    pub fn has_won(&self, other_player_available_moves: Vec<(String,String)>, other_player_placement: bool) -> bool {
        if other_player_placement {
            self.score.get() >= WIN_SCORE
        } else {
            other_player_available_moves.is_empty()
        }
    }

    pub fn score(&self) -> i8 {
        self.score.get()
    }

    pub fn get_move(&mut self, available_moves: Vec<(String, String)>) -> (String, String) {
        assert!(!self.is_placement());
        self.input_handler.get_move(available_moves)
    }

    pub fn get_placement(&mut self, available_places: Vec<String>) -> String {
        assert!(self.is_placement());
        self.input_handler.get_placement(available_places)
    }

    pub fn get_pieces_left_to_place(&self) -> i8 {
        self.pieces_left_to_place.get()
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        return write!(f, "Player {}: (name: {}, score: {}, pieces_left_to_place: {})",
                        self.id, self.name, self.score.get(), self.pieces_left_to_place.get());

    }
}
