use std::cell::Cell;
use std::fmt;

pub const WIN_SCORE: i8 = 7;
pub const STARTING_SCORE: i8 = 0;
pub const STARTING_PIECES: i8 = 9;

// TODO: get rid of cells and just make it return new playerstates after updates
#[derive(Clone, Eq, PartialEq)]
pub struct PlayerState {
    score: Cell<i8>,
    pieces_left_to_place: Cell<i8>,
}

impl PlayerState {
    pub fn at_beginning() -> Self {
        PlayerState {
            score: Cell::new(STARTING_SCORE),
            pieces_left_to_place: Cell::new(STARTING_PIECES),
        }
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
        let current_score = self.score.get();
        self.score.set(current_score + 1);
        println!("Player now has {} points", self.score.get());
    }

    pub fn has_won(&self, other_player_available_moves: Vec<(String,String)>,
                    other_player_placement: bool) -> bool {

        self.score.get() >= WIN_SCORE ||
        (!other_player_placement && other_player_available_moves.is_empty())
    }

    pub fn score(&self) -> i8 {
        self.score.get()
    }

    pub fn pieces_left_to_place(&self) -> i8 {
        self.pieces_left_to_place.get()
    }
}

impl fmt::Debug for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        return write!(f, "PlayerState (score: {}, pieces_left_to_place: {})",
                        self.score.get(), self.pieces_left_to_place.get());

    }
}
