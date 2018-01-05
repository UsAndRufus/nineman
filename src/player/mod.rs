mod input_handler;
mod human_input;
mod random_input;

use std::cell::Cell;
use std::fmt;

pub use self::input_handler::InputHandler;
pub use self::human_input::HumanInput;
pub use self::random_input::RandomInput;

const WIN_SCORE: i8 = 7;
const STARTING_PIECES: i8 = 9;

pub struct Player {
    pub name: String,
    pub id: i8,
    input_handler: Box<InputHandler>,
    score: Cell<i8>,
    pieces_left_to_place: Cell<i8>,
}

impl Player {
    pub fn new(name: String, id: i8, input_handler: Box<InputHandler>) -> Self {
        Player { name: name, id: id, input_handler: input_handler, score: Cell::new(0),
                 pieces_left_to_place: Cell::new(STARTING_PIECES) }
    }

    pub fn mill(&self, available_mills: Vec<String>) -> String {
        self.input_handler.get_mill(available_mills)
    }

    pub fn is_placement(&self) -> bool {
        self.pieces_left_to_place.get() > 0
    }

    pub fn place_piece(&self) {
        if self.is_placement() {
            self.pieces_left_to_place.set(self.pieces_left_to_place.get() - 1);
        }
    }

    pub fn increment_score(&self) {
        self.score.set(self.pieces_left_to_place.get() + 1);
    }

    pub fn has_won(&self) -> bool {
        self.score.get() >= WIN_SCORE
    }

    pub fn score(&self) -> i8 {
        self.score.get()
    }

    pub fn get_move(&self, available_moves: Vec<(String, String)>) -> (String, String) {
        assert!(!self.is_placement());
        self.input_handler.get_move(available_moves)
    }

    pub fn get_placement(&self, available_places: Vec<String>) -> String {
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
